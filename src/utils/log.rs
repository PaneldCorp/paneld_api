pub enum Level {
    Info, 
    Success,
    Error
}

pub fn log(level: Level, msg: &str) {
    match level {
        Level::Info => println!("\x1b[1;32m[INFO]\x1b[0m {msg}"),
        Level::Success => println!("\x1b[1;34m[SUCCESS]\x1b[0m {msg}"),
        Level::Error => eprintln!("\x1b[1;31m[ERROR]\x1b[0m {msg}")
    }
}
