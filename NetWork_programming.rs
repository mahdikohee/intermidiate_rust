//using a famous rust library for network analysis ......lib.name=pcap 
//example for to capture one packet simply 
fn main() {
    // get the default Device
    let device = pcap::Device::lookup()
        .expect("device lookup failed")
        .expect("no device available");
    println!("Using device {}", device.name);

    // Setup Capture
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    // get a packet and print its bytes
    println!("{:?}", cap.next_packet());
}

//another example to show output in a large scale
use pcap::Device;

fn main() {
    match Device::list() {
        Ok(devices) => {
            println!("🔍 Found {} device(s):\n", devices.len());
            for (i, device) in devices.iter().enumerate() {
                println!("📡 Device #{}", i + 1);
                println!("  Name       : {}", device.name);

                // Optional description
                if let Some(desc) = &device.desc {
                    println!("  Description: {}", desc);
                } else {
                    println!("  Description: (None)");
                }

                // Addresses
                if !device.addresses.is_empty() {
                    println!("  Addresses  :");
                    for addr in &device.addresses {
                        println!("    - IP      : {:?}", addr.addr);
                        if let Some(mask) = addr.netmask {
                            println!("      Netmask : {:?}", mask);
                        }
                        if let Some(bcast) = addr.broadcast_addr {
                            println!("      Broadcast: {:?}", bcast);
                        }
                        if let Some(dst) = addr.dst_addr {
                            println!("      Destination: {:?}", dst);
                        }
                    }
                } else {
                    println!("  Addresses  : (None)");
                }

                println!("  Flags      : {:?}", device.flags);
                println!(); // blank line between devices
            }
        }
        Err(e) => {
            eprintln!("❌ Error occurred: {:?}", e);
        }
    }
}

