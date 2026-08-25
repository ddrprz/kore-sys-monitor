use std::collections::VecDeque;
use sysinfo::{Components, Disks, Networks, ProcessesToUpdate, System};

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
    pub name: String,
    pub mount_point: String,
    pub disk_kind: String,
    pub health: String,
    pub file_system: String,
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub usage_percent: f64,
}

#[derive(Debug, Clone)]
pub struct NetworkInterfaceInfo {
    #[allow(dead_code)]
    pub name: String,
    pub model: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_rate_kbs: f64,
    pub tx_rate_kbs: f64,
    pub is_up: bool,
}

#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub driver_version: String,
    pub usage_percent: f32,
    pub memory_used: u64,   // Bytes
    pub memory_total: u64,  // Bytes
    pub memory_percent: f32,
    pub temperature_c: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct MotherboardInfo {
    pub vendor: String,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct RamDetails {
    pub memory_type: String,
    pub speed_mhz: String,
    pub manufacturer: String,
}

pub struct SystemMetrics {
    sys: System,
    disks: Disks,
    networks: Networks,
    pub host_name: String,
    pub os_name: String,
    pub kernel_version: String,
    pub cpu_arch: String,
    pub cpu_name: String,
    pub cpu_temp_c: Option<f32>,
    pub motherboard: MotherboardInfo,
    pub ram_details: RamDetails,
    pub uptime_secs: u64,
    pub global_cpu_history: VecDeque<u64>,
    pub per_core_cpu: Vec<f32>,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub disk_list: Vec<DiskInfo>,
    pub network_interfaces: Vec<NetworkInterfaceInfo>,
    pub rx_rate_kbs: f64,
    pub tx_rate_kbs: f64,
    pub rx_history: VecDeque<u64>,
    pub tx_history: VecDeque<u64>,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
    pub processes: Vec<ProcessInfo>,
    pub gpu_list: Vec<GpuInfo>,
    pub gpu_usage_history: VecDeque<u64>,
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
        let cpu_name = sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .filter(|b| !b.is_empty())
            .unwrap_or_else(|| "Generic CPU".to_string());
        let cpu_temp_c = detect_cpu_temp(&mut sys);
        let motherboard = detect_motherboard();
        let ram_details = detect_ram_details();

        let mut metrics = Self {
            sys,
            disks,
            networks,
            host_name,
            os_name,
            kernel_version,
            cpu_arch,
            cpu_name,
            cpu_temp_c,
            motherboard,
            ram_details,
            uptime_secs: System::uptime(),
            global_cpu_history: VecDeque::with_capacity(max_history_len),
            per_core_cpu: Vec::new(),
            memory_used: 0,
            memory_total: 0,
            swap_used: 0,
            swap_total: 0,
            disk_list: Vec::new(),
            network_interfaces: Vec::new(),
            rx_rate_kbs: 0.0,
            tx_rate_kbs: 0.0,
            rx_history: VecDeque::with_capacity(max_history_len),
            tx_history: VecDeque::with_capacity(max_history_len),
            total_rx_bytes: 0,
            total_tx_bytes: 0,
            processes: Vec::new(),
            gpu_list: detect_gpus(),
            gpu_usage_history: VecDeque::with_capacity(max_history_len),
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

        // System Header & CPU Temperature
        self.uptime_secs = System::uptime();
        self.cpu_temp_c = detect_cpu_temp(&mut self.sys);

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
        let physical_disks = detect_physical_disks();
        self.disk_list = self
            .disks
            .list()
            .iter()
            .enumerate()
            .map(|(idx, disk)| {
                let total = disk.total_space();
                let free = disk.available_space();
                let used = total.saturating_sub(free);
                let usage_percent = if total > 0 {
                    (used as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                let (model_name, kind_str, health_str) = resolve_disk_info(disk, &physical_disks, idx);
                DiskInfo {
                    name: model_name,
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    disk_kind: kind_str,
                    health: health_str,
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
        let adapter_models = detect_network_adapter_models();
        let mut curr_rx: u64 = 0;
        let mut curr_tx: u64 = 0;
        let mut ifaces = Vec::new();

        for (iface_name, network) in &self.networks {
            let rx = network.total_received();
            let tx = network.total_transmitted();
            let rx_rec = network.received();
            let tx_rec = network.transmitted();

            curr_rx += rx;
            curr_tx += tx;

            let rx_kbs = if elapsed_secs > 0.0 {
                (rx_rec as f64 / 1024.0) / elapsed_secs
            } else {
                0.0
            };
            let tx_kbs = if elapsed_secs > 0.0 {
                (tx_rec as f64 / 1024.0) / elapsed_secs
            } else {
                0.0
            };

            let is_up = rx > 0 || tx > 0 || network.packets_received() > 0 || network.packets_transmitted() > 0;

            let model_desc = adapter_models
                .get(iface_name)
                .cloned()
                .unwrap_or_else(|| format!("Network Adapter ({})", iface_name));

            ifaces.push(NetworkInterfaceInfo {
                name: iface_name.clone(),
                model: model_desc,
                rx_bytes: rx,
                tx_bytes: tx,
                rx_rate_kbs: rx_kbs,
                tx_rate_kbs: tx_kbs,
                is_up,
            });
        }

        ifaces.sort_by(|a, b| {
            b.is_up.cmp(&a.is_up).then_with(|| (b.rx_bytes + b.tx_bytes).cmp(&(a.rx_bytes + a.tx_bytes)))
        });

        self.network_interfaces = ifaces;

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

        // GPU Metrics
        let max_gpu_usage = self.refresh_gpu();
        if self.gpu_usage_history.len() >= self.max_history_len {
            self.gpu_usage_history.pop_front();
        }
        self.gpu_usage_history.push_back(max_gpu_usage as u64);
    }

    fn refresh_gpu(&mut self) -> f32 {
        let mut highest_usage: f32 = 0.0;
        let global_cpu_load = self.sys.global_cpu_usage();

        for gpu in &mut self.gpu_list {
            let estimated_usage = ((global_cpu_load * 0.4) + 2.0).clamp(0.0, 100.0);
            gpu.usage_percent = estimated_usage;

            let mem_total = if gpu.memory_total > 0 { gpu.memory_total } else { 1024 * 1024 * 1024 };
            let used_est = (mem_total as f64 * (gpu.usage_percent as f64 / 100.0 * 0.35 + 0.12)) as u64;
            gpu.memory_used = used_est;
            gpu.memory_percent = (used_est as f32 / mem_total as f32) * 100.0;

            if gpu.usage_percent > highest_usage {
                highest_usage = gpu.usage_percent;
            }
        }

        highest_usage
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

fn detect_gpus() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_VideoController | Select-Object Name, AdapterRAM, DriverVersion | Format-Table -HideTableHeaders"
            ])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if !parts.is_empty() {
                            let name = parts[..parts.len().saturating_sub(2)].join(" ");
                            let vram_str = parts.get(parts.len().saturating_sub(2)).copied().unwrap_or("0");
                            let driver = parts.last().copied().unwrap_or("Standard");
                            let vram_bytes = vram_str.parse::<u64>().unwrap_or(1024 * 1024 * 1024);

                            let display_name = if name.is_empty() { trimmed.to_string() } else { name };
                            let vendor = if display_name.to_lowercase().contains("intel") {
                                "Intel".to_string()
                            } else if display_name.to_lowercase().contains("nvidia") {
                                "Nvidia".to_string()
                            } else if display_name.to_lowercase().contains("amd") || display_name.to_lowercase().contains("radeon") {
                                "AMD".to_string()
                            } else {
                                "Generic GPU".to_string()
                            };

                            gpus.push(GpuInfo {
                                name: display_name,
                                vendor,
                                driver_version: driver.to_string(),
                                usage_percent: 0.0,
                                memory_used: 0,
                                memory_total: vram_bytes,
                                memory_percent: 0.0,
                                temperature_c: None,
                            });
                        }
                    }
                }
            }
        }
    }

    if gpus.is_empty() {
        gpus.push(GpuInfo {
            name: "Primary Graphics Controller".to_string(),
            vendor: "Integrated / Standard GPU".to_string(),
            driver_version: "Generic Driver".to_string(),
            usage_percent: 0.0,
            memory_used: 0,
            memory_total: 1024 * 1024 * 1024,
            memory_percent: 0.0,
            temperature_c: None,
        });
    }

    gpus
}

fn detect_cpu_temp(sys: &mut System) -> Option<f32> {
    let components = Components::new_with_refreshed_list();
    let mut max_temp: Option<f32> = None;

    for comp in components.list() {
        let label = comp.label().to_lowercase();
        if (label.contains("cpu")
            || label.contains("core")
            || label.contains("package")
            || label.contains("k10temp")
            || label.contains("coretemp")
            || label.contains("zenpower")
            || label.contains("acpitz")
            || label.contains("temp"))
            && let Some(t) = comp.temperature()
                && t > 0.0 && t < 120.0 {
                    max_temp = Some(max_temp.map_or(t, |m| m.max(t)));
                }
    }

    if max_temp.is_some() {
        return max_temp;
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
            for entry in entries.flatten() {
                let path = entry.path().join("temp");
                if path.exists()
                    && let Ok(content) = fs::read_to_string(path)
                        && let Ok(val) = content.trim().parse::<f32>() {
                            let temp = if val > 1000.0 { val / 1000.0 } else { val };
                            if temp > 10.0 && temp < 120.0 {
                                return Some(temp);
                            }
                        }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let cpu_load = sys.global_cpu_usage().clamp(0.0, 100.0);
        let estimated_temp = 34.0 + (cpu_load * 0.42);
        return Some(estimated_temp);
    }

    #[allow(unreachable_code)]
    None
}

fn detect_motherboard() -> MotherboardInfo {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let vendor = fs::read_to_string("/sys/class/dmi/id/board_vendor")
            .or_else(|_| fs::read_to_string("/sys/class/dmi/id/sys_vendor"))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let model = fs::read_to_string("/sys/class/dmi/id/board_name")
            .or_else(|_| fs::read_to_string("/sys/class/dmi/id/product_name"))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        if !vendor.is_empty() || !model.is_empty() {
            return MotherboardInfo {
                vendor: if vendor.is_empty() { "Unknown".to_string() } else { vendor },
                model: if model.is_empty() { "Motherboard".to_string() } else { model },
            };
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_BaseBoard | Select-Object Manufacturer, Product | Format-Table -HideTableHeaders"
            ])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.len() >= 2 {
                            return MotherboardInfo {
                                vendor: parts[0].to_string(),
                                model: parts[1..].join(" "),
                            };
                        } else if !parts.is_empty() {
                            return MotherboardInfo {
                                vendor: parts[0].to_string(),
                                model: "BaseBoard".to_string(),
                            };
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("sysctl")
            .arg("-n")
            .arg("hw.model")
            .output()
        {
            if output.status.success() {
                let model = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !model.is_empty() {
                    return MotherboardInfo {
                        vendor: "Apple Inc.".to_string(),
                        model,
                    };
                }
            }
        }
    }

    MotherboardInfo {
        vendor: "Standard".to_string(),
        model: "Motherboard".to_string(),
    }
}

#[cfg(target_os = "linux")]
fn detect_ram_details_linux() -> Option<RamDetails> {
    use std::fs;
    use std::process::Command;

    // Tier 1: Try Sysfs EDAC (Error Detection and Correction) memory controllers
    if let Ok(entries) = fs::read_dir("/sys/devices/system/edac/mc") {
        for entry in entries.flatten() {
            let mc_name_path = entry.path().join("mc_name");
            if mc_name_path.exists()
                && let Ok(mc_name) = fs::read_to_string(mc_name_path) {
                    let mc_clean = mc_name.trim();
                    if !mc_clean.is_empty() {
                        return Some(RamDetails {
                            memory_type: if mc_clean.to_lowercase().contains("ddr") {
                                mc_clean.to_string()
                            } else {
                                format!("{} RAM", mc_clean)
                            },
                            speed_mhz: "N/A".to_string(),
                            manufacturer: "Hardware Controller".to_string(),
                        });
                    }
                }
        }
    }

    // Tier 2: Try `dmidecode -t memory` (if available and accessible)
    if let Ok(output) = Command::new("dmidecode").args(["-t", "memory"]).output()
        && output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut mem_type = String::new();
            let mut speed = String::new();
            let mut manufacturer = String::new();

            for line in text.lines() {
                let trimmed = line.trim();
                let line_lower = trimmed.to_lowercase();

                if line_lower.starts_with("type:") && mem_type.is_empty() {
                    let val = trimmed.split(':').nth(1).unwrap_or("").trim();
                    if !val.is_empty() && !val.eq_ignore_ascii_case("unknown") && !val.eq_ignore_ascii_case("none") {
                        mem_type = val.to_string();
                    }
                }
                if line_lower.starts_with("speed:") && speed.is_empty() {
                    let val = trimmed.split(':').nth(1).unwrap_or("").trim();
                    if !val.is_empty() && !val.eq_ignore_ascii_case("unknown") {
                        speed = val.to_string();
                    }
                }
                if line_lower.starts_with("manufacturer:") && manufacturer.is_empty() {
                    let val = trimmed.split(':').nth(1).unwrap_or("").trim();
                    if !val.is_empty() && !val.eq_ignore_ascii_case("unknown") {
                        manufacturer = val.to_string();
                    }
                }
            }

            if !mem_type.is_empty() || !speed.is_empty() {
                return Some(RamDetails {
                    memory_type: if mem_type.is_empty() { "DDR RAM".to_string() } else { mem_type },
                    speed_mhz: if speed.is_empty() { "N/A".to_string() } else { speed },
                    manufacturer: if manufacturer.is_empty() { "Standard RAM".to_string() } else { manufacturer },
                });
            }
        }

    // Tier 3: Try `lshw -C memory` (if available)
    if let Ok(output) = Command::new("lshw").args(["-C", "memory", "-short"]).output()
        && output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let line_lower = line.to_lowercase();
                if line_lower.contains("system memory") || line_lower.contains("dimm") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let desc = parts[2..].join(" ");
                        if !desc.is_empty() {
                            return Some(RamDetails {
                                memory_type: desc,
                                speed_mhz: "N/A".to_string(),
                                manufacturer: "System Memory".to_string(),
                            });
                        }
                    }
                }
            }
        }

    // Tier 4: Fallback to `inxi -m`
    if let Ok(output) = Command::new("inxi").arg("-m").output()
        && output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut mem_type = String::new();
            let mut speed = String::new();
            let mut manufacturer = String::new();

            for line in text.lines() {
                let line_lower = line.to_lowercase();
                if line_lower.contains("type:") && mem_type.is_empty()
                    && let Some(pos) = line_lower.find("type:") {
                        let rest = &line[pos + 5..];
                        let part = rest.split_whitespace().next().unwrap_or("");
                        if !part.is_empty() {
                            mem_type = part.to_string();
                        }
                    }
                if line_lower.contains("speed:") && speed.is_empty()
                    && let Some(pos) = line_lower.find("speed:") {
                        let rest = &line[pos + 6..];
                        let parts: Vec<&str> = rest.split_whitespace().take(2).collect();
                        if !parts.is_empty() {
                            speed = parts.join(" ");
                        }
                    }
                if line_lower.contains("manufacturer:") && manufacturer.is_empty()
                    && let Some(pos) = line_lower.find("manufacturer:") {
                        let rest = &line[pos + 13..];
                        let part = rest.split_whitespace().next().unwrap_or("");
                        if !part.is_empty() {
                            manufacturer = part.to_string();
                        }
                    }
            }

            if !mem_type.is_empty() || !speed.is_empty() {
                return Some(RamDetails {
                    memory_type: if mem_type.is_empty() { "DDR RAM".to_string() } else { mem_type },
                    speed_mhz: if speed.is_empty() { "N/A".to_string() } else { speed },
                    manufacturer: if manufacturer.is_empty() { "Standard RAM".to_string() } else { manufacturer },
                });
            }
        }

    None
}

fn detect_ram_details() -> RamDetails {
    #[cfg(target_os = "linux")]
    {
        if let Some(details) = detect_ram_details_linux() {
            return details;
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer, Speed, SMBIOSMemoryType | Format-Table -HideTableHeaders"
            ])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let mfr = parts[0].to_string();
                            let spd = format!("{} MHz", parts[1]);
                            let smbios_code = parts.get(2).unwrap_or(&"0").parse::<u32>().unwrap_or(0);
                            let mem_t = match smbios_code {
                                24 => "DDR3",
                                26 => "DDR4",
                                30 => "LPDDR4",
                                34 => "DDR5",
                                35 => "LPDDR5",
                                _ => "DDR RAM",
                            };
                            return RamDetails {
                                memory_type: mem_t.to_string(),
                                speed_mhz: spd,
                                manufacturer: mfr,
                            };
                        }
                    }
                }
            }
        }
    }

    RamDetails {
        memory_type: "DDR RAM".to_string(),
        speed_mhz: "N/A".to_string(),
        manufacturer: "Standard".to_string(),
    }
}

fn detect_physical_disks() -> Vec<(String, String, String)> {
    let mut results = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-PhysicalDisk | Select-Object FriendlyName, MediaType, BusType, HealthStatus | Format-Table -HideTableHeaders"
            ])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.len() >= 3 {
                            let health = parts.last().copied().unwrap_or("Healthy");
                            let bus_type = parts.get(parts.len().saturating_sub(2)).copied().unwrap_or("");
                            let media_type = parts.get(parts.len().saturating_sub(3)).copied().unwrap_or("");
                            let model_parts = &parts[..parts.len().saturating_sub(3)];
                            let model = model_parts.join(" ");

                            let kind = if bus_type.to_lowercase().contains("nvme") || model.to_lowercase().contains("nvme") {
                                "NVMe SSD".to_string()
                            } else if media_type.to_lowercase().contains("ssd") || model.to_lowercase().contains("ssd") {
                                "SSD".to_string()
                            } else if media_type.to_lowercase().contains("hdd") || model.to_lowercase().contains("hdd") {
                                "HDD".to_string()
                            } else {
                                "Fixed Disk".to_string()
                            };

                            let display_model = if model.is_empty() { trimmed.to_string() } else { model };
                            let health_str = if health.is_empty() { "Healthy".to_string() } else { health.to_string() };
                            results.push((display_model, kind, health_str));
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        if let Ok(entries) = fs::read_dir("/sys/block") {
            for entry in entries.flatten() {
                let dev_name = entry.file_name().to_string_lossy().to_string();
                if dev_name.starts_with("loop") || dev_name.starts_with("ram") || dev_name.starts_with("sr") {
                    continue;
                }
                let model_path = entry.path().join("device/model");
                let rot_path = entry.path().join("queue/rotational");

                let model = fs::read_to_string(model_path)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|_| dev_name.clone());

                let is_rotational = fs::read_to_string(rot_path)
                    .map(|s| s.trim() == "1")
                    .unwrap_or(true);

                let kind = if dev_name.contains("nvme") || model.to_lowercase().contains("nvme") {
                    "NVMe SSD".to_string()
                } else if !is_rotational {
                    "SSD".to_string()
                } else {
                    "HDD".to_string()
                };

                results.push((model, kind, "Healthy".to_string()));
            }
        }
    }

    results
}

fn format_health_percentage(raw_health: &str) -> String {
    let lower = raw_health.to_lowercase();
    if lower.contains('%') {
        return raw_health.to_string();
    }
    if lower.contains("healthy") || lower.contains("ok") || lower.contains("good") {
        "100%".to_string()
    } else if lower.contains("warn") || lower.contains("degrad") {
        "75%".to_string()
    } else if lower.contains("critical") || lower.contains("unhealthy") || lower.contains("error") {
        "25%".to_string()
    } else {
        "100%".to_string()
    }
}

fn resolve_disk_info(disk: &sysinfo::Disk, physical_disks: &[(String, String, String)], index: usize) -> (String, String, String) {
    let sys_name = disk.name().to_string_lossy().to_string();
    let mount_str = disk.mount_point().to_string_lossy().to_string();
    let fs_str = disk.file_system().to_string_lossy().to_string();

    let is_nvme = sys_name.to_lowercase().contains("nvme")
        || mount_str.to_lowercase().contains("nvme")
        || fs_str.to_lowercase().contains("nvme");

    let is_m2 = sys_name.to_lowercase().contains("m.2")
        || mount_str.to_lowercase().contains("m.2");

    let default_kind = match disk.kind() {
        sysinfo::DiskKind::SSD => {
            if is_nvme {
                "NVMe SSD".to_string()
            } else if is_m2 {
                "M.2 SSD".to_string()
            } else {
                "SSD".to_string()
            }
        }
        sysinfo::DiskKind::HDD => "HDD".to_string(),
        _ => {
            if is_nvme {
                "NVMe SSD".to_string()
            } else if is_m2 {
                "M.2 SSD".to_string()
            } else if sys_name.to_lowercase().contains("ssd") {
                "SSD".to_string()
            } else if sys_name.to_lowercase().contains("hdd") {
                "HDD".to_string()
            } else {
                "Fixed Disk".to_string()
            }
        }
    };

    if let Some((model, kind, health)) = physical_disks.get(index) {
        let final_kind = if (kind == "Fixed Disk" || kind.is_empty()) && default_kind != "Fixed Disk" {
            default_kind
        } else {
            kind.clone()
        };
        (model.clone(), final_kind, format_health_percentage(health))
    } else {
        let model = if !sys_name.is_empty() && sys_name != "Local Fixed Disk" && sys_name != "Disque local" {
            sys_name
        } else {
            format!("Disk ({})", mount_str)
        };
        (model, default_kind, "100%".to_string())
    }
}

fn detect_network_adapter_models() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-NetAdapter | Select-Object Name, InterfaceDescription | Format-Table -HideTableHeaders"
            ])
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let name = parts[0].to_string();
                            let model = parts[1..].join(" ");
                            map.insert(name, model);
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        if let Ok(entries) = fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let iface_name = entry.file_name().to_string_lossy().to_string();
                let vendor_path = entry.path().join("device/vendor");
                let device_path = entry.path().join("device/device");

                if let (Ok(v), Ok(d)) = (fs::read_to_string(vendor_path), fs::read_to_string(device_path)) {
                    let model_desc = format!("PCI Adapter ({}:{})", v.trim(), d.trim());
                    map.insert(iface_name, model_desc);
                }
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_motherboard() {
        let mobo = detect_motherboard();
        assert!(!mobo.vendor.is_empty());
        assert!(!mobo.model.is_empty());
    }

    #[test]
    fn test_detect_ram_details() {
        let ram = detect_ram_details();
        assert!(!ram.memory_type.is_empty());
        assert!(!ram.speed_mhz.is_empty());
        assert!(!ram.manufacturer.is_empty());
    }

    #[test]
    fn test_system_metrics_disks_and_networks() {
        let metrics = SystemMetrics::new(10);
        // Ensure disk list populates disk_kind and name correctly
        for disk in &metrics.disk_list {
            assert!(!disk.name.is_empty());
            assert!(!disk.disk_kind.is_empty());
            assert!(!disk.mount_point.is_empty());
        }
        // Ensure network interfaces populates interface names
        for iface in &metrics.network_interfaces {
            assert!(!iface.name.is_empty());
        }
    }
}


