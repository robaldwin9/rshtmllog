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

    // String constants
    const CLASS: &'static str = "class";
    const WARN: &'static str = "warn";
    const ERROR: &'static str = "error";
    const DEBUG: &'static str = "debug";
    const INFO: &'static str = "info";

    // Html properties, or classes applied based on log level */
    let warn_class = Property::new(CLASS, WARN);
    let error_class = Property::new(CLASS, ERROR);
    let debug_class = Property::new(CLASS, DEBUG);
    let info_class = Property::new(CLASS, INFO);

    // Parse input, and create htmldoc tags
    if let Ok(lines) = read_lines("LICENSE") {
        for line in lines {
            if let Ok(log_line) = line {
                let lower_log_line = log_line.to_lowercase().clone();
                println!("{}", lower_log_line);

                if log_line.contains(WARN) {
                    add_log_level_tag(warn_class, lower_log_line, &mut html_doc);
                } else if log_line.contains(ERROR) {
                    add_log_level_tag(error_class, lower_log_line, &mut html_doc);
                } else if log_line.contains(DEBUG) {
                    add_log_level_tag(debug_class, lower_log_line, &mut html_doc);
                } else if lower_log_line.is_empty() {
                } else {
                    add_log_level_tag(info_class, lower_log_line, &mut html_doc)
                }
            }
        }
    }

    // Parse path to create file name from log file name
    let mut name: &str;

    // Universal directory seperator
    if filename.contains("/") {
        let split = filename.split("/");
        let vec = split.collect::<Vec<&str>>();
        name = vec[vec.len() - 1];

        let split2 = name.split(".");
        let vec = split2.collect::<Vec<&str>>();
        name = vec[0];

    // Windows  specific path seperator
    } else if filename.contains("\\") {
        let split = filename.split("\\");
        let vec = split.collect::<Vec<&str>>();
        name = vec[vec.len() - 1];

        let split2 = name.split(".");
        let vec = split2.collect::<Vec<&str>>();
        name = vec[0];

    // Path is not an absolute
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

// add div, with paragraph child containing log line
fn add_log_level_tag(propterty: Property, text: String, html_doc: &mut doclib::html_file::HtmlDoc) {
    let mut div = Tag::new(propterty, TagType::DIV, "".to_string());
    div.children
        .push(Tag::new(Property::new("", ""), TagType::P, text));
    html_doc.tags.push(div);
}
