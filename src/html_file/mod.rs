const GREATER_THAN: &str = ">";
const SYTLE_SHEET: &str = " <link rel=\"stylesheet\" href=\"rshtml.css\">";

pub enum TagType {
    DIV,
    P,
    HTML,
    ARTICLE,
    HEAD,
    SCRIPT,
    FOOTER,
    BODY,
}

impl TagType {
    pub fn open_tag(&self) -> String {
        match *self {
            TagType::DIV => "<div".to_string(),
            TagType::P => "<p".to_string(),
            TagType::HTML => "<html".to_string(),
            TagType::ARTICLE => "<article".to_string(),
            TagType::HEAD => "<head".to_string(),
            TagType::SCRIPT => "<script".to_string(),
            TagType::FOOTER => "<footer".to_string(),
            TagType::BODY => "<body".to_string(),
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
            TagType::BODY => "</body>".to_string(),
        }
    }
}

use std::fmt::{self, Display, Formatter};
#[derive(Copy, Clone)]
pub struct Property {
    pub propertyname: &'static str,
    pub propertyvalue: &'static str,
}

// struct for html tag
pub struct Tag {
    pub properties: Property,
    pub tagtype: TagType,
    pub content: String,
}

impl Property {
    pub fn new(propertyname: &'static str, propertyvalue: &'static str) -> Property {
        Property {
            propertyname,
            propertyvalue,
        }
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, " {}={}", self.propertyname, self.propertyvalue)
    }
}

// impliment functions for tag struct
impl Tag {
    pub fn new(properties: Property, tagtype: TagType, content: String) -> Tag {
        Tag {
            properties,
            tagtype,
            content,
        }
    }

    pub fn display(&self) {
        println!(
            "{}{}{}{}{}",
            self.tagtype.open_tag(),
            self.properties,
            GREATER_THAN,
            self.content,
            self.tagtype.close_tag()
        );
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.tagtype.open_tag(),
            self.properties,
            GREATER_THAN,
            self.content,
            self.tagtype.close_tag()
        )
    }
}

// struct for an html doc
pub struct HtmlDoc {
    pub tags: Vec<Tag>,
}

// impliment functions for HtmlDoc struct
impl HtmlDoc {
    pub fn new() -> HtmlDoc {
        HtmlDoc { tags: vec![] }
    }
    pub fn display(&self) {
        for tag in self.tags.iter() {
            tag.display();
        }
    }
}

impl fmt::Display for HtmlDoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            TagType::HTML.open_tag(),
            GREATER_THAN,
            TagType::HEAD.open_tag(),
            GREATER_THAN,
            SYTLE_SHEET,
            TagType::HEAD.close_tag(),
            TagType::BODY.open_tag(),
            GREATER_THAN
        )?;

        for i in &self.tags {
            write!(f, "{}", i.to_string())?;
        }
        write!(
            f,
            "{}{}",
            TagType::BODY.close_tag(),
            TagType::HTML.close_tag()
        )?;
        Ok(())
    }
}
