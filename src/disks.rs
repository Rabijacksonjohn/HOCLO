use sysinfo::{Disks,System};

pub fn disk_space(){
    let mut sys=System::new_all();
    sys.refresh_all();

    println!("----------------List of Disk-----------------");
    let disk = Disks::new_with_refreshed_list();

    for disk in &disk {
        let name=disk.name().to_string_lossy();
        let mount_point=disk.mount_point().display();
	
	//get disk space
	let total_space = disk.total_space();
    let available_space = disk.available_space();
    let used_space = total_space - available_space ;
    
    //convert to gb
    let total_gb = total_space as f64 /1024.0 /1024.0 /1024.0;
    let ave_gb = available_space as f64 /1024.0 /1024.0 /1024.0;
    let used_gb = used_space as f64 /1024.0 /1024.0 /1024.0;
    
    println!("Disk name         : {:?}",name);
    println!("Disk mount point  : {:?}",mount_point);
    println!("Disk total GB     : {:.2}",total_gb);
    println!("Disk available GB : {:.2}",ave_gb);
    println!("Disk used space   : {:.2}\n",used_gb);

    }
}
