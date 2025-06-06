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


///anothee example to capture
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

    let mut count = 0;
    cap.for_each(None, |packet| {
        println!("Got {:?}", packet.header);
        count += 1;
        if count > 100 {
            panic!("ow");
        }
    })
    .unwrap();
}




//best practise of oop with  networking 

use pcap::{Capture, Device, PacketHeader};
use std::error::Error;

// PacketIter struct, যা প্রতিবার প্যাকেট পড়ে (PacketHeader, Vec<u8>) রিটার্ন করবে
struct PacketIter {
    cap: Capture<pcap::Active>,
}

impl Iterator for PacketIter {
    // আমরা (PacketHeader, Vec<u8>) রিটার্ন করছি, যাতে হেডার ও ডেটা দুটোই মেলে
    type Item = (PacketHeader, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        // cap.next_packet() রিটার্ন করে Result<Packet, Error>
        match self.cap.next_packet() {
            Ok(packet) => {
                // packet.header এখানে &PacketHeader, তাই * দিয়ে কপি করে নিচ্ছি
                let header = *packet.header;          // PacketHeader (Copy-able)
                let data = packet.data.to_vec();      // &[u8] → Vec<u8>
                Some((header, data))
            }
            Err(_) => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ১) সিস্টেম থেকে একটা নেটওয়ার্ক ডিভাইস খুঁজে বের করা
    let device = Device::lookup()?
        .ok_or("কোনো নেটওয়ার্ক ডিভাইস পাওয়া যায়নি!")?;
    println!("Using device: {}", device.name);

    // ২) ডিভাইস থেকে ক্যাপচার শুরু
    let cap = Capture::from_device(device)?
        .immediate_mode(true)
        .promisc(true)
        .open()?;

    // ৩) PacketIter তৈরি
    let packet_iter = PacketIter { cap };

    // ৪) for-লুপে প্রতিটি প্যাকেটকে (header, data) টুপল হিসেবে প্রিন্ট করা
    for (header, data) in packet_iter {
        println!("--- নতুন প্যাকেট ---");
        println!("Timestamp: {}.{}", header.ts.tv_sec, header.ts.tv_usec);
        println!("Captured length: {}", header.caplen);
        println!("Original length: {}", header.len);
        println!("Raw data ({} bytes): {:02X?}", data.len(), data);
    }

    Ok(())
}

