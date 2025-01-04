use std::collections::HashMap;

//dom module
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrMap,
}

type AttrMap = HashMap<String, String>;

impl Node {
    pub fn text(data: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Self {
        Node {
            children,
            node_type: NodeType::Element(ElementData { tag_name, attrs }),
        }
    }
}

/// html parser
pub struct Parser {
    pub position: usize,
    pub input: String,
}

impl Parser {
    pub fn next_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    pub fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.position += s.len()
        } else {
            panic!(
                "Expected {:?} at byte {} but it was not found",
                s, self.position,
            )
        }
    }

    pub fn eof(&self) -> bool {
        self.position >= self.input.len()
    }

    pub fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.position += c.len_utf8();
        c
    }

    //consume chars until test returns false
    pub fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    //parse a tag or attr name
    pub fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' |'0'..='9'))
    }

    pub fn parse_node(&mut self) -> Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    pub fn parse_text(&mut self) -> Node {
        Node::text(self.consume_while(|c| c != '<'))
    }

    pub fn parse_element(&mut self) -> Node {
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        //content
        let children = self.parse_nodes();

        self.expect("</");
        self.expect(&tag_name);
        self.expect(">");

        Node::elem(tag_name, attrs, children)
    }

    pub fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        (name, value)
    }

    pub fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        value
    }

    pub fn parse_attributes(&mut self) -> AttrMap {
        let mut attrs = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }

            let (name, value) = self.parse_attr();
            attrs.insert(name, value);
        }

        attrs
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }
}

//parse the html doc and return root element
pub fn parse(source: String) -> Node {
    let mut nodes = Parser {
        position: 0,
        input: source,
    }
    .parse_nodes();

    //if no root element, create one
    if nodes.len() == 1 {
        nodes.remove(0)
    } else {
        Node::elem("html".to_string(), HashMap::new(), nodes)
    }
}
