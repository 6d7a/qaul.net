use std::{
    collections::BTreeMap,
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

use filetime::FileTime;

mod proto_sys {
    include!("../../libqaul/src/rpc/protobuf_generated/rust/qaul.sys.ble.rs");
}

/// initialize and start the ble_module
///
#[tokio::main]
async fn main() {
    // --- initialize logger ---
    // prepare logger path
    // the path of the log file follows libqaul's naming convention
    let log_path = std::env::current_dir().unwrap().as_path().join("logs");

    // create log directory if missing
    std::fs::create_dir_all(&log_path).unwrap();

    // create log file name
    let log_file_name: String =
        "error_".to_string() + SystemTime::duration_since(UNIX_EPOCH).unwrap() + ".log";
    let log_file_path = log_path.join(log_file_name);

    // maintain log files
    let paths = std::fs::read_dir(log_path).unwrap();

    let mut logfiles: BTreeMap<i64, String> = BTreeMap::new();
    let mut logfile_times: Vec<i64> = vec![];
    for path in paths {
        let filename = String::from(path.as_ref().unwrap().path().to_str().unwrap());
        let metadata = std::fs::metadata(filename.clone()).unwrap();
        let mtime = FileTime::from_last_modification_time(&metadata);
        logfile_times.push(mtime.seconds());
        logfiles.insert(mtime.seconds(), filename);
    }
    logfile_times.sort();

    if logfile_times.len() > 2 {
        for i in 0..(logfile_times.len() - 2) {
            if let Some(filename) = logfiles.get(&logfile_times[i]) {
                std::fs::remove_file(std::path::Path::new(filename)).unwrap();
            }
        }
    }

    // find rust env var
    let mut env_log_level = String::from("error");
    for (key, value) in std::env::vars() {
        if key == "RUST_LOG" {
            env_log_level = value;
            break;
        }
    }

    // define log level
    let mut level_filter = log::LevelFilter::Error;
    if env_log_level == "warn" {
        level_filter = log::LevelFilter::Warn;
    } else if env_log_level == "debug" {
        level_filter = log::LevelFilter::Debug;
    } else if env_log_level == "info" {
        level_filter = log::LevelFilter::Info;
    } else if env_log_level == "trace" {
        level_filter = log::LevelFilter::Trace;
    }

    let env_logger = Box::new(
        pretty_env_logger::formatted_builder()
            .filter(None, level_filter)
            .build(),
    );
    let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
        simplelog::LevelFilter::Error,
        simplelog::Config::default(),
        File::create(log_file_path).unwrap(),
    ));
    multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info).unwrap();

    //log::trace!("test log to ensure that logging is working");
}
