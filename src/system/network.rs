use sysinfo::Networks;

#[derive(Debug, Clone)]
pub struct NetInterfaceInfo {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub mac_address: String,
}

#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    pub interfaces: Vec<NetInterfaceInfo>,
    pub total_rx_rate_sec: u64,
    pub total_tx_rate_sec: u64,
}

impl NetworkMetrics {
    pub fn collect() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let mut total_rx = 0;
        let mut total_tx = 0;

        let interfaces = networks
            .iter()
            .map(|(name, net)| {
                let rx = net.received();
                let tx = net.transmitted();
                total_rx += rx;
                total_tx += tx;

                NetInterfaceInfo {
                    name: name.clone(),
                    rx_bytes: rx,
                    tx_bytes: tx,
                    total_rx_bytes: net.total_received(),
                    total_tx_bytes: net.total_transmitted(),
                    mac_address: net.mac_address().to_string(),
                }
            })
            .collect();

        Self {
            interfaces,
            total_rx_rate_sec: total_rx,
            total_tx_rate_sec: total_tx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_metrics_collect() {
        let net = NetworkMetrics::collect();
        // Verify vector initialization
        let _ = net.interfaces.len();
    }
}
