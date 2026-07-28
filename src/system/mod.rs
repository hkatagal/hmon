pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod process;

pub use cpu::CpuMetrics;
pub use disk::DiskInfo;
pub use memory::MemoryMetrics;
pub use network::NetworkMetrics;
pub use process::{ProcInfo, ProcessManager, ProcessSortKey};

use std::collections::VecDeque;
use sysinfo::System;

pub struct SysMetrics {
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub processes: Vec<ProcInfo>,
    pub disks: Vec<DiskInfo>,
    pub network: NetworkMetrics,
    pub host_name: String,
    pub os_name: String,
    pub os_version: String,
    pub uptime_secs: u64,
    pub cpu_history: VecDeque<u64>,
    pub ram_history: VecDeque<u64>,
}

impl SysMetrics {
    pub fn new() -> Self {
        Self {
            cpu: CpuMetrics {
                global_usage: 0.0,
                brand: "".into(),
                cores: vec![],
            },
            memory: MemoryMetrics {
                total_mem_bytes: 0,
                used_mem_bytes: 0,
                free_mem_bytes: 0,
                total_swap_bytes: 0,
                used_swap_bytes: 0,
            },
            processes: vec![],
            disks: vec![],
            network: NetworkMetrics {
                interfaces: vec![],
                total_rx_rate_sec: 0,
                total_tx_rate_sec: 0,
            },
            host_name: System::host_name().unwrap_or_else(|| "Unknown".into()),
            os_name: System::name().unwrap_or_else(|| "OS".into()),
            os_version: System::os_version().unwrap_or_default(),
            uptime_secs: System::uptime(),
            cpu_history: VecDeque::with_capacity(60),
            ram_history: VecDeque::with_capacity(60),
        }
    }

    pub fn refresh(&mut self, sys: &mut System, sort_key: ProcessSortKey, filter: &str) {
        sys.refresh_all();
        self.cpu = CpuMetrics::collect(sys);
        self.memory = MemoryMetrics::collect(sys);
        self.processes = ProcessManager::collect(sys, sort_key, filter);
        self.disks = DiskInfo::collect_all();
        self.network = NetworkMetrics::collect();
        self.uptime_secs = System::uptime();

        // Push CPU & RAM history ring buffers (max 60 points)
        if self.cpu_history.len() >= 60 {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(self.cpu.global_usage as u64);

        if self.ram_history.len() >= 60 {
            self.ram_history.pop_front();
        }
        self.ram_history
            .push_back(self.memory.ram_usage_percent() as u64);
    }
}
