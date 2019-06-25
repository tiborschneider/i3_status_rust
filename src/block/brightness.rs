use super::element::Element;
use crate::event::Event;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

use std::fs::File;
use std::io::Read;

const PERIOD_MS: u64 = 100;
const MAX_VALUE: i32 = 852;
const ICON_RAMP: [char; 3] = ['\u{f5dd}', '\u{f5de}', '\u{f5df}'];


pub fn brightness_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>, event: Receiver<Event>) {
    loop {
        // read current brightness level
        let val: i32 = match get_brightness_val() {
            Ok(val) => val,
            Err(_) => {
                thread::sleep(Duration::from_millis(1));
                continue;
            }
        };
        let perc = (val * 100) / MAX_VALUE;
        let mut icon_idx: usize = ((val * 3 / MAX_VALUE)) as usize;
        if icon_idx > 2 { icon_idx = 2; }
        // prepare text
        let mut new_text: String = String::with_capacity(6);
        new_text.push(ICON_RAMP[icon_idx]);
        new_text.push(' ');
        new_text.push_str(&format!("{}", perc));
        new_text.push('%');

        // get mutex
        let mut updated = false;
        if let Ok(mut e) = elem.lock() {
            // check if text has changed
            if e.text != new_text {
                e.set_text(new_text);
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
        match event.recv_timeout(Duration::from_millis(PERIOD_MS)) {
            Ok(_) => {},
            Err(_) => {}
        };
    }
}

fn get_brightness_val() -> Result<i32, ()> {
    if let Ok(mut f) = File::open("/sys/class/backlight/intel_backlight/brightness") {
        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
            Ok(_) => {
                match buffer.trim().parse::<i32>() {
                    Ok(val) => Ok(val),
                    Err(_e) => Err(())
                }
            },
            Err(_) => Err(())
        }
    } else {
        Err(())
    }
}
