use doclib::html_file::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {

    // Get input file path
    let args: Vec<String> = env::args().collect();
    let mut html_doc = HtmlDoc::new();
    let filename = args[0].clone();

    // Html properties, or classes applied based on log level */
    let warn_class = Property::new("class", "warn");
    let error_class = Property::new("class", "error");
    let debug_class = Property::new("class", "debug");
    let info_class = Property::new("class", "info");

    // Parse input, and create htmldoc tags
    if let Ok(lines) = read_lines("LICENSE") {
        for line in lines {
            if let Ok(log_line) = line {
                let lower_log_line = log_line.to_lowercase().clone();
                println!("{}", lower_log_line);
                if log_line.contains("warn") {
                    html_doc
                        .tags
                        .push(Tag::new(warn_class, TagType::DIV, lower_log_line));
                } else if log_line.contains("error") {
                    html_doc
                        .tags
                        .push(Tag::new(error_class, TagType::DIV, lower_log_line));
                } else if log_line.contains("debug") {
                    html_doc
                        .tags
                        .push(Tag::new(debug_class, TagType::DIV, lower_log_line))
                } else if lower_log_line == "" {
                } else {
                    html_doc
                        .tags
                        .push(Tag::new(info_class, TagType::DIV, lower_log_line))
                }
            }
        }
    }

    // Parse path to create file name from log file name
    let mut name: &str;
    if filename.contains("/") {
        let split = filename.split("/");
        let vec = split.collect::<Vec<&str>>();
        name = vec[vec.len() - 1];

        let split2 = name.split(".");
        let vec = split2.collect::<Vec<&str>>();
        name = vec[0];
    } else if filename.contains("\\") {
        let split = filename.split("\\");
        let vec = split.collect::<Vec<&str>>();
        name = vec[vec.len() - 1];

        let split2 = name.split(".");
        let vec = split2.collect::<Vec<&str>>();
        name = vec[0];
    } else {
        name = &args[0];
    }

    // concatinate file name string
    let mut output_file_name: String = "".to_owned();
    output_file_name.push_str(name);
    output_file_name.push_str(".html");

    // Write to file
    let mut f = File::create(output_file_name.clone()).expect("Unable to create file");
    f.write_all(html_doc.to_string().as_bytes())
        .expect("Unable to write data");
    println!("created: {}", output_file_name);
}

// Get lines from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
