extern crate hidapi;

use hidapi::HidApi;

fn main() {
    let api = HidApi::new().unwrap();
    let mut vid = 0;
    let mut pid = 0;
    for device in api.device_list() {
        if let Some(name) = device.product_string() {
            if name.contains("Magic Mouse") {
                vid = device.vendor_id();
                pid = device.product_id();
            }
        }
    }

    println!("vid {:x}, pid {:x}", vid, pid);
    api.set_open_exclusive(false);
    let device = api.open(vid, pid).unwrap();

    loop {
        let mut buf = [0u8; 8];
        let res = device.read(&mut buf[..]).unwrap();
        if res == 8 {
            if buf[1] == 1 {
                println!("Left key is pressed");
            } else if buf[1] == 2 {
                println!("Right key is pressed");
            }
        }
    }
}