use crate::block::element::{Element, ElementFormat};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver};
use std::thread;
use std::time::Duration;

const MAX_REFRESH_S: u64 = 1;
const MAX_REFRESH_NS: u32 = 0;//500_000_000;

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

        let mut must_update = false;
        
        loop {
            if !must_update {
                // wait for update command
                self.rx.recv().unwrap();
            }

            // show
            self.show();

            // remove all other entries
            while self.rx.try_recv().is_ok() {
                // if there are other entries, wait for the duration and then instantly show the update
                must_update = true;
            }

            thread::sleep(Duration::new(MAX_REFRESH_S, MAX_REFRESH_NS));
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
