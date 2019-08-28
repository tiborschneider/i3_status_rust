use super::element::Element;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use network_manager::{Connection, NetworkManager};

pub fn network_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>) {
    let manager = NetworkManager::new();
    loop {
        let mut updated = false;

        // get ssid
        let connections: Vec<Connection> = match manager.get_active_connections() {
            Ok(c) => c,
            Err(_) => Vec::new(),
        };
        let mut new_text: String = String::new();
        if connections.is_empty() {
            new_text.push_str("offline");
        } else {
            new_text.push_str(&connections[0].settings().id);
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
