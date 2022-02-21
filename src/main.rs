use doclib::html_file::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process;

fn main() {
    // Get input file path
    let args: Vec<String> = env::args().collect();
    let mut html_doc = HtmlDoc::new();
    html_doc.style = "<link rel=\"stylesheet\" href=\"src/css/rshtml.css\">".to_string();
    html_doc.script = "<script src=\"src/js/rshtmllog.js\"></script>".to_string();
    let filename = args[1].clone();
    println!("filename: {}", filename);

    // String constants
    const CLASS: &str = "class";
    const WARN: &str = "WARN";
    const ERROR: &str = "ERROR";
    const DEBUG: &str = "DEBUG";
    const TRACE: &str = "TRACE";
    const INFO: &str = "INFO";
    const ON_CLICK: &str = "onclick";
    const JS_FUNCTION: &str = "\"filter(this);\"";

    // Html properties, or classes applied based on log level */
    let warn_class = Property::new(CLASS, WARN);
    let error_class = Property::new(CLASS, ERROR);
    let debug_class = Property::new(CLASS, DEBUG);
    let info_class = Property::new(CLASS, INFO);
    let trace_class = Property::new(CLASS, TRACE);
    let onclick_property = Property::new(ON_CLICK, JS_FUNCTION);

    // Create Buttons
    let filter_header = Tag::new(TagType::H3, "filters".to_string());
    html_doc.tags.push(filter_header);
    add_filter_button(
        onclick_property,
        "Trace".to_string(),
        trace_class,
        &mut html_doc,
    );
    add_filter_button(
        onclick_property,
        "Debug".to_string(),
        debug_class,
        &mut html_doc,
    );
    add_filter_button(
        onclick_property,
        "Info".to_string(),
        info_class,
        &mut html_doc,
    );
    add_filter_button(
        onclick_property,
        "Warn".to_string(),
        warn_class,
        &mut html_doc,
    );
    add_filter_button(
        onclick_property,
        "Error".to_string(),
        error_class,
        &mut html_doc,
    );

    let mut warn_count = 0;
    let mut error_count = 0;
    let mut debug_count = 0;
    let mut info_count = 0;
    let mut trace_count = 0;

    // Parse input, and create htmldoc tags
    if let Ok(lines) = read_lines(filename.to_string()) {
        for line in lines {
            if let Ok(log_line) = line {
                let lower_log_line = log_line.to_lowercase().clone();
                let mut class = info_class;
                let mut isnotloghead = false;

                if log_line.contains(DEBUG) {
                    debug_count += 1;
                    class = debug_class;
                } else if log_line.contains(INFO) {
                    info_count += 1;
                    class = info_class;
                } else if log_line.contains(WARN) {
                    warn_count += 1;
                    class = warn_class;
                } else if log_line.contains(ERROR) {
                    error_count += 1;
                    class = error_class;
                } else if log_line.contains(TRACE) {
                    trace_count += 1;
                    class = trace_class;
                } else if log_line.len() == 0 {
                    continue;
                } else {
                    isnotloghead = true;
                    let index = html_doc.tags.len() - 1;
                    let last_tag = &mut html_doc.tags[index];
                    last_tag.content.push_str(&lower_log_line);
                }

                if !isnotloghead {
                    add_log_level_tag(class, log_line, &mut html_doc)
                }
            }
        }
    } else {
        println!("File not found: {}", filename);
        process::exit(0x0100);
    }

    println!("Trace Count: {}", trace_count);
    println!("Debug Count: {}", debug_count);
    println!("Info  Count: {}", info_count);
    println!("Warn  Count: {}", warn_count);
    println!("Error Count: {}", error_count);

    let mut name: &str;
    let mut lhpath: String = "".to_string();
    // Universal directory seperator
    if filename.contains('/') {
        let split = filename.split('/');
        let vec = split.collect::<Vec<&str>>();
        for (index, _element) in vec.iter().enumerate() {
            if index != vec.len() - 1 {
                let vecstr = String::from(vec[index]);
                lhpath.push_str(&vecstr);
                lhpath.push_str(&"/".to_string());
            }
        }
        name = vec[vec.len() - 1];

        let split2 = name.split('.');
        let vec = split2.collect::<Vec<&str>>();
        name = vec[0];

    // Windows  specific path seperator
    } else if filename.contains('\\') {
        let split = filename.split('\\');
        let vec = split.collect::<Vec<&str>>();
        for (index, _element) in vec.iter().enumerate() {
            if index != vec.len() - 1 {
                let vecstr = String::from(vec[index]);
                lhpath.push_str(&vecstr);
                lhpath.push_str(&"\\".to_string());
            }
        }
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
    println!("lhpath: {}", lhpath);
    output_file_name.push_str(&lhpath);
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
    let mut div = Tag::new(TagType::DIV, "".to_string());
    div.properties.push(propterty);
    div.properties
        .push(Property::new("onclick", "\"view(this);\""));
    div.children.push(Tag::new(TagType::P, text));
    html_doc.tags.push(div);
}

// Add filter button
fn add_filter_button(
    property: Property,
    text: String,
    class: Property,
    html_doc: &mut doclib::html_file::HtmlDoc,
) {
    let mut filter_button = Tag::new(TagType::BUTTON, text.to_string());
    filter_button.properties.push(class);
    filter_button.properties.push(property);
    html_doc.tags.push(filter_button);
}
