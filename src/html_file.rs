/**
TODO:
* Add ability to have multiple properties in a tag
* Add rest of header tags



*/

const GREATER_THAN: &str = ">";

pub enum TagType {
    DIV,
    P,
    HTML,
    ARTICLE,
    HEAD,
    SCRIPT,
    FOOTER,
    BODY,
    BUTTON,
    H3,
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
            TagType::BUTTON => "<button".to_string(),
            TagType::H3 => "<h3".to_string(),
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
            TagType::BUTTON => "</button>".to_string(),
            TagType::H3 => "</h3>".to_string(),
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
    pub properties: Vec<Property>,
    pub tagtype: TagType,
    pub content: String,
    pub children: Vec<Tag>,
}

impl Property {
    pub fn new(propertyname: &'static str, propertyvalue: &'static str) -> Property {
        Property {
            propertyname,
            propertyvalue,
        }
    }

        pub fn display(&self) {
           if self.propertyname.len() > 0 && self.propertyvalue.len() > 0 {
            println!( " {}={} ", self.propertyname, self.propertyvalue)
        } else {
            println!("")
        }
            
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.propertyname.len() > 0 && self.propertyvalue.len() > 0 {
            write!(f, " {}={} ", self.propertyname, self.propertyvalue)
        } else {
            write!(f, "")
        }
    }
}

// impliment functions for tag struct
impl Tag {
    pub fn new(tagtype: TagType, content: String) -> Tag {
        Tag {
            properties: vec![],
            tagtype,
            content,
            children: vec![],
        }
    }

    pub fn display(&self) {
        println!(
            "{}",
            self.tagtype.open_tag(),
            //self.properties,
           // GREATER_THAN
        );

           
               for property in self.properties.iter() {
            property.display();
        }

        println!("{}", GREATER_THAN);

        for item in self.children.iter() {
            item.display();
        }

        println!("{}{}", self.content, self.tagtype.close_tag());
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tagtype.open_tag(),

        )?;


          for property in self.properties.iter() {
            write!(f, "{}", property.to_string())?;
        }

        write!(f, "{}", GREATER_THAN)?;

        for child in self.children.iter() {
            write!(f, "{}", child.to_string())?;
        }
        write!(f, "{}{}", self.content, self.tagtype.close_tag())
    }
}

// struct for an html doc
pub struct HtmlDoc {
    pub script: String,
    pub style: String,
    pub tags: Vec<Tag>,
}

// impliment functions for HtmlDoc struct
impl HtmlDoc {
    pub fn new() -> HtmlDoc {
        HtmlDoc {
            tags: vec![],
            script: "".to_string(),
            style: "".to_string(),
        }
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
            "{}{}{}{}{}{}{}{}{}",
            TagType::HTML.open_tag(),
            GREATER_THAN,
            TagType::HEAD.open_tag(),
            GREATER_THAN,
            self.style,
            self.script,
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
