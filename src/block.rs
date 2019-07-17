#[allow(dead_code)]
pub mod element;
pub mod time;
pub mod battery;
pub mod network;
pub mod brightness;
pub mod audio;
pub mod mail;
pub mod memory;
pub mod cpu;
pub mod traffic;
pub mod weather;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

use element::Element;

pub trait Block {
    fn work(&self, event_tx: Sender<i32>, elem: Arc<Mutex<Element>>); 
    fn get_id(&self) -> i32;
    fn get_id_str(&self) -> &str;
}
