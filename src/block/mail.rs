use super::element::{Element, ElementFormat};
use crate::event::Event;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
// use std::process::Command;

use std::fs::read_dir;
use std::path::Path;

const ICON: char = '\u{f6ee}';
const BLUEWIN_DIR: &str = "/home/tibor/Mail/bluewin/INBOX/new";
const ETH_DIR: &str = "/home/tibor/Mail/ETH/INBOX/new";

// const MAIL_EMACS_CMD: &str = "\'(mu4e-headers-search-bookmark \"flag:unread and not flag:trashed\")\'";

pub fn mail_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>, event: Receiver<Event>) {
    let bluewin_dir = Path::new(BLUEWIN_DIR);
    let eth_dir = Path::new(ETH_DIR);
    loop {
        let mut new_text = String::with_capacity(6);
        let unread = (read_dir(&bluewin_dir).unwrap().count() + read_dir(&eth_dir).unwrap().count()) as u32;
        new_text.push(ICON);
        new_text.push(' ');
        new_text.push_str(&format!("{}", unread));
        
        // get mutex
        let mut updated = false;
        if let Ok(mut e) = elem.lock() {
            // check if text has changed
            if e.text != new_text {
                e.set_text(new_text);
                if unread > 0 { e.set_format(ElementFormat::Warning); }
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
        match event.recv_timeout(Duration::new(0, 100_000_000)) {
            Ok(_) => {
                // launch emacs client for mail and read unread
                //Command::new("emacsclient").arg("-ce").arg(MAIL_EMACS_CMD).spawn().unwrap();
            },
            Err(_) => { }
        }
    }
}
