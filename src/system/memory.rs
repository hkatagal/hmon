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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_usage_percent_normal() {
        let mem = MemoryMetrics {
            total_mem_bytes: 16_000_000_000,
            used_mem_bytes: 8_000_000_000,
            free_mem_bytes: 8_000_000_000,
            total_swap_bytes: 4_000_000_000,
            used_swap_bytes: 1_000_000_000,
        };
        assert_eq!(mem.ram_usage_percent(), 50.0);
    }

    #[test]
    fn test_ram_usage_percent_zero_total() {
        let mem = MemoryMetrics {
            total_mem_bytes: 0,
            used_mem_bytes: 0,
            free_mem_bytes: 0,
            total_swap_bytes: 0,
            used_swap_bytes: 0,
        };
        assert_eq!(mem.ram_usage_percent(), 0.0);
        assert_eq!(mem.swap_usage_percent(), 0.0);
    }

    #[test]
    fn test_swap_usage_percent() {
        let mem = MemoryMetrics {
            total_mem_bytes: 100,
            used_mem_bytes: 50,
            free_mem_bytes: 50,
            total_swap_bytes: 200,
            used_swap_bytes: 50,
        };
        assert_eq!(mem.swap_usage_percent(), 25.0);
    }
}
