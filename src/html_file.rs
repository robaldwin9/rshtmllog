
use std::fmt::{self,Formatter,Display};

// constants for opening and closing tags
const SGT: &str = "/>";
const LT: &str = "<";
const GT: &str = ">";

// struct for an html doc
pub struct HtmlDoc {

	tags: Vec<Tag>
}

// struct for html tag
pub struct Tag {

	pub properties: String,
	pub tagtype: String,
	pub content: String
}

// impliment functions for tag struct
impl Tag {

	pub fn new(properties: String, tagtype: String, content: String) -> Tag {
		Tag {
			properties: properties,
			tagtype: tagtype,
			content: content

		}
	}

		pub fn display(&self) {

		println!("{}{} {}{}{}{}{}{}",LT,self.tagtype, 
			self.properties,GT,self.content, LT, self.tagtype, SGT);
	}


}

impl Display for Tag {
	fn fmt (&self, f: &mut Formatter) -> fmt::Result {
		write!(f,"{}{} {}{}{}{}{}{}","<",self.tagtype, 
			self.properties,">",self.content, "<", self.tagtype, "/>")
	}
}

// impliment functions for HtmlDoc struct
impl HtmlDoc {
	pub fn display(&self) {
		for tag in self.tags.iter() {
			tag.display();
		}
	}
}