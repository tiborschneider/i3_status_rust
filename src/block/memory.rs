use super::element::{Element, ElementFormat};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use systemstat::{System, Platform};


const ICON: char = '\u{f2db}';


pub fn memory_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>) {
    let sys = System::new();
    loop {
        let mem = sys.memory().unwrap();
        let perc = 100.0 - ((mem.free.as_usize() as f64) * 100.0) / (mem.total.as_usize() as f64);
        let new_text = String::from(format!("{} {:.1}%", ICON, perc));
        
        // get mutex
        let mut updated = false;
        if let Ok(mut e) = elem.lock() {
            // check if text has changed
            if e.text != new_text {
                e.set_text(new_text);
                if perc > 75.0 { e.set_format(ElementFormat::Error); }
                else if perc > 50.0 { e.set_format(ElementFormat::Warning); }
                else { e.set_format(ElementFormat::Normal); }
                updated = true;
            }
        }
        // mutex closed here

        if updated {
            if tx.send(1).is_err() {
                eprintln!("(time): cannot communicate to main bar. Abort!");
                break;
            }
        }

        // sleep for one second
        thread::sleep(Duration::new(2, 0));
    }
}
