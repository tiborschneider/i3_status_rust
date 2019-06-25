use super::element::{Element, PangoFormat, PangoFontWeight};
use crate::event::Event;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};

use std::time::Duration;
use chrono::{Local, DateTime};

const PERIOD: u64 = 1;
const LONG_FORMAT: &str = "%a, %m.%d.%Y %H:%M";
const SHORT_FORMAT: &str = "%H:%M";
const ICON: char = '\u{f64f}';

pub fn time_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>, event: Receiver<Event>) {
    let mut extended = false;
    let mut old_text = String::new();
    loop {

        // compute updated time string
        let dt: DateTime<Local> = Local::now();
        let new_text: String = match extended {
            true => dt.format(LONG_FORMAT).to_string(),
            false => dt.format(SHORT_FORMAT).to_string(),
        };

        let mut updated = false;
        if old_text != new_text {
            // get mutex
            if let Ok(mut e) = elem.lock() {
                old_text = new_text;
                e.clear_text();
                e.append_pango(&format!("{} ", ICON), vec![PangoFormat::FontWeight(PangoFontWeight::Heavy)]);
                e.append_pango(old_text.as_str(), vec![PangoFormat::FontWeight(PangoFontWeight::Bold)]);
                updated = true;
            }
            // mutex closed here
        }

        if updated {
            if tx.send(1).is_err() {
                eprintln!("(time): cannot communicate to main bar. Abort!");
                break;
            }
        }

        // check if received an event
        extended = match event.recv_timeout(Duration::from_secs(PERIOD)) {
            Ok(_) => !extended,
            Err(_) => extended
        };
        
    }
}
