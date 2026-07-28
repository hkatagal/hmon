pub mod cpu;
pub mod memory;
pub mod process;
pub mod disk;

pub use cpu::CpuMetrics;
pub use memory::MemoryMetrics;
pub use process::{ProcessManager, ProcessSortKey, ProcInfo};
pub use disk::DiskInfo;

use sysinfo::System;

pub struct SysMetrics {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub processes: Vec<ProcInfo>,
    pub disks: Vec<DiskInfo>,
    pub host_name: String,
    pub os_name: String,
    pub os_version: String,
    pub uptime_secs: u64,
}

impl SysMetrics {
    pub fn new() -> Self {
        Self {
            cpu: CpuMetrics { global_usage: 0.0, brand: "".into(), cores: vec![] },
            memory: MemoryMetrics { total_mem_bytes: 0, used_mem_bytes: 0, free_mem_bytes: 0, total_swap_bytes: 0, used_swap_bytes: 0 },
            processes: vec![],
            disks: vec![],
            host_name: System::host_name().unwrap_or_else(|| "Unknown".into()),
            os_name: System::name().unwrap_or_else(|| "OS".into()),
            os_version: System::os_version().unwrap_or_default(),
            uptime_secs: System::uptime(),
        }
    }

    pub fn refresh(&mut self, sys: &mut System, sort_key: ProcessSortKey, filter: &str) {
        sys.refresh_all();
        self.cpu = CpuMetrics::collect(sys);
        self.memory = MemoryMetrics::collect(sys);
        self.processes = ProcessManager::collect(sys, sort_key, filter);
        self.disks = DiskInfo::collect_all();
        self.uptime_secs = System::uptime();
    }
}
