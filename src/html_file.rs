
use std::fmt::{self,Formatter,Display};

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

		println!("{}{} {}{}{}{}{}{}","<",self.tagtype, 
			self.properties,">",self.content, "<", self.tagtype, "/>");
	}


}

impl Display for Tag {
	fn fmt (&self, f: &mut Formatter) -> fmt::Result {
		write!(f,"{}{} {}{}{}{}{}{}","<",self.tagtype, 
			self.properties,">",self.content, "<", self.tagtype, "/>")
	}
}

// struct for an html doc
pub struct HtmlDoc {
	
	tags: Vec<Tag>
}

// impliment functions for HtmlDoc struct
impl HtmlDoc {
	pub fn display(&self) {
		for tag in self.tags.iter() {
			tag.display();
		}
	}
}

