use sysinfo::System;

#[derive(Debug, Clone)]
pub struct CpuCoreInfo {
    pub name: String,
    pub usage: f32,
    pub frequency_mhz: u64,
}

#[derive(Debug, Clone)]
pub struct CpuMetrics {
    pub global_usage: f32,
    pub brand: String,
    pub cores: Vec<CpuCoreInfo>,
}

impl CpuMetrics {
    pub fn collect(sys: &System) -> Self {
        let global_usage = sys.global_cpu_usage();
        let cpus = sys.cpus();
        let brand = cpus
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Generic CPU".into());

        let cores = cpus
            .iter()
            .enumerate()
            .map(|(i, cpu)| CpuCoreInfo {
                name: format!("Core {}", i),
                usage: cpu.cpu_usage(),
                frequency_mhz: cpu.frequency(),
            })
            .collect();

        Self {
            global_usage,
            brand,
            cores,
        }
    }
}
