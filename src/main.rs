use doclib::html_file::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    // Get input file path
    let args: Vec<String> = env::args().collect();
    let mut html_doc = HtmlDoc::new();
    html_doc.style = "<link rel=\"stylesheet\" href=\"src/css/rshtml.css\">".to_string();
    html_doc.script = "<script src=\"src/js/YourExternalJQueryScripts.js\"></script>".to_string();
    let filename = args[0].clone();

    // String constants
    const CLASS: &str = "class";
    const WARN: &str = "warn";
    const ERROR: &str = "error";
    const DEBUG: &str = "debug";
    const INFO: &str = "info";

    // Html properties, or classes applied based on log level */
    let warn_class = Property::new(CLASS, WARN);
    let error_class = Property::new(CLASS, ERROR);
    let debug_class = Property::new(CLASS, DEBUG);
    let info_class = Property::new(CLASS, INFO);

    let mut warn_count = 0;
    let mut error_count = 0;
    let mut debug_count = 0;
    let mut info_count = 0;

    let filter_header = Tag::new(Property::new("", ""), TagType::H3, "filters".to_string());
    html_doc.tags.push(filter_header);
    let warn_button = Tag::new(warn_class, TagType::BUTTON, "Warn".to_string());
    let error_button = Tag::new(error_class, TagType::BUTTON, "Error".to_string());
    let debug_button = Tag::new(debug_class, TagType::BUTTON, "Debug".to_string());
    let info_button = Tag::new(info_class, TagType::BUTTON, "Info".to_string());
    html_doc.tags.push(debug_button);
    html_doc.tags.push(warn_button);
    html_doc.tags.push(error_button);
    html_doc.tags.push(info_button);

    // Parse input, and create htmldoc tags
    if let Ok(lines) = read_lines("test.log") {
        for line in lines {
            if let Ok(log_line) = line {
                let lower_log_line = log_line.to_lowercase().clone();
                println!("{}", lower_log_line);

                if log_line.contains(WARN) {
                    if warn_count == 0 {
                        warn_count += 1;
                    }
                    add_log_level_tag(warn_class, lower_log_line, &mut html_doc);
                } else if log_line.contains(ERROR) {
                    if error_count == 0 {
                        error_count += 1;
                    }
                    add_log_level_tag(error_class, lower_log_line, &mut html_doc);
                } else if log_line.contains(DEBUG) {
                    if debug_count == 0 {
                        debug_count += 1;
                    }
                    add_log_level_tag(debug_class, lower_log_line, &mut html_doc);
                } else if lower_log_line.is_empty() {
                } else {
                    if info_count == 0 {
                        info_count += 1;
                    }
                    add_log_level_tag(info_class, lower_log_line, &mut html_doc)
                }
            }
        }
    }

    let mut name: &str;
    // Universal directory seperator
    if filename.contains('/') {
        let split = filename.split('/');
        let vec = split.collect::<Vec<&str>>();
        name = vec[vec.len() - 1];

        let split2 = name.split('.');
        let vec = split2.collect::<Vec<&str>>();
        name = vec[0];

    // Windows  specific path seperator
    } else if filename.contains('\\') {
        let split = filename.split('\\');
        let vec = split.collect::<Vec<&str>>();
        name = vec[vec.len() - 1];

        let split2 = name.split('.');
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
