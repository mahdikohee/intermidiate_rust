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

//simple packet capture and and minimally analyze
use pcap::{Capture, Device};

fn main() {
    // ১) ডিফল্ট নেটওয়ার্ক ডিভাইস খুঁজে বের করি
    let device = Device::lookup()
        .expect("⚠️ ডিভাইস লুকআপ ব্যর্থ")
        .expect("❌ কোনো ডিভাইস পাওয়া যায়নি");
    println!("📡 ব্যবহার করছি ডিভাইস: {}", device.name);

    // ২) ক্যাপচার সেটআপ: প্রমিসকিউয়াস + ইমিডিয়েট মোড + টাইমআউট
    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)       // ✅ সব প্যাকেট ধরতে চাই
        .immediate(true)     // ⚡ দেরি ছাড়াই সঙ্গে সঙ্গে দাও
        .timeout(1000)       // ⏱ ১ সেকেন্ড টাইমআউট
        .open()
        .unwrap();

    // ৩) ১০টি প্যাকেট পড়ি এবং তথ্য প্রিন্ট করি
    for i in 0..10 {
        match cap.next_packet() {
            Ok(packet) => {
                println!(
                    "📦 প্যাকেট {}: আকার = {} বাইট, টাইম = {:?}",
                    i + 1,
                    packet.data.len(),
                    packet.header.ts
                );
            }
            Err(e) => {
                println!("❌ প্যাকেট {} পড়তে সমস্যা: {}", i + 1, e);
            }
        }
    }

    // ৪) ক্যাপচারের পরিসংখ্যান দেখি
    match cap.stats() {
        Ok(stats) => {
            println!(
                "📊 স্ট্যাটস — মোট: {}, বাদ পড়েছে: {}, ইন্টারফেস বাদ: {}",
                stats.received, stats.dropped, stats.if_dropped
            );
        }
        Err(e) => {
            println!("❌ স্ট্যাটস নিতে সমস্যা: {}", e);
        }
    }
}

//another example to capture packet in raw way 
use pcap::{Capture, Device};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // ডিফল্ট ডিভাইস খুঁজে বের করি
    let device = Device::lookup()?.ok_or("No device available")?;

    println!("Using device: {}", device.name);

    // ক্যাপচার ওপেন করি
    let mut cap = Capture::from_device(device)?
        .immediate_mode(true)
        .open()?;

    // প্যাকেট পড়া শুরু
    loop {
        match cap.next_packet() {
            Ok(packet) => {
                println!("{:?}", packet);
            }
            Err(pcap::Error::NoMorePackets) => {
                // আর প্যাকেট নেই (এইটা সাধারণত dump file-এর জন্য হয়)
                break;
            }
            Err(err) => {
                // অন্য কোনো সমস্যা হলে
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}
