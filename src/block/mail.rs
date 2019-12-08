use super::element::{Element, ElementFormat};
use crate::event::Event;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

use maildir;

const ICON: char = '\u{f6ee}';
const BLUEWIN_DIR: &str = "/home/tibor/Mail/bluewin/INBOX";
const ETH_DIR: &str = "/home/tibor/Mail/ETH/INBOX";

pub fn mail_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>, event: Receiver<Event>) {
    loop {
        let mut new_text = String::with_capacity(6);
        let unread = count_unread(BLUEWIN_DIR) + count_unread(ETH_DIR);
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
                eprintln!("(mail): cannot communicate to main bar. Abort!");
                break;
            }
        }

        // sleep for one second or until an interrupt occurrs
        event.recv_timeout(Duration::new(0, 100_000_000)).unwrap();
    }
}

fn count_unread(maildir_path: &str) -> usize {
    let _maildir = maildir::Maildir::from(maildir_path);
    _maildir.count_new()
}
