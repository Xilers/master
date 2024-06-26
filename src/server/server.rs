use crate::device::manager::DeviceManager;
use crate::device::spec::DeviceSpec;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const HEADER_SIZE: usize = 5;

#[derive(Debug)]
enum Request {
    Connect,   // Get spec of device from client
    ListCheck, // Give Client DeviceSpec list
}

impl Request {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Request::Connect),
            1 => Some(Request::ListCheck),
            _ => None,
        }
    }
}

pub struct Server {
    address: &'static str,
    port: u16,
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl Server {
    pub fn new(address: &'static str, port: u16) -> Self {
        Server {
            address,
            port,
            device_manager: Arc::new(Mutex::new(DeviceManager::new())),
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", &self.address, &self.port))?;

        for stream in listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    let device_manager = Arc::clone(&self.device_manager);
                    thread::spawn(move || {
                        Self::handle_client(tcp_stream, device_manager);
                    });
                }

                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_client(mut stream: TcpStream, device_manager: Arc<Mutex<DeviceManager>>) {
        let mut buffer = [0; 1024];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => return,
                Ok(n) => {
                    if n < HEADER_SIZE {
                        eprintln!("Invalid request");
                        return;
                    }

                    let request_type = buffer[0];
                    let data_length = u32::from_be_bytes(buffer[1..5].try_into().unwrap()) as usize;

                    if n < HEADER_SIZE + data_length {
                        eprintln!("Incomplete request");
                        return;
                    }

                    let data = &buffer[HEADER_SIZE..HEADER_SIZE + data_length];

                    match Request::from_u8(request_type) {
                        Some(Request::Connect) => {
                            Self::device_connect(&data, device_manager.clone())
                        }
                        Some(Request::ListCheck) => {}
                        None => {
                            eprintln!("Unknown request type");
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Failed to read from stream");
                    return;
                }
            }
        }
    }

    fn device_connect(data: &[u8], device_manager: Arc<Mutex<DeviceManager>>) {
        let device_spec: DeviceSpec = serde_json::from_slice(data).unwrap();

        let mut dm = device_manager.lock().unwrap();
        dm.add_device(device_spec);
    }
}
