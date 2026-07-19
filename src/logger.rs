#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("[{}] {}", terminal_log_symbols::INFO_SYMBOL.to_string().bright_blue(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("[{}] {}", terminal_log_symbols::SUCCESS_SYMBOL.to_string().bright_green(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("[{}] {}", terminal_log_symbols::WARNING_SYMBOL.to_string().bright_yellow(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("[{}] {}", terminal_log_symbols::ERROR_SYMBOL.to_string().bright_red(), format!($($arg)*))
    }
}
