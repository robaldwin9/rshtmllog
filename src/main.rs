extern crate doclib;

use doclib::html_file::*;
use doclib::log_file::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
	let mut logs = vec![];
	let path =  get_path(env::args().collect());
	if let Ok(lines) = read_lines("D:/Git/test.log") {
		for line in lines {
			if let Ok(log_line) = line {

				let lower_log_line = log_line.to_lowercase().clone();

				if log_line.contains("warn") {

					logs.push(Entry::new(LogLevel::WARN, lower_log_line))
				}

				else if log_line.contains("error") {

					logs.push(Entry::new(LogLevel::ERROR, lower_log_line))
				}

				else if log_line.contains("debug"){

					logs.push(Entry::new(LogLevel::DEBUG, lower_log_line))

				}

				else if log_line.contains("info"){

					logs.push(Entry::new(LogLevel::INFO, lower_log_line))

				}

				else {

					logs.push(Entry::new(LogLevel::INFO, lower_log_line))
				}
				println!("{}", log_line);
				logs.push(Entry::new(LogLevel::INFO, log_line))

			}
		}
	}
	let tag = Tag::new("class=info".to_string(),TagType::DIV,"[INFO] info log info".to_string());
    println!("{}",tag);
}


fn get_path(args: Vec<String>) -> String {

	return args[0].clone();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
