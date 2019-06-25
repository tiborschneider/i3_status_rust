use super::element::{Element, ElementFormat};
use crate::event::Event;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

use systemstat::{System, Platform};


const ICON: char = '\u{f85a}';
const TEMP_ICON: char = '\u{fa03}';


pub fn cpu_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>, event: Receiver<Event>) {
    let sys = System::new();
    let mut extended = false;
    loop {
        // start measurement
        let cpu = sys.cpu_load_aggregate().unwrap();

        // wait one second
        thread::sleep(Duration::new(1, 0));

        // evaluate measurement
        let cpu = cpu.done().unwrap();

        // get percentage
        let perc = (1.0 - cpu.idle) * 100.0;
        let mut new_text = String::from(format!("{} {:.1}%", ICON, perc));

        // append temperature if necessary
        if extended {
            match sys.cpu_temp() {
                Ok(temp) => new_text.push_str(&format!(", {:.0}{}", temp, TEMP_ICON)),
                Err(_) => eprintln!("could not receive cpu temperature"),
            }
        }
        
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

        // check if new messages have arrived
        extended = match event.try_recv() {
            Ok(_) => !extended,
            Err(_) => extended
        }
    }
}
