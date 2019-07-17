mod block;
mod bar;
mod event;

use block::element::{Element, ElementFormat};
use bar::Bar;
use event::{EventSystem, Event};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use block::time::time_loop;
use block::battery::battery_loop;
use block::network::network_loop;
use block::traffic::traffic_loop;
use block::brightness::brightness_loop;
use block::audio::audio_loop;
use block::mail::mail_loop;
use block::memory::memory_loop;
use block::cpu::cpu_loop;
use block::weather::weather_loop;

fn main() {
    // setup bar
    let (tx, rx) = channel::<i32>();
    let mut bar = Bar::new(rx);

    // setup event system
    let mut event_system = EventSystem::new();

    // setup traffic block
    let traffic_elem = Arc::new(Mutex::new(Element::new("traffic", "traffic", String::new(), ElementFormat::Normal)));
    let traffic_tx = tx.clone();
    bar.add(traffic_elem.clone());
    thread::Builder::new().name(String::from("traffic"))
        .spawn(move || traffic_loop(traffic_elem.clone(), traffic_tx)).unwrap();

    // setup weather block
    let weather_elem = Arc::new(Mutex::new(Element::new("weather", "weather", String::new(), ElementFormat::Normal)));
    let weather_tx = tx.clone();
    let (weather_event_tx, weather_event_rx) = channel::<Event>();
    event_system.add("weather", weather_event_tx);
    bar.add(weather_elem.clone());
    thread::Builder::new().name(String::from("weather"))
        .spawn(move || weather_loop(weather_elem.clone(), weather_tx, weather_event_rx)).unwrap();

    // setup cpu block
    let cpu_elem = Arc::new(Mutex::new(Element::new("cpu", "cpu", String::new(), ElementFormat::Normal)));
    let cpu_tx = tx.clone();
    let (cpu_event_tx, cpu_event_rx) = channel::<Event>();
    event_system.add("cpu", cpu_event_tx);
    bar.add(cpu_elem.clone());
    thread::Builder::new().name(String::from("cpu"))
        .spawn(move || cpu_loop(cpu_elem.clone(), cpu_tx, cpu_event_rx)).unwrap();

    // setup memory block
    let memory_elem = Arc::new(Mutex::new(Element::new("memory", "memory", String::new(), ElementFormat::Normal)));
    let memory_tx = tx.clone();
    bar.add(memory_elem.clone());
    thread::Builder::new().name(String::from("memory"))
        .spawn(move || memory_loop(memory_elem.clone(), memory_tx)).unwrap();

    // setup mail block
    let mail_elem = Arc::new(Mutex::new(Element::new("mail", "mail", String::new(), ElementFormat::Normal)));
    let mail_tx = tx.clone();
    let (mail_event_tx, mail_event_rx) = channel::<Event>();
    event_system.add("mail", mail_event_tx);
    bar.add(mail_elem.clone());
    thread::Builder::new().name(String::from("mail"))
        .spawn(move || mail_loop(mail_elem.clone(), mail_tx, mail_event_rx)).unwrap();

    // setup audio block
    let audio_elem = Arc::new(Mutex::new(Element::new("audio", "audio", String::new(), ElementFormat::Normal)));
    let audio_tx = tx.clone();
    bar.add(audio_elem.clone());
    thread::Builder::new().name(String::from("audio"))
        .spawn(move || audio_loop(audio_elem.clone(), audio_tx)).unwrap();

    // setup brightness block
    let brightness_elem = Arc::new(Mutex::new(Element::new("brightness", "brightness", String::new(), ElementFormat::Normal)));
    let brightness_tx = tx.clone();
    let (brightness_event_tx, brightness_event_rx) = channel::<Event>();
    event_system.add("brightness", brightness_event_tx);
    bar.add(brightness_elem.clone());
    thread::Builder::new().name(String::from("brightness"))
        .spawn(move || brightness_loop(brightness_elem.clone(), brightness_tx, brightness_event_rx)).unwrap();

    // setup network block
    let network_elem = Arc::new(Mutex::new(Element::new("network", "network", String::new(), ElementFormat::Normal)));
    let network_tx = tx.clone();
    bar.add(network_elem.clone());
    thread::Builder::new().name(String::from("network"))
        .spawn(move || network_loop(network_elem.clone(), network_tx)).unwrap();

    // setup battery block
    let battery_elem = Arc::new(Mutex::new(Element::new("battery", "battery", String::new(), ElementFormat::Normal)));
    let battery_tx = tx.clone();
    bar.add(battery_elem.clone());
    thread::Builder::new().name(String::from("battery"))
        .spawn(move || battery_loop(battery_elem.clone(), battery_tx)).unwrap();

    // setup time block
    let time_elem = Arc::new(Mutex::new(Element::new("time", "time", String::new(), ElementFormat::Normal)));
    let time_tx = tx.clone();
    let (time_event_tx, time_event_rx) = channel::<Event>();
    event_system.add("time", time_event_tx);
    bar.add(time_elem.clone());
    thread::Builder::new().name(String::from("time"))
        .spawn(move || time_loop(time_elem.clone(), time_tx, time_event_rx)).unwrap();

    // start the bar (main loop)
    let bar_thread = thread::Builder::new().name(String::from("bar"))
        .spawn(move || bar.run()).unwrap();

    // do events
    let event_thread = thread::Builder::new().name(String::from("event"))
        .spawn(move || event_system.run()).unwrap();

    // keep main thread alive
    event_thread.join().unwrap();
    bar_thread.join().unwrap();

}
