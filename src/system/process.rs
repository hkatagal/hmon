use sysinfo::{Pid, ProcessesToUpdate, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSortKey {
    Pid,
    Name,
    Cpu,
    Memory,
}

#[derive(Debug, Clone, PartialEq)]
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

        let procs: Vec<ProcInfo> = sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcInfo {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
                status: format!("{:?}", proc.status()),
            })
            .collect();

        Self::filter_and_sort(procs, sort_key, &query)
    }

    pub fn filter_and_sort(
        procs: Vec<ProcInfo>,
        sort_key: ProcessSortKey,
        query: &str,
    ) -> Vec<ProcInfo> {
        let query_lower = query.to_lowercase();

        let mut filtered: Vec<ProcInfo> = procs
            .into_iter()
            .filter(|p| {
                if query_lower.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&query_lower)
                        || p.pid.to_string().contains(&query_lower)
                }
            })
            .collect();

        match sort_key {
            ProcessSortKey::Cpu => filtered.sort_by(|a, b| {
                b.cpu_usage
                    .partial_cmp(&a.cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            ProcessSortKey::Memory => filtered.sort_by_key(|b| std::cmp::Reverse(b.memory_bytes)),
            ProcessSortKey::Pid => filtered.sort_by_key(|a| a.pid),
            ProcessSortKey::Name => filtered.sort_by_key(|a| a.name.to_lowercase()),
        }

        filtered
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_processes() -> Vec<ProcInfo> {
        vec![
            ProcInfo {
                pid: 100,
                name: "zsh".into(),
                cpu_usage: 5.0,
                memory_bytes: 10_000,
                status: "Run".into(),
            },
            ProcInfo {
                pid: 20,
                name: "cargo".into(),
                cpu_usage: 85.0,
                memory_bytes: 500_000,
                status: "Run".into(),
            },
            ProcInfo {
                pid: 300,
                name: "hmon".into(),
                cpu_usage: 12.0,
                memory_bytes: 50_000,
                status: "Run".into(),
            },
        ]
    }

    #[test]
    fn test_filter_by_name() {
        let procs = mock_processes();
        let res = ProcessManager::filter_and_sort(procs, ProcessSortKey::Pid, "cargo");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "cargo");
    }

    #[test]
    fn test_filter_by_pid() {
        let procs = mock_processes();
        let res = ProcessManager::filter_and_sort(procs, ProcessSortKey::Pid, "300");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].pid, 300);
    }

    #[test]
    fn test_sort_by_cpu_descending() {
        let procs = mock_processes();
        let res = ProcessManager::filter_and_sort(procs, ProcessSortKey::Cpu, "");
        assert_eq!(res[0].name, "cargo"); // 85.0%
        assert_eq!(res[1].name, "hmon"); // 12.0%
        assert_eq!(res[2].name, "zsh"); // 5.0%
    }

    #[test]
    fn test_sort_by_memory_descending() {
        let procs = mock_processes();
        let res = ProcessManager::filter_and_sort(procs, ProcessSortKey::Memory, "");
        assert_eq!(res[0].name, "cargo"); // 500k
        assert_eq!(res[1].name, "hmon"); // 50k
        assert_eq!(res[2].name, "zsh"); // 10k
    }

    #[test]
    fn test_sort_by_pid_ascending() {
        let procs = mock_processes();
        let res = ProcessManager::filter_and_sort(procs, ProcessSortKey::Pid, "");
        assert_eq!(res[0].pid, 20);
        assert_eq!(res[1].pid, 100);
        assert_eq!(res[2].pid, 300);
    }
}
