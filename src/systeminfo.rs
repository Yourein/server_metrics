use sysinfo::{System, SystemExt, CpuExt, DiskExt};

#[derive(Debug)]
pub struct SystemInfo {
    pub mem_used        : u64,
    pub mem_free        : u64,
    pub mem_usage       : f32,
    pub storage_used    : u64,
    pub storage_free    : u64,
    pub storage_usage   : f32,
    pub global_cpu_usage: f32
}

pub fn is_supported() -> bool {
    System::IS_SUPPORTED
}

pub fn fetch_info() -> SystemInfo {
    let mut sys = System::new_all();
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
        global_cpu_usage: sys.global_cpu_info().cpu_usage()
    }
}
