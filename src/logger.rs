/// # Info Log
/// Logs a message into the console with the severity "info".
pub fn info<T: AsRef<str>>(text: T) {
    println!("[INFO] {}", text.as_ref());
}

/// # Warn Log
/// Logs a message into the console with the severity "warn".
pub fn warn<T: AsRef<str>>(text: T) {
    println!("[WARNING] {}", text.as_ref());
}

/// # Error Log
/// Logs a message into the console with the severity "error".
pub fn error<T: AsRef<str>>(text: T) {
    println!("[ERROR] {}", text.as_ref());
}

// Make debug only print stuff if we in a debug build.
#[cfg(debug_assertions)]
pub fn debug<T: AsRef<str>>(text: T) {
    println!("[DEBUG] {}", text.as_ref());
}
#[cfg(not(debug_assertions))]
pub fn debug<T: AsRef<str>>(text: T) {}