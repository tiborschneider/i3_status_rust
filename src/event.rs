use std::sync::mpsc::Sender;
use std::io;
use std::io::prelude::*;

use json;

#[allow(dead_code)]
pub struct Event {
    button: i32,
    modifiers: Vec<EventModifier>,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

pub enum EventModifier {
    Shift,
    Ctrl,
    Mod1,
    Mod4
}

struct EventBlock<'a> {
    name: &'a str,
    tx: Sender<Event>
}

pub struct EventSystem<'a> {
    blocks: Vec<EventBlock<'a>>
}

impl<'a> EventSystem<'a> {
    pub fn new() -> EventSystem<'a> {
        EventSystem{ blocks: Vec::new() }
    }

    pub fn add(&mut self, name: &'a str, tx: Sender<Event>) {
        self.blocks.push(EventBlock{ name: name, tx: tx });
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();

        // initially, expect a single "["
        let mut dummy: String = String::new();
        stdin.lock().read_line(&mut dummy).unwrap();
        
        // wait for input
        for line in stdin.lock().lines() {
            let data = json::parse(line.unwrap().trim_matches(',')).unwrap();
            // find the correct channel
            let target_name: String = data["name"].to_string();
            let tx = match self.blocks.iter()
                                      .filter(|x| x.name == target_name)
                                      .map(|x| x.tx.clone())
                                      .next() {
                Some(_tx) => _tx,
                None => { continue; } 
            };

            // prepare modifiers
            let mut modifiers: Vec<EventModifier> = Vec::new();
            match &data["modifiers"] {
                json::JsonValue::Array(_mod) => {
                    for modifier in _mod {
                        modifiers.push(match modifier.as_str().unwrap() {
                            "Shift" => EventModifier::Shift,
                            "Mod1" => EventModifier::Mod1,
                            "Mod4" => EventModifier::Mod4,
                            "Control" => EventModifier::Ctrl,
                            _ => { continue; }
                        });
                    }
                },
                _ => { continue; }
            }

            // send event
            tx.send(Event{ button: data["button"].as_i32().unwrap(),
                           modifiers: modifiers,
                           x: data["x"].as_usize().unwrap(),
                           y: data["y"].as_usize().unwrap(),
                           width: data["width"].as_usize().unwrap(),
                           height: data["height"].as_usize().unwrap()}).unwrap();
        }
    }
}
