use super::spec::DeviceSpec;

use std::collections::HashMap;

#[derive(Debug)]
pub struct DeviceManager {
    devices: HashMap<String, DeviceSpec>,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: HashMap::new(),
        }
    }

    pub fn print_devices(&self) {
        for device in &self.devices {
            println!("{:?}", device);
        }
    }

    pub fn get_devices(&self) -> &HashMap<String, DeviceSpec> {
        &self.devices
    }

    pub fn add_device(&mut self, device: DeviceSpec) {
        self.devices.insert(device.id.clone(), device);
    }

    pub fn remove_device(&mut self, id: &String) {
        self.devices.remove(id);
    }

    pub fn update_device(&mut self, id: &String, device: DeviceSpec) {
        self.devices.insert(id.to_string(), device);
    }

    pub fn get_device(&self, id: &String) -> Option<&DeviceSpec> {
        self.devices.get(id)
    }
}
