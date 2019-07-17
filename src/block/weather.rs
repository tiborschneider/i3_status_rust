use super::element::Element;
use crate::event::Event;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};

use std::time::Duration;
use curl::easy::Easy;

const PERIOD: u64 = 300; // once every minute
const ICON_C: char = '\u{fa03}';
const ICON_RAMP: [char; 5] = ['\u{f2cb}', '\u{f2ca}', '\u{f2c9}', '\u{f2c8}', '\u{f2c7}'];
const RAMP_COLD: i32 = 0;
const RAMP_WARM: i32 = 30;
const URL: &str = "http://www.meteocentrale.ch/de/europe/schweiz/wetter-zuerich-fluntern-sma/details/S066600/";
//const URL: &str = "http://www.meteocentrale.ch/de/europa/schweiz/wetter-zuerich-kaserne/details/S069090/";

enum SearchState {
    Page,
    Content,
    LowerContent,
    Summary,
    Temperature
}

pub fn weather_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>, event: Receiver<Event>) {
    loop {
        let new_text = match get_temperature() {
            Ok(Some(t)) => format!("{} {}{}", ICON_RAMP[get_ramp_index(t)], t, ICON_C),
            Ok(None) => format!(""),
            Err(e) => {eprintln!("{}", e); format!("error") }
        };

        let mut updated = false;
        //get mutex
        if let Ok(mut e) = elem.lock() {
            // check if text has changed
            if e.text != new_text {
                e.set_text(new_text);
                updated = true;
            }
        } //release mutex

        if updated {
            if tx.send(1).is_err() {
                eprintln!("(time): cannot communicate to main bar. Abort!");
                break;
            }
        }

        // check if received an event
        match event.recv_timeout(Duration::from_secs(PERIOD)) {
            Ok(_) => {},
            Err(_) => {} 
        };
        
    }
}

fn get_ramp_index(temperature: i32) -> usize {
    if temperature <= RAMP_COLD {
        0 as usize
    } else if temperature >= RAMP_WARM {
        4 as usize
    } else {
        (5 * (temperature - RAMP_COLD) / (RAMP_WARM - RAMP_COLD)) as usize
    }
}

fn get_temperature() -> Result<Option<i32>, String> {
    let mut buf: Vec<u8> = Vec::new();
    let mut easy = Easy::new();
    easy.url(URL).unwrap();

    // do the transfer
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        match transfer.perform() {
            Ok(_) => {},
            Err(_) => {return Ok(None);}
        };
    }

    let dst = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(_) => return Err(String::from("Temperature: Received non-utf8 text!")),
    };

    // search for the correct line
    let mut search_state: SearchState = SearchState::Page;
    for line in dst.split("\n") {
        let new_state = match search_state {
            SearchState::Page => {
                if line.contains("<div id=\"page\">") {
                    SearchState::Content
                } else {
                    SearchState::Page
                }
            },
            SearchState::Content => {
                if line.contains("<div id=\"content\">") {
                    SearchState::LowerContent
                } else {
                    SearchState::Content
                }
            },
            SearchState::LowerContent => {
                if line.contains("<div id=\"lower-content\">") {
                    SearchState::Summary
                } else {
                    SearchState::LowerContent
                }
            },
            SearchState::Summary => {
                if line.contains("<div id=\"weather-detail-summary\">") {
                    SearchState::Temperature
                } else {
                    SearchState::Summary
                }
            },
            SearchState::Temperature => {
                if line.contains("<div class=\"column-4\">") {
                    let tmp: Vec<_> = line.split(">").collect();
                    let tmp: Vec<_> = tmp[1].split("<").collect();
                    match tmp[0].parse::<i32>() {
                        Ok(temperature) => return Ok(Some(temperature)),
                        Err(_) => return Err(String::from("Temperature: cannot parse webpage (reading out value)"))
                    };
                }
                SearchState::Temperature
            }
        };
        search_state = new_state;
    }
    Err(String::from("Error: cannot parse webpage"))
}
