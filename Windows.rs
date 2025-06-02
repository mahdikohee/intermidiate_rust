//get your local time by rust .....not crossplatform but only for windows ...................A local time project made by win-api 
use std::mem::zeroed;
use winapi::um::sysinfoapi::GetLocalTime;
use winapi::um::minwinbase::SYSTEMTIME;

fn main() {
    unsafe {
        let mut system_time: SYSTEMTIME = zeroed();
        GetLocalTime(&mut system_time);

        println!(
            "Current Local Time: {:02}:{:02}:{:02}",
            system_time.wHour, system_time.wMinute, system_time.wSecond
        );
    }
}
