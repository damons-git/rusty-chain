extern crate chrono;

use std::time::SystemTime;
use chrono::DateTime;
use chrono::offset::Utc;
use std::fs::{File, write, create_dir, read};
use std::io::{Write, Error, ErrorKind};
use std::fs::OpenOptions;
use crate::env::{DEBUG, SAVE_LOG};
use crate::LOGFILE;


// Add string to log
pub fn log(line: String) -> () {
    let text = format!("\n> {}", line);
    println!("{}", text);
    append_to_logfile(text);
}

// Create a titled section within the log.
pub fn tlog(title: &str, data: &[String]) -> () {
    let mut text = String::from("");
    text.push_str(&format!("\n> {}", title));
    text.push_str(&format!("\n{}", "-".repeat(title.len() + 2)));
    for line in data.iter() {
        text.push_str(&format!("\n\t - {}", line));
    }

    println!("{}", text);
    append_to_logfile(text);
}

// Add debug data to the log.
pub fn dlog(module: &str, desc: &str, data: &[String]) -> () {
    if DEBUG {
        let mut debug = String::from("");
        let ts: DateTime<Utc> = SystemTime::now().into();
        debug.push_str("\n== DEBUG ==");
        debug.push_str(&format!("\nmodule: {}", module));
        debug.push_str(&format!("\ndescription: {}", desc));
        debug.push_str(&format!("\ntimestamp: {}", ts.format("%d/%m/%Y %T")));
        for line in data.iter() {
            debug.push_str(&format!("\n\t - {}", line));
        }

        println!("{}", debug);
        append_to_logfile(debug);
    }
}

// Append data string to log file on disk.
// Create logs directory if does not exist.
fn append_to_logfile(mut data: String) -> () {
    if SAVE_LOG {
        match create_dir("logs/") {
            Err(why) => if Error::last_os_error().kind() != ErrorKind::AlreadyExists {
                    panic!("Failed to create logs directory: {}", why);
                },
            Ok(_) => ()
        };

        {
            let file_url = format!("logs/{}", LOGFILE.clone());
            data.push_str("\n");
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_url)
                .expect("Cannot open file.");
            file.write_all(&data.as_bytes()).expect("Failed to write to file");
        }
    }
}