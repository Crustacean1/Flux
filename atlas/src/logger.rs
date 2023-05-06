pub mod console_logger;

pub enum LogMsg<'a> {
    Debug(&'a str),
    Info(&'a str),
    Warning(&'a str),
    Error(&'a str),
}

pub trait Logger {
    fn log_info(&self, msg: &str);
    fn log_debug(&self, msg: &str);
    fn log_warning(&self, msg: &str);
    fn log_error(&self, msg: &str);
}
