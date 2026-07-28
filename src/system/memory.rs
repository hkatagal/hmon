use sysinfo::System;

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub total_mem_bytes: u64,
    pub used_mem_bytes: u64,
    pub free_mem_bytes: u64,
    pub total_swap_bytes: u64,
    pub used_swap_bytes: u64,
}

impl MemoryMetrics {
    pub fn collect(sys: &System) -> Self {
        let total_mem_bytes = sys.total_memory();
        let used_mem_bytes = sys.used_memory();
        let free_mem_bytes = sys.free_memory();
        let total_swap_bytes = sys.total_swap();
        let used_swap_bytes = sys.used_swap();

        Self {
            total_mem_bytes,
            used_mem_bytes,
            free_mem_bytes,
            total_swap_bytes,
            used_swap_bytes,
        }
    }

    pub fn ram_usage_percent(&self) -> f32 {
        if self.total_mem_bytes == 0 {
            0.0
        } else {
            (self.used_mem_bytes as f64 / self.total_mem_bytes as f64 * 100.0) as f32
        }
    }

    pub fn swap_usage_percent(&self) -> f32 {
        if self.total_swap_bytes == 0 {
            0.0
        } else {
            (self.used_swap_bytes as f64 / self.total_swap_bytes as f64 * 100.0) as f32
        }
    }
}
