use crate::system::{ProcessSortKey, SysMetrics};
use sysinfo::System;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview = 0,
    Cpu = 1,
    Memory = 2,
    Processes = 3,
    Disks = 4,
}

impl Tab {
    pub const ALL: [Tab; 5] = [Tab::Overview, Tab::Cpu, Tab::Memory, Tab::Processes, Tab::Disks];

    pub fn title(&self) -> &'static str {
        match self {
            Tab::Overview => "1: Overview",
            Tab::Cpu => "2: CPU Cores",
            Tab::Memory => "3: Memory",
            Tab::Processes => "4: Processes",
            Tab::Disks => "5: Storage",
        }
    }
}

pub struct App {
    pub active_tab: Tab,
    pub metrics: SysMetrics,
    pub sys: System,
    pub selected_proc_index: usize,
    pub sort_key: ProcessSortKey,
    pub search_query: String,
    pub is_searching: bool,
    pub status_message: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        let mut metrics = SysMetrics::new();
        metrics.refresh(&mut sys, ProcessSortKey::Cpu, "");

        Self {
            active_tab: Tab::Overview,
            metrics,
            sys,
            selected_proc_index: 0,
            sort_key: ProcessSortKey::Cpu,
            search_query: String::new(),
            is_searching: false,
            status_message: Some("Welcome to hmon! Press '?' for help, Tab to navigate.".into()),
            should_quit: false,
        }
    }

    pub fn tick(&mut self) {
        self.metrics.refresh(&mut self.sys, self.sort_key, &self.search_query);
        // Bound selection index within processes length
        if !self.metrics.processes.is_empty() {
            if self.selected_proc_index >= self.metrics.processes.len() {
                self.selected_proc_index = self.metrics.processes.len() - 1;
            }
        } else {
            self.selected_proc_index = 0;
        }
    }

    pub fn next_tab(&mut self) {
        let current = self.active_tab as usize;
        let next = (current + 1) % Tab::ALL.len();
        self.active_tab = Tab::ALL[next];
    }

    pub fn previous_tab(&mut self) {
        let current = self.active_tab as usize;
        let prev = if current == 0 { Tab::ALL.len() - 1 } else { current - 1 };
        self.active_tab = Tab::ALL[prev];
    }

    pub fn next_process(&mut self) {
        if !self.metrics.processes.is_empty() {
            if self.selected_proc_index + 1 < self.metrics.processes.len() {
                self.selected_proc_index += 1;
            }
        }
    }

    pub fn previous_process(&mut self) {
        if self.selected_proc_index > 0 {
            self.selected_proc_index -= 1;
        }
    }

    pub fn cycle_sort_key(&mut self) {
        self.sort_key = match self.sort_key {
            ProcessSortKey::Cpu => ProcessSortKey::Memory,
            ProcessSortKey::Memory => ProcessSortKey::Pid,
            ProcessSortKey::Pid => ProcessSortKey::Name,
            ProcessSortKey::Name => ProcessSortKey::Cpu,
        };
        self.status_message = Some(format!("Sorted by {:?}", self.sort_key));
    }

    pub fn kill_selected_process(&mut self) {
        if let Some(proc) = self.metrics.processes.get(self.selected_proc_index) {
            let pid = proc.pid;
            let name = proc.name.clone();
            if crate::system::ProcessManager::kill_process(pid) {
                self.status_message = Some(format!("Killed process {} (PID: {})", name, pid));
            } else {
                self.status_message = Some(format!("Failed to kill process {} (PID: {})", name, pid));
            }
        }
    }
}
