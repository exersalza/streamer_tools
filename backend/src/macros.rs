#[macro_export]
macro_rules! config {
    () => {
        $crate::config::conf.lock()
    };
}

#[macro_export]
macro_rules! top_level {
    ($level:expr, $writer:expr, $($msg:tt)+) => {{
        // TODO: add check for the existence
        let inter_log = &*$crate::logs::log;
        let mut lock = inter_log.lock();

        lock.with_builder(
            $crate::logs::LogBuilder::new()
                .msg(format!($($msg)*))
                .level($level)
                .line(line!())
                .col(column!())
                .file(module_path!().to_string())
                .build(),
            $writer
        );
    }}
}

#[macro_export]
macro_rules! info {
    ( $writer:expr, $($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Info,  $writer, $($msg)+)
    };

    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Info, &mut std::io::stdout(), $($msg)+)
    };
}

#[macro_export]
macro_rules! error {
    ( $writer:expr, $($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Error,  $writer, $($msg)+)
    };

    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Error, &mut std::io::stdout(), $($msg)+)
    };
}

#[macro_export]
macro_rules! warn {
    ( $writer:expr, $($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Warn, $writer, $($msg)+)
    };

    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Warn, &mut std::io::stdout(), $($msg)+)
    };
}

#[macro_export]
macro_rules! debug {
    ($writer:expr, $($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Debug,  $writer, $($msg)+)
    };

    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Debug, &mut std::io::stdout(), $($msg)+)
    };
}
