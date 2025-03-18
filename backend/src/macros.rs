#[macro_export]
macro_rules! config {
    () => {
        $crate::config::conf.lock()
    };
}

#[macro_export]
macro_rules! top_level {
    ($level:expr, $($msg:tt)+) => {{
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
        );
    }}
}

#[macro_export]
macro_rules! info {
    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Info, $($msg)+)
    };
}

#[macro_export]
macro_rules! error {
    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Error, $($msg)+)
    };
}

#[macro_export]
macro_rules! warn {
    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Warn, $($msg)+)
    };
}

#[macro_export]
macro_rules! debug {
    ($($msg:tt)+) => {
        $crate::top_level!($crate::logs::LogLevel::Debug, $($msg)+)
    };
}
