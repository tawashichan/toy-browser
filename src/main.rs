use std::{collections::HashMap, fs::read_to_string, iter::Peekable, str::Chars};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Element(Element),
    Text(Text),
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub tag_name: String, // body,divなど
    pub attrinbutes: AttrMap,
}

#[derive(Debug, PartialEq)]
pub struct Text {
    pub data: String,
}

fn parse(s: &mut Peekable<Chars>) -> Node {
    parse_node(s).unwrap()
}

fn parse_nodes(s: &mut Peekable<Chars>) -> Vec<Node> {
    let mut nodes = vec![];
    loop {
        match s.next() {
            Some(_) => {
                let node = parse_node(s);
                if let Some(node) = node {
                    nodes.push(node)
                }
            }
            None => return nodes,
        }
    }
}

fn parse_node(s: &mut Peekable<Chars>) -> Option<Node> {
    match s.peek() {
        Some('<') => Some(parse_element(s)),
        Some(_) => parse_text(s),
        _ => unimplemented!(),
    }
}

fn parse_element(s: &mut Peekable<Chars>) -> Node {
    let (tag, attr) = parse_element_head(s);
    let nodes = parse_nodes(s);
    parse_element_tail(s, &tag);
    Node {
        node_type: NodeType::Element(Element {
            tag_name: tag,
            attrinbutes: attr,
        }),
        children: nodes,
    }
}

fn parse_text(s: &mut Peekable<Chars>) -> Option<Node> {
    let mut result = String::from("");
    loop {
        let peek = s.peek();
        dbg!(peek);
        match peek {
            Some('<') => break,
            Some(_) => {
                let next = s.next().unwrap();
                result.push(next);
            }
            None => return None,
        }
    }
    Some(Node {
        node_type: NodeType::Text(Text { data: result }),
        children: vec![],
    })
}

fn parse_element_head(s: &mut Peekable<Chars>) -> (String, AttrMap) {
    s.next(); // consume >
    let tag = parse_tag(s);
    let attr = parse_attr(s);
    s.next(); // consume >
    (tag, attr)
}

fn parse_element_tail(s: &mut Peekable<Chars>, tag_name: &str) {
    s.next(); // consume <
    if let Some('/') = s.next() {
        let tag = parse_tag(s);
        if tag != tag_name {
            panic!("unmatched tag_name, head: {:?}, tail: {:?}", tag_name, tag)
        }
        loop {
            match s.next() {
                Some(' ') => {}
                Some('>') => break,
                _ => panic!("unexpected token"),
            }
        }
    }
}

fn parse_tag(s: &mut Peekable<Chars>) -> String {
    match s.peek() {
        Some('>') => {
            panic!("unexpected token: >")
        }
        Some(_) => parse_tag_sub(s),
        None => {
            panic!("unexpected end of input")
        }
    }
}

fn parse_tag_sub(s: &mut Peekable<Chars>) -> String {
    let mut result = String::from("");
    loop {
        let peek = s.peek();
        match peek {
            Some(' ') | Some('>') => return result,
            Some(_) => {
                let next = s.next().unwrap();
                result.push(next);
            }
            None => panic!("unexpected end of input"),
        }
    }
}

fn parse_attr(s: &mut Peekable<Chars>) -> AttrMap {
    let mut attr_map = AttrMap::new();
    loop {
        match s.peek() {
            Some(' ') => {
                s.next();
            }
            Some('>') => {
                s.next(); // consume >
                break;
            }
            Some(_) => {
                parse_attr_sub(s, &mut attr_map);
            }
            None => panic!("unexpected end of input"),
        }
    }
    attr_map
}

fn parse_attr_sub<'a>(s: &mut Peekable<Chars>, attr_map: &'a mut AttrMap) {
    match s.peek() {
        Some('>') => {} // 中断
        Some(head) => parse_attr_body(s, attr_map),
        _ => unimplemented!(),
    };
}

fn parse_attr_key(s: &mut Peekable<Chars>) -> String {
    let mut result = String::from("");
    loop {
        let peek = s.peek();
        match peek {
            Some(' ') | Some('>') | Some('=') => return result,
            Some(_) => {
                let next = s.next().unwrap();
                result.push(next);
            }
            None => panic!("unexpected end of input"),
        }
    }
}

// <div id="aaa"></di>の"aaa"の部分
fn parse_attr_value(s: &mut Peekable<Chars>) -> String {
    let mut result = String::from("");
    loop {
        let peek = s.peek();
        match peek {
            Some(' ') | Some('>') => return result,
            Some('"') => {
                s.next(); // skip ",
            }
            Some(_) => {
                let next = s.next().unwrap();
                result.push(next);
            }
            None => panic!("unexpected end of input"),
        }
    }
}

fn parse_attr_body<'a>(s: &mut Peekable<Chars>, attr_map: &'a mut AttrMap) {
    let key = parse_attr_key(s);
    match s.next() {
        Some('=') => {
            let value = parse_attr_value(s);
            attr_map.insert(key, value);
        }
        next => {
            unimplemented!()
        }
    }
}

fn main() {
    let s = "<body><body>";
    parse(&mut s.chars().peekable());
}

#[test]
fn parse_element_head_test() {
    let str = "<body>";
    let result = parse_element_head(&mut str.chars().peekable());
    let expected = ("body".to_string(), AttrMap::new());
    assert_eq!(expected, result);

    let str = "<div id=\"nyan\">";
    let result = parse_element_head(&mut str.chars().peekable());
    let mut attr_map = HashMap::new();
    attr_map.insert("id".to_string(), "nyan".to_string());

    let expected = ("div".to_string(), attr_map);
    assert_eq!(expected, result);

    let str = "<div id=\"nyan\" class=\"aaa\">";
    let result = parse_element_head(&mut str.chars().peekable());
    let mut attr_map = HashMap::new();
    attr_map.insert("id".to_string(), "nyan".to_string());
    attr_map.insert("class".to_string(), "aaa".to_string());

    let expected = ("div".to_string(), attr_map);
    assert_eq!(expected, result)
}

#[test]
fn parse_element_test() {
    let str = "<body></body>";
    let result = parse_element(&mut str.chars().peekable());
    let expected = Node {
        node_type: NodeType::Element(Element {
            tag_name: "body".to_string(),
            attrinbutes: HashMap::new(),
        }),
        children: vec![],
    };
    assert_eq!(expected, result);
}

#[test]
fn parse_tag_sub_test() {
    let str = "body ";
    let result = parse_tag_sub(&mut str.chars().peekable());
    let expected = "body";
    assert_eq!(expected, result)
}

#[test]
fn test_parse() {
    /*    let html = read_to_string("./simple.html").unwrap();
    let result = parse(&mut html.chars().peekable());

    let mut div_attr = HashMap::new();
    div_attr.insert("class".to_string(), "aa".to_string());
    div_attr.insert("id".to_string(), "aaa".to_string());

    let goal = Node {
        node_type: NodeType::Element(Element {
            tag_name: String::from("body"),
            attrinbutes: HashMap::new(),
        }),
        children: vec![Node {
            node_type: NodeType::Element(Element {
                tag_name: String::from("div"),
                attrinbutes: div_attr,
            }),
            children: vec![Node {
                node_type: NodeType::Text(Text {
                    data: "nyan".to_string(),
                }),
                children: vec![],
            }],
        }],
    };

    assert_eq!(goal, result);*/
}
