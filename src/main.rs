use std::str;

use quick_xml::{
    events::{attributes::Attributes, Event},
    Reader as XMLReader,
};

enum Protocol {
    TCP,
    UDP,
}

struct Rule {
    name: Option<String>,
    proto: Protocol,
    host_ip: String,
    host_port: u16,
    guest_ip: String,
    guest_port: u16,
}

impl Rule {
    fn from_attributes(attrs: Attributes) -> Rule {
        let mut name = None;
        let mut proto = None;
        let mut host_ip = None;
        let mut host_port = None;
        let mut guest_ip = None;
        let mut guest_port = None;
        for (key, val) in attrs.map(|a| a.unwrap()).map(move |a| {
            (
                str::from_utf8(a.key).unwrap(),
                str::from_utf8(&a.value).unwrap(),
            )
        }) {
            match key {
                k if k == "name" => {
                    name = Some(String::from(val))
                }
                k if k == "proto" => proto = Some(match val {}),
                _ => (),
            }
        }
        Rule {
            name,
            proto: proto.unwrap(),
            host_ip: host_ip.unwrap(),
            host_port: host_port.unwrap(),
            guest_ip: guest_ip.unwrap(),
            guest_port: guest_port.unwrap(),
        }
    }
}

fn main() {
    let mut parser = XMLReader::from_file("test.vbox").unwrap();
    let mut buf = Vec::new();
    let xmlpath = [
        "VirtualBox",
        "Machine",
        "Hardware",
        "Network",
        "Adapter",
        "NAT",
        "Forwarding",
    ];
    let mut matchptr = 0;
    let mut ptr = 0;
    let mut rules: Vec<Rule> = Vec::new();
    loop {
        match parser.read_event(&mut buf).unwrap() {
            Event::Start(ref e) => {
                if matchptr < xmlpath.len()
                    && ptr == matchptr
                    && str::from_utf8(e.name()).unwrap() == xmlpath[matchptr]
                {
                    matchptr += 1;
                }
                ptr += 1;

                if matchptr == 8 {
                    rules.push(Rule::from_attributes(e.attributes()));
                }
            }
            Event::End(_) => {
                if matchptr == ptr {
                    matchptr -= 1;
                }
                ptr -= 1;
            }
            Event::Eof => break,
            _ => (),
        }
    }
}
