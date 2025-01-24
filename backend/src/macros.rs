#[macro_export]
macro_rules! config {
    () => {
        $crate::config::conf.lock()
    };
}
