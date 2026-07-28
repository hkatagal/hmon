use sysinfo::{Pid, System, ProcessesToUpdate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSortKey {
    Pid,
    Name,
    Cpu,
    Memory,
}

#[derive(Debug, Clone)]
pub struct ProcInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub status: String,
}

pub struct ProcessManager;

impl ProcessManager {
    pub fn collect(sys: &System, sort_key: ProcessSortKey, filter_query: &str) -> Vec<ProcInfo> {
        let query = filter_query.to_lowercase();

        let mut procs: Vec<ProcInfo> = sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcInfo {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
                status: format!("{:?}", proc.status()),
            })
            .filter(|p| {
                if query.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&query) || p.pid.to_string().contains(&query)
                }
            })
            .collect();

        // Sort process list
        match sort_key {
            ProcessSortKey::Cpu => procs.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            ProcessSortKey::Memory => procs.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes)),
            ProcessSortKey::Pid => procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
            ProcessSortKey::Name => procs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        }

        procs
    }

    pub fn kill_process(pid: u32) -> bool {
        let sys_pid = Pid::from_u32(pid);
        let mut sys = System::new();
        sys.refresh_processes(ProcessesToUpdate::Some(&[sys_pid]), true);
        if let Some(process) = sys.process(sys_pid) {
            return process.kill();
        }
        false
    }
}
