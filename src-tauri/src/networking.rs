use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::thread;
use std::time::Duration;

const SERVICE_TYPE: &str = "_loom-app._tcp.local.";
const SERVICE_NAME: &str = "Loom App";
const SERVICE_PORT: u16 = 8080;

pub fn start_networking_service() {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // let txt_properties = vec![("role", "master")];

    // Register our service
    let service_info = ServiceInfo::new(
        SERVICE_TYPE,
        SERVICE_NAME,
        "loom.local.",
        "",
        SERVICE_PORT,
        None,
        // Some(&txt_properties)
    ).expect("Failed to create service info");
    
    mdns.register(service_info).expect("Failed to register service");

    // Browse for other instances
    let receiver = mdns.browse(SERVICE_TYPE).expect("Failed to browse");

    thread::spawn(move || {
        loop {
            if let Ok(event) = receiver.recv() {
                println!("Service discovery event: {:?}", event);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
