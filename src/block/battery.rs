use super::element::Element;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use battery;
use battery::units::time::{minute, hour};
use battery::units::ratio::percent;

static ICON_CHARGE: char = '\u{f583}';
static ICON_RAMP: [char; 11] = ['\u{f58d}', '\u{f579}', '\u{f57a}', '\u{f57b}', '\u{f57c}',
                                '\u{f57d}', '\u{f57e}', '\u{f57f}', '\u{f580}', '\u{f581}',
                                '\u{f578}'];

pub fn battery_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>) {

    // setup manager and battery
    let manager = battery::Manager::new().unwrap();
    let mut battery = match manager.batteries().unwrap().next() {
        Some(Ok(battery)) => battery,
        Some(Err(_)) => { panic!("Unable to access battery information"); },
        None => { panic!("Unable to find any batteries"); }
    };

    loop {
        let mut updated = false;

        // update battery
        manager.refresh(&mut battery).unwrap();

        // get battery state
        let state = battery.state();
        let perc: f32 = battery.state_of_charge().get::<percent>();
        let time: battery::units::Time = match state {
            battery::State::Charging => battery.time_to_full().unwrap(),
            battery::State::Discharging => battery.time_to_empty().unwrap(),
            _ => battery::units::Time::new::<minute>(0.0)
        };
        
        // generate new text
        let mut new_text: String = String::with_capacity(16);
        match state {
            battery::State::Charging => { new_text.push(ICON_CHARGE); },
            _ => { new_text.push(ICON_RAMP[((perc) / 10.0) as usize]); }
        }
        new_text.push(' ');
        new_text.push_str(&String::from(format!("{:.0}", perc)));
        new_text.push('%');
        match state {
            battery::State::Full => {},
            _ => {
                let mut time_val: f32 = time.get::<minute>();
                let mut unit = 'm';
                if time_val > 60.0 {
                    time_val = time.get::<hour>();
                    unit = 'h';
                }
                new_text.push_str(&format!(" ({:.1}{})", time_val, unit));
            }
        }

        // get mutex
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
        thread::sleep(Duration::new(5, 0));
    }
}
