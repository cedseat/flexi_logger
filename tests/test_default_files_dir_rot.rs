extern crate flexi_logger;
#[macro_use]
extern crate log;

extern crate glob;

use flexi_logger::{default_format, Logger};
use glob::glob;
use std::ops::{Add};
use std::path::PathBuf;
use glob::GlobError;
use std::vec::Vec;

#[test]
fn test_default_files_dir_rot() {
    Logger::with_str("info")
        .format(default_format)
        .log_to_file()
        .directory("log_files")
        .rotate_over_size(2000)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");
    trace!("This is a trace message - you must not see it!");
}

#[test]
fn test_default_files_dir_rot_max_backup() {
    Logger::with_str("info")
        .format(default_format)
        .log_to_file()
        .directory("log_files_max_backup")
        .rotate_over_size(100)
        .max_backup(2)
        .append()
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let fn_pattern = String::with_capacity(180)
        .add("log_files_max_backup/test_default_files_dir_rot-*")
        .add("_r*")
        .add(".")
        .add("log");

    glob(&fn_pattern).map(|globresults| assert_eq!(globresults.count(), 2));

    error!("This is an error message 1");
    warn!("This is a warning 1");
    error!("This is an error message 2");
    warn!("This is a warning 2");
    error!("This is an error message 3");
    warn!("This is a warnn 3");
}
