extern crate doclib;

use doclib::html_file::*;
use std::io::Write;

use std::fs::File;

use std::env;

use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut html_doc = HtmlDoc::new();

    //let mut logs = vec![];
    let path = get_path(env::args().collect());
    if let Ok(lines) = read_lines("D:/Git/test.log") {
        for line in lines {
            if let Ok(log_line) = line {
                let lower_log_line = log_line.to_lowercase().clone();

                if log_line.contains("warn") {
                    let warn_class = Property::new("class".to_string(), "warn".to_string());
                    html_doc
                        .tags
                        .push(Tag::new(warn_class, TagType::DIV, lower_log_line));
                } else if log_line.contains("error") {
                    let error_class = Property::new("class".to_string(), "error".to_string());
                    html_doc
                        .tags
                        .push(Tag::new(error_class, TagType::DIV, lower_log_line));
                } else if log_line.contains("debug") {
                    let debug_class = Property::new("class".to_string(), "debug".to_string());
                    html_doc
                        .tags
                        .push(Tag::new(debug_class, TagType::DIV, lower_log_line))
                } else {
                    let info_class = Property::new("class".to_string(), "info".to_string());
                    html_doc
                        .tags
                        .push(Tag::new(info_class, TagType::DIV, lower_log_line))
                }
            }
        }
    }

    html_doc.display();

    let mut f = File::create("output.html").expect("Unable to create file");
    for i in html_doc.tags {
        f.write_all(i.to_string().as_bytes())
            .expect("Unable to write data");
    }
}

fn get_path(args: Vec<String>) -> String {
    args[0].clone()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
