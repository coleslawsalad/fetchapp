
use sysinfo::{, Disks, System,};
//writes the info into output
fn main() {
    check_sys_specs();
}
//checks system specs and os version
fn check_sys_specs() {
    let mut sys = System::new_all();
    sys.refresh_all();
    //OS info 
    let sys_name = System::name();
    let sys_kernel = System::kernel_version();
    let os_version = System::os_version();
    let host_name = System::host_name();
    //Memory info
    let total_ram = sys.total_memory()/1048576;
    let used_ram = sys.used_memory() /1048576;
    //disk info
    let disk = Disks::new_with_refreshed_list();
    
    //outputs os info
    println!("OS info: \n OS name: {:?}\n Kernel: {:?}\n OS version: {:?}\n Hostname:{:?}",sys_name,sys_kernel,os_version,host_name);
    //outputs ram info
    println!("RAM info \n Used RAM: {} MB out of: {} MB",used_ram,total_ram);
    //outputs disk info 
    let mut disk_no = 1;
    println!("Disk info:");
    for disk in &disk {
        //i have no idea whats happening here but it works
        let used_space = disk.total_space() - disk.available_space();
        println!{"Disk number {}: {:?} {:?} \n Space: {:?} GB out of {} GB",disk_no,disk.name(),disk.kind(),used_space /1073741824,disk.total_space()/1073741824};
        disk_no += 1;
   } 
    
}
