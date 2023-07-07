use sysinfo::{System, SystemExt, CpuExt, DiskExt};

#[derive(Debug)]
pub struct SystemInfo {
    pub mem_used        : u64, //Used memory as Byte
    pub mem_free        : u64, //Free memory as Byte
    pub mem_usage       : f32, //Percentage of used memory over total
    pub storage_used    : u64, //Used storage as Byte
    pub storage_free    : u64, //Free storage as Byte
    pub storage_usage   : f32, //Percentage of used storage over total
    pub global_cpu_usage: f32  //Percentage of usage of CPU
}

pub fn is_supported() -> bool {
    System::IS_SUPPORTED
}

pub fn fetch_info() -> SystemInfo {
    let mut sys = System::new_all();

    println!{"Fetching cpu info..."};
    //We do blocking here since cpu_usage() use differential information from past.
    //So, to fetch collect cpu usage value, we need to loop at least twice.
    //We looping 3 times for more accurate value
    let mut global_cpu_usage: f32 = 0.0;
    for _ in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        sys.refresh_cpu();
        global_cpu_usage = sys.global_cpu_info().cpu_usage();
    }

    println!{"Fetching storage and memory info..."};
    sys.refresh_all();
    let disk = &sys.disks()[0];
    let disk_used = disk.total_space() - disk.available_space();

    SystemInfo {
        mem_used: sys.used_memory(),
        mem_free: sys.total_memory() - sys.used_memory(),
        mem_usage: (sys.used_memory() as f32) * 100.0 / (sys.total_memory() as f32),
        storage_used: disk_used,
        storage_free: disk.available_space(),
        storage_usage: (disk_used as f32) * 100.0 / disk.total_space() as f32,
        global_cpu_usage: global_cpu_usage
    }
}
