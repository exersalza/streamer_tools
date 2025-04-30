use anyhow::bail;
use chrono::Utc;
use chrono_tz::Europe::Berlin;
use colored::Colorize;
use core::{fmt, panic};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::fmt::Display;
use std::io::Write;
use std::{fs, path};

//use tracing::field::{AsField, Field, Visit};
//use tracing_subscriber::Layer;

lazy_static! {
    pub static ref log: Mutex<Log> = Mutex::new(Log::new(
        std::env::var_os("LC_LOG_LVL").map(|i| i.to_str().unwrap().to_owned()),
        None,
        None
    ));
}

const MAX_BUF_CAP: usize = 4096;

#[derive(Clone, Debug, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

trait LogLevelT {
    fn into(&self) -> LogLevel;
}

pub struct Log {
    buf: LocalBuffer,
    scope: Vec<LogLevel>,
    log_path: String,
}

pub struct LogItem {
    msg: String,
    level: LogLevel,
    timestamp_format: String,
    target: String,
}

#[derive(Clone)]
pub struct LogBuilder {
    msg: String,
    level: LogLevel,
    timestamp_format: String,
    file: Option<String>,
    line: Option<u32>,
    col: Option<u32>,
}

struct LocalBuffer {
    buf: Vec<u8>,
    cap: usize,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

impl LocalBuffer {
    fn new(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
            cap,
        }
    }

    /// Writes to the buffer
    fn write(&mut self, data: &[u8]) -> usize {
        let write_size = std::cmp::min(data.len(), self.cap - self.buf.len());

        self.buf.extend_from_slice(&data[..write_size]);
        write_size
    }

    /// Read from the buffer
    fn read(&self, buf: &mut [u8]) -> usize {
        let read_size = std::cmp::min(buf.len(), self.buf.len());

        buf[..read_size].copy_from_slice(&self.buf[..read_size]);
        read_size
    }

    /// Read from the buffer and drain the read content
    fn read_and_drain(&mut self, buf: &mut [u8]) -> usize {
        let read_size = self.read(buf);
        self.buf.drain(..read_size);
        read_size
    }

    /// Clears the buffer
    #[allow(unused)]
    fn clear(&mut self) {
        self.buf.clear()
    }

    fn get_buf_length(&self) -> usize {
        self.buf.len()
    }

    fn get_cap(&self) -> usize {
        self.cap
    }
}

impl LogLevelT for str {
    fn into(&self) -> LogLevel {
        match self.to_lowercase().as_str() {
            "error" => LogLevel::Error,
            "debug" => LogLevel::Debug,
            "warn" => LogLevel::Warn,
            &_ => LogLevel::Info,
        }
    }
}

impl LogLevel {
    fn flatten() -> Vec<LogLevel> {
        vec![
            LogLevel::Info,
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Debug,
        ]
    }
}

impl Log {
    /// Creates a new Logging Instance
    ///
    /// # Parameters
    /// - scope -> The scope for the logging, note here you have to define every scope you want.
    ///     Ex: debug,error <- this will only print log messages that triggered for the debug
    ///     and error macros
    /// - buffer_cap -> the max buffer_cap, it will get filled with the messages and then send to
    ///     the file if its going to overflow
    /// - log_path -> the destination of the log file
    ///
    /// # Returns
    ///
    pub fn new(scope: Option<String>, buffer_cap: Option<usize>, log_path: Option<String>) -> Self {
        let mut scope = scope;
        if scope.is_none() {
            scope = Some("all".to_owned());
        }

        let t_scope = Self::parse_scope(scope.unwrap());
        let buf = LocalBuffer::new(buffer_cap.unwrap_or(MAX_BUF_CAP));

        let path = if let Some(p) = log_path {
            p
        } else {
            r"C:\tmp".to_string()
        };

        Self {
            buf,
            scope: t_scope,
            log_path: path,
        }
    }

    fn get_new_log_location(&self) -> String {
        let now = chrono::Utc::now();
        format!("{}log_{}.log", self.log_path, now.date_naive())
    }

    fn parse_scope(to_parse: String) -> Vec<LogLevel> {
        if &to_parse.to_lowercase() == "all" {
            LogLevel::flatten()
        } else {
            to_parse.split(',').map(str::into).collect()
        }
    }

    pub fn set_scope<T: Display>(&mut self, new_scope: T) {
        self.scope = Self::parse_scope(new_scope.to_string());
    }

    /// Writes directly to the buffer
    pub fn write_to_buf<T: Display>(&mut self, msg: T) -> usize {
        self.buf.write(msg.to_string().as_bytes())
    }

    fn get_file_handle(&self) -> fs::File {
        let backup_path = format!(
            "{}log.log",
            std::env::temp_dir().into_os_string().into_string().unwrap()
        );
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.get_new_log_location())
        {
            Err(e) => {
                println!("Couldn't open log file. {e} using default {backup_path}");
                let location = path::PathBuf::from(&backup_path);
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&location)
                    .unwrap() // this should not fail, if it does, we dont have perm or a temp
                              // directory for some reason
            }
            Ok(o) => o,
        }
    }

    /// flushes everything to the file
    fn flush(&mut self) -> usize {
        return 0; // disable log file temp

        #[allow(unreachable_code)]
        let buf: &mut [u8] = &mut [0; MAX_BUF_CAP];
        let buf_length = self.buf.get_buf_length();
        let mut lines_written = 0;

        self.buf.read_and_drain(buf);
        let thing = String::from_utf8(buf.to_vec()).unwrap();

        match self
            .get_file_handle()
            .write(thing.trim_matches('\0').as_bytes())
        {
            Ok(l_write) => {
                lines_written = l_write;
                self.__internal_log(
                    LogBuilder::new()
                        .level(LogLevel::Debug)
                        .file(module_path!().to_string())
                        .msg(format!(
                            "log written check File_write {{ buf_len: {} lines_written: {} }}",
                            buf_length, lines_written
                        ))
                        .build(),
                    &mut std::io::stdout(),
                )
            }
            Err(e) => self.__internal_log(LogBuilder::new().msg(e).build(), &mut std::io::stdout()),
        };

        lines_written
    }

    fn add_timestamp<T: Display>(pre: T, timestamp_format: &str) -> String {
        let now = Utc::now().with_timezone(&Berlin).format(timestamp_format);

        format!("{} {}", now.to_string().bright_black(), pre)
    }

    /// This function parses everything and prints it after its done
    ///
    ///
    /// # Arguments
    ///
    /// * `msg`
    /// * `level`
    /// * `timestamp_format`
    /// * `target`
    /// * `writer`
    ///
    fn parse_log_message<T, W>(
        &mut self,
        msg: T,
        level: LogLevel,
        timestamp_format: T,
        target: T,
        __internal: bool,
        writer: &mut W,
    ) where
        T: fmt::Display,
        W: std::io::Write,
    {
        if !self.scope.contains(&level) {
            return;
        }

        let ret = match level {
            LogLevel::Warn => format!(" {}", level.to_string().yellow().bold()),
            LogLevel::Info => format!(" {}", level.to_string().green().bold()),
            LogLevel::Error => format!("{}", level.to_string().red().bold()),
            LogLevel::Debug => format!("{}", level.to_string().purple().bold()),
        };

        let temp = if target.to_string().is_empty() {
            format!("{} {msg}", ret.bright_black())
        } else {
            format!("{ret} {} {msg}", target.to_string().bright_black())
        };

        let pre = Self::add_timestamp(temp, timestamp_format.to_string().as_str()) + "\n";

        if !__internal {
            let buf_len = self.buf.get_buf_length();
            if self.buf.get_cap() <= buf_len + pre.len() {
                let _ = self.flush();
            }
            self.write_to_buf(&pre);
        }

        let _ = write!(writer, "{pre}");
    }

    // helper functions
    pub fn with_builder<W: std::io::Write + fmt::Debug>(
        &mut self,
        builder: LogItem,
        write: &mut W,
    ) {
        if !self.scope.contains(&builder.level) {
            return;
        }

        self.parse_log_message(
            builder.msg,
            builder.level,
            builder.timestamp_format,
            builder.target,
            false,
            write,
        )
    }

    fn __internal_log<W: std::io::Write>(&mut self, builder: LogItem, write: &mut W) {
        self.parse_log_message(
            builder.msg,
            builder.level,
            builder.timestamp_format,
            builder.target,
            true,
            write,
        )
    }
}

impl LogBuilder {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
            level: LogLevel::Info,
            timestamp_format: String::new(),
            file: None,
            line: None,
            col: None,
        }
    }

    /// Add the message to the log message
    ///
    /// # Parameters
    /// - msg -> Anything that implements the fmt::Display trait
    ///
    /// # Returns
    /// &mut self
    pub fn msg<T: Display>(&mut self, msg: T) -> &mut Self {
        self.msg = msg.to_string();
        self
    }

    /// Sets the log level
    ///
    /// # Parameters
    /// - level -> the target log level
    ///
    /// # Returns
    /// &mut self
    pub fn level(&mut self, level: LogLevel) -> &mut Self {
        self.level = level;
        self
    }

    /// Set the format for the timestamp at the beginning of the log message
    ///
    /// # Default
    /// `%Y-%m-%dT%H:%M:%S%.6fZ`
    ///
    /// # Parameters
    /// - timestamp_format -> The format used to diplay the timestamp
    ///
    /// # Returns
    /// &mut self
    pub fn timestamp_format<T: Display>(&mut self, timestamp_format: T) -> &mut Self {
        self.timestamp_format = timestamp_format.to_string();
        self
    }

    /// Add the line number of the origin of the log message
    ///
    /// # Note
    /// this has to be used in combination with `col`
    ///
    /// # Parameters
    /// - line -> set the line that is being displayed
    ///
    /// # Returns
    /// &mut self
    pub fn line(&mut self, line: u32) -> &mut Self {
        self.line = Some(line);
        self
    }

    /// Add the column number of the origin
    ///
    /// # Note
    /// this has to be used in combination with `line`
    ///
    /// # Parameters
    /// - col -> set the column that is being displayed
    ///
    /// # Returns
    /// &mut self
    pub fn col(&mut self, col: u32) -> &mut Self {
        self.col = Some(col);
        self
    }

    /// Add the column number of the origin
    ///
    /// # Note
    /// this has to be used in combination with `line`
    ///
    /// # Parameters
    /// - col -> set the column that is being displayed
    ///
    /// # Returns
    /// &mut self
    pub fn file(&mut self, file: String) -> &mut Self {
        self.file = Some(file);
        self
    }

    /// Build with all the flags set before
    ///
    /// # Returns
    /// The built and ready to use LogItem
    ///
    /// # Example
    /// ```
    ///
    /// use lufe_core::logs::{self, LogBuilder};
    /// let inter_log = &*logs::log;
    /// let mut lock = inter_log.lock();
    ///
    /// lock.with_builder(
    ///     LogBuilder::new()
    ///        .msg("hello")
    ///        .level(lufe_core::logs::LogLevel::Info)
    ///        .line(line!())
    ///        .col(column!())
    ///        .file(module_path!().to_string())
    ///        .build(),
    /// );
    /// ```
    pub fn build(&mut self) -> LogItem {
        if self.msg.is_empty() {
            panic!("Can't build Log message without message.");
        }

        if self.timestamp_format.is_empty() {
            self.timestamp_format = "%Y-%m-%dT%H:%M:%S%.6fZ".to_string();
        }

        let mut target = match (self.line, self.col) {
            (None, None) => String::new(),
            (None, Some(_)) => panic!("Can't use column number without line number"),
            (Some(_), None) => panic!("Can't use line number without column number"),
            (Some(line), Some(col)) => format!("{}:{}", line, col),
        };

        target = match &self.file {
            Some(file) => format!("{} {}", file, target),
            None => target,
        };

        LogItem {
            msg: self.msg.clone(),
            level: self.level.clone(),
            timestamp_format: self.timestamp_format.clone(),
            target,
        }
    }
}

impl Default for LogBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// TESTING
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {}
}
