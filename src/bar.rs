use crate::block::element::{Element, ElementFormat};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver};

pub struct Bar<'a> {
    blocks: Vec<Arc<Mutex<Element<'a>>>>,
    sep: Element<'a>,
    space: Element<'a>,
    space_time: Element<'a>,
    rx: Receiver<i32>
}

impl<'a> Bar<'a> {
    pub fn new(rx: Receiver<i32>) -> Bar<'a> {
        Bar { blocks: Vec::new(),
              sep: Element::sep(),
              space: Element::space(),
              space_time: Element::space_time(),
              rx: rx }
    }

    pub fn add(&mut self, elem: Arc<Mutex<Element<'a>>>) {
        self.blocks.push(elem);
    }

    pub fn run(&mut self) {

        // initialize infinity array
        //println!("{{ \"version\": 1, \"stop_signal\": 10, \"cont_signal\": 12, \"click_events\": true }}");
        println!("{{ \"version\": 1, \"stop_signal\": 20, \"cont_signal\": 18, \"click_events\": true }}");
        //println!("{{ \"version\": 1, \"click_events\": true }}");
        println!("[");
        
        loop {
            // wait for update command
            self.rx.recv().unwrap();

            // send all elements
            self.show();
        }
    }

    fn show(&self) {
        print!("  [");
        let mut is_start = true;
        for block in self.blocks.iter() {
            let elem = block.lock().unwrap();
            if elem.text.is_empty() { continue; }
            if is_start {
                print!("\n    ");
                is_start = false;
            } else {
                print!(",\n    ");
                self.sep.show();
                print!(",\n    ");
            }
            match elem.format {
                ElementFormat::Time => self.space_time.show(),
                _ => self.space.show()
            }
            print!(",\n");
            elem.show();
            print!(",\n");
            match elem.format {
                ElementFormat::Time => self.space_time.show(),
                _ => self.space.show()
            }
        }
        println!("\n  ],");
    }
}
