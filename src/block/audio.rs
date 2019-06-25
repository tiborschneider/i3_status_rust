use super::element::{Element, ElementFormat};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use alsa::mixer::{Mixer, Selem, SelemChannelId} ;

const ICON_MUTE: char = '\u{fc5d}';
const ICON_NORM: char = '\u{fa7d}';

pub fn audio_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>) {

    let (min_val, max_val) = (0 as i64, 74 as i64);
    let channel = SelemChannelId::FrontCenter;

    loop {
        let mixer = Mixer::new("hw:0", false).unwrap();
        let master = mixer.iter().map(|x| Selem::new(x).unwrap())
                                .filter(|x| x.get_id().get_name().unwrap() == "Master")
                                .next().unwrap();
        let perc = scale_to_perc(master.get_playback_volume(channel).unwrap(), min_val, max_val);
        let active = master.get_playback_switch(channel) == Ok(1);
        let new_text = match active {
            true => format!("{} {:.0}%", ICON_NORM, perc),
            false => format!("{}", ICON_MUTE)
        };
        
        // get mutex
        let mut updated = false;
        if let Ok(mut e) = elem.lock() {
            // check if text has changed
            if e.text != new_text {
                e.set_text(new_text);
                match active {
                    true  => e.set_format(ElementFormat::Normal),
                    false => e.set_format(ElementFormat::Info),
                }
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

        // wait for new event
        mixer.wait(None).unwrap();        
    }
}

fn scale_to_perc(cur: i64, min: i64, max: i64) -> f32 {
    100.0 * ((cur - min) as f32) / ((max - min) as f32)
}
