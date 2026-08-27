use sysinfo::{Disks,System};

pub fn disk_space(){
    let mut sys=System::new_all();
    sys.refresh_all();

    println!("----------------List of Disk-----------------");
    let disk = Disks::new_with_refreshed_list();
    for disk in &disk {
        println!("{:?}",disk);
    }
}
