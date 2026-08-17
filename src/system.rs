use std::collections::VecDeque;
use sysinfo::{Disks, Networks, ProcessesToUpdate, System};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64, // Bytes
    pub memory_percent: f32,
    pub status: String,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub mount_point: String,
    pub file_system: String,
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub usage_percent: f64,
}

pub struct SystemMetrics {
    sys: System,
    disks: Disks,
    networks: Networks,
    pub host_name: String,
    pub os_name: String,
    pub kernel_version: String,
    pub cpu_arch: String,
    pub uptime_secs: u64,
    pub global_cpu_history: VecDeque<u64>,
    pub per_core_cpu: Vec<f32>,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub disk_list: Vec<DiskInfo>,
    pub rx_rate_kbs: f64,
    pub tx_rate_kbs: f64,
    pub rx_history: VecDeque<u64>,
    pub tx_history: VecDeque<u64>,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub processes: Vec<ProcessInfo>,
    max_history_len: usize,
    prev_rx_total: u64,
    prev_tx_total: u64,
}

impl SystemMetrics {
    pub fn new(max_history_len: usize) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_name = System::name()
            .map(|n| format!("{} {}", n, System::os_version().unwrap_or_default()))
            .unwrap_or_else(|| "Unknown OS".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let cpu_arch = {
            let arch = System::cpu_arch();
            if arch.is_empty() { "Unknown".to_string() } else { arch }
        };

        let mut metrics = Self {
            sys,
            disks,
            networks,
            host_name,
            os_name,
            kernel_version,
            cpu_arch,
            uptime_secs: System::uptime(),
            global_cpu_history: VecDeque::with_capacity(max_history_len),
            per_core_cpu: Vec::new(),
            memory_used: 0,
            memory_total: 0,
            swap_used: 0,
            swap_total: 0,
            disk_list: Vec::new(),
            rx_rate_kbs: 0.0,
            tx_rate_kbs: 0.0,
            rx_history: VecDeque::with_capacity(max_history_len),
            tx_history: VecDeque::with_capacity(max_history_len),
            total_rx_bytes: 0,
            total_tx_bytes: 0,
            processes: Vec::new(),
            max_history_len,
            prev_rx_total: 0,
            prev_tx_total: 0,
        };

        metrics.refresh(1.0);
        metrics
    }

    pub fn refresh(&mut self, elapsed_secs: f64) {
        // Refresh CPU, Memory & Processes
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        // System Header
        self.uptime_secs = System::uptime();

        // CPU Metrics
        let global_cpu = self.sys.global_cpu_usage().clamp(0.0, 100.0);
        if self.global_cpu_history.len() >= self.max_history_len {
            self.global_cpu_history.pop_front();
        }
        self.global_cpu_history.push_back(global_cpu as u64);

        self.per_core_cpu = self.sys.cpus().iter().map(|c| c.cpu_usage()).collect();

        // Memory & Swap
        self.memory_used = self.sys.used_memory();
        self.memory_total = self.sys.total_memory();
        self.swap_used = self.sys.used_swap();
        self.swap_total = self.sys.total_swap();

        // Disks
        self.disks.refresh(true);
        self.disk_list = self
            .disks
            .list()
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let free = disk.available_space();
                let used = total.saturating_sub(free);
                let usage_percent = if total > 0 {
                    (used as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                DiskInfo {
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    file_system: disk.file_system().to_string_lossy().to_string(),
                    total_space: total,
                    used_space: used,
                    free_space: free,
                    usage_percent,
                }
            })
            .collect();

        // Networks
        self.networks.refresh(true);
        let mut curr_rx: u64 = 0;
        let mut curr_tx: u64 = 0;

        for (_iface, network) in &self.networks {
            curr_rx += network.total_received();
            curr_tx += network.total_transmitted();
        }

        self.total_rx_bytes = curr_rx;
        self.total_tx_bytes = curr_tx;

        if self.prev_rx_total > 0 && elapsed_secs > 0.0 {
            let rx_diff = curr_rx.saturating_sub(self.prev_rx_total);
            let tx_diff = curr_tx.saturating_sub(self.prev_tx_total);

            self.rx_rate_kbs = (rx_diff as f64 / 1024.0) / elapsed_secs;
            self.tx_rate_kbs = (tx_diff as f64 / 1024.0) / elapsed_secs;
        } else {
            self.rx_rate_kbs = 0.0;
            self.tx_rate_kbs = 0.0;
        }

        self.prev_rx_total = curr_rx;
        self.prev_tx_total = curr_tx;

        if self.rx_history.len() >= self.max_history_len {
            self.rx_history.pop_front();
        }
        self.rx_history.push_back(self.rx_rate_kbs as u64);

        if self.tx_history.len() >= self.max_history_len {
            self.tx_history.pop_front();
        }
        self.tx_history.push_back(self.tx_rate_kbs as u64);

        // Processes
        let total_mem = if self.memory_total > 0 {
            self.memory_total as f32
        } else {
            1.0
        };

        self.processes = self
            .sys
            .processes()
            .iter()
            .map(|(pid, proc_info)| {
                let mem = proc_info.memory();
                let mem_pct = (mem as f32 / total_mem) * 100.0;
                let cmd = proc_info
                    .cmd()
                    .iter()
                    .map(|s| s.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" ");

                ProcessInfo {
                    pid: pid.as_u32(),
                    name: proc_info.name().to_string_lossy().to_string(),
                    cpu_usage: proc_info.cpu_usage(),
                    memory: mem,
                    memory_percent: mem_pct,
                    status: format!("{:?}", proc_info.status()),
                    command: if cmd.is_empty() {
                        proc_info.name().to_string_lossy().to_string()
                    } else {
                        cmd
                    },
                }
            })
            .collect();
    }

    pub fn kill_process(&mut self, pid: u32) -> Result<(), String> {
        let sys_pid = sysinfo::Pid::from_u32(pid);
        if let Some(process) = self.sys.process(sys_pid) {
            if process.kill() {
                Ok(())
            } else {
                Err(format!("No se pudo terminar el proceso con PID {}", pid))
            }
        } else {
            Err(format!("Proceso con PID {} no encontrado", pid))
        }
    }
}
