use libpulse_binding::{sample, stream};
use libpulse_simple_binding::Simple;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread;

const SAMPLING_RATE: u32 = 48000;
const FRAME_SIZE: usize = 2880;

fn main() {
    let clients: Arc<RwLock<Vec<SocketAddr>>> = Arc::new(RwLock::new(Vec::new()));
    let clients_server = clients.clone();
    let clients_main = clients.clone();

    thread::spawn(move || start_server(clients_server));

    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();

    let mut audio = [0; 2 * FRAME_SIZE];

    loop {
        grab_audio(&mut audio);
        println!("{:?}\n", audio.to_vec());
        for client in clients_main.read().unwrap().iter() {
            socket.send_to(&audio, client).unwrap();
        }
    }
}

fn start_server(clients: Arc<RwLock<Vec<SocketAddr>>>) {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    let mut buf = [0; 100];

    loop {
        let (_size, addr) = socket.recv_from(&mut buf).unwrap();
        if !clients.read().unwrap().contains(&addr) {
            clients.write().unwrap().push(addr);
        }
    }
}

fn grab_audio(mut buffer: &mut [u8]) {
    let spec = sample::Spec {
        format: sample::SAMPLE_S16NE,
        channels: 2,
        rate: SAMPLING_RATE,
    };

    let s = Simple::new(
        None,
        "onkyo",
        stream::Direction::Record,
        None,
        "Stream",
        &spec,
        None,
        None,
    )
    .unwrap();

    s.read(&mut buffer).unwrap();
}
