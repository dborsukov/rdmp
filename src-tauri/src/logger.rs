use crate::fs;
use flexi_logger::{FileSpec, Logger, WriteMode};

pub fn setup() -> () {
    let app_dir = fs::get_app_base_dir();
    let logs_dir = app_dir.join("logs");
    Logger::try_with_str("debug")
        .unwrap()
        .log_to_file(FileSpec::default().directory(logs_dir))
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();
}
