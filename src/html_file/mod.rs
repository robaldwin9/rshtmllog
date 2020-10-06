pub enum TagType {
	DIV,
	P,
	HTML,
	ARTICLE,
	HEAD,
	SCRIPT,
	FOOTER,
	BODY
}

impl TagType{

	pub fn open_tag(&self) -> String {
		match *self {
			TagType::DIV => "<div>".to_string(),
			TagType::P => "<p>".to_string(),
			TagType::HTML => "<html>".to_string(),
			TagType::ARTICLE => "<article>".to_string(),
			TagType::HEAD => "<head>".to_string(),
			TagType::SCRIPT => "<script>".to_string(),
			TagType::FOOTER => "<footer>".to_string(),
			TagType::BODY => "<body>".to_string()
		}
	}

	pub fn close_tag(&self) -> String {
			match *self {
			TagType::DIV => "</div>".to_string(),
			TagType::P => "</p>".to_string(),
			TagType::HTML => "</html>".to_string(),
			TagType::ARTICLE => "</article>".to_string(),
			TagType::HEAD => "</head>".to_string(),
			TagType::SCRIPT => "</script>".to_string(),
			TagType::FOOTER => "</footer>".to_string(),
			TagType::BODY => "</body>".to_string()
		}
	}
}

use std::fmt::{self,Formatter,Display};

// struct for html tag
pub struct Tag {

	pub properties: String,
	pub tagtype: TagType,
	pub content: String
}

pub struct Property {

	pub propertyname: String,
	pub propertyvalue: String
}

impl Property {
	
	pub fn new(propertyname: String, propertyvalue: String)
	{
		propertyname: propertyname,
		propertyvalue: propertyvalue,
	}
}

// impliment functions for tag struct
impl Tag {

	pub fn new(properties: String, tagtype: TagType, content: String) -> Tag {
		Tag {

			properties: properties,
			tagtype: tagtype,
			content: content
		}
	}

	pub fn display(&self) {

		println!("{}{}{}{}", self.tagtype.open_tag(), 
			self.properties, self.content, self.tagtype.close_tag());
	}


}

impl Display for Tag {
	fn fmt (&self, f: &mut Formatter) -> fmt::Result {

		write!(f,"{}{}{}{}",self.tagtype.open_tag(), 
			self.properties,self.content, self.tagtype.close_tag())
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
