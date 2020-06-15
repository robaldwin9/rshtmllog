extern crate doclib;

use doclib::html_file::Tag;

fn main() {

	let tag = Tag::new("class=info ".to_string(),"div".to_string(),"[INFO] info log info".to_string());
    println!("{}",tag);
}
