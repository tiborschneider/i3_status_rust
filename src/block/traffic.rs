use super::element::Element;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::thread;

use systemstat::{System, Platform};

const PERIOD: u64 = 1;
const NETIFACE: &str = "wlp4s0";
const ICON_RX: char = '\u{f019}';
const ICON_TX: char = '\u{f093}';

pub fn traffic_loop(elem: Arc<Mutex<Element>>, tx: Sender<i32>) {
    let sys = System::new();

    let stats = sys.network_stats(NETIFACE).unwrap();
    let (mut cur_rx, mut cur_tx) = (stats.rx_bytes, stats.tx_bytes);

    loop {
        // do measurement
        let stats = sys.network_stats(NETIFACE).unwrap();
        let (new_rx, new_tx) = (stats.rx_bytes, stats.tx_bytes);

        // compute difference
        let (d_rx, d_tx) = ((new_rx - cur_rx) as f32 / (1024.0 * 1024.0) / (PERIOD as f32),
                            (new_tx - cur_tx) as f32 / (1024.0 * 1024.0) / (PERIOD as f32));

        cur_rx = new_rx;
        cur_tx = new_tx;

        // get text
        let new_text = String::from(format!("{} {:.2} mB/s | {} {:.2} mB/s", ICON_RX, d_rx, ICON_TX, d_tx));
        
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

        // wait a bit
        thread::sleep(Duration::new(PERIOD, 0));
    }
}
