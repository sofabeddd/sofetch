
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

fn main() {
    
    let mut sys = System::new_all();
    sys.refresh_all();
}
