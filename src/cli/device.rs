// CF-VOID Device Statistics Module
// Author: CYBER-FORCE
// Live system stats: CPU, RAM, Disk, Network, Temperature, Battery

use std::time::Instant;

pub struct DeviceStats {
    pub cpu_percent: f32,
    pub cpu_cores: usize,
    pub cpu_freq_mhz: u64,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub ram_percent: f32,
    pub disk_used_gb: f32,
    pub disk_total_gb: f32,
    pub disk_percent: f32,
    pub disk_mount: String,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub net_iface: String,
    pub temperature_c: f32,
    pub uptime_secs: u64,
    pub platform: String,
    pub arch: String,
    pub kernel: String,
    pub hostname: String,
    pub last_update: Instant,
    _sys: sysinfo::System,
}

impl DeviceStats {
    pub fn new() -> Self {
        let mut sys = sysinfo::System::new_all();
        sysinfo::set_open_files_limit(isize::MAX);

        let platform = if cfg!(target_os = "android") || std::env::var("ANDROID_ROOT").is_ok() {
            "Linux (Termux)"
        } else if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "macos") {
            "macOS"
        } else {
            "Linux"
        };

        Self {
            cpu_percent: 0.0,
            cpu_cores: sys.cpus().len(),
            cpu_freq_mhz: 0,
            ram_used_mb: 0,
            ram_total_mb: 0,
            ram_percent: 0.0,
            disk_used_gb: 0.0,
            disk_total_gb: 0.0,
            disk_percent: 0.0,
            disk_mount: "/".to_string(),
            net_rx_bytes: 0,
            net_tx_bytes: 0,
            net_iface: "lo".to_string(),
            temperature_c: 0.0,
            uptime_secs: 0,
            platform: platform.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            kernel: Self::kernel_version(),
            hostname: Self::hostname(),
            last_update: Instant::now(),
            _sys: sys,
        }
    }

    pub fn update(&mut self) {
        self._sys.refresh_all();
        self._sys.refresh_processes();
        self._sys.refresh_cpu();
        self._sys.refresh_memory();

        self.ram_total_mb = self._sys.total_memory() / 1024 / 1024;
        self.ram_used_mb = self._sys.used_memory() / 1024 / 1024;
        self.ram_percent = if self.ram_total_mb > 0 {
            (self.ram_used_mb as f32 / self.ram_total_mb as f32) * 100.0
        } else { 0.0 };

        if let Some(cpu) = self._sys.cpus().first() {
            self.cpu_freq_mhz = cpu.frequency();
        }

        let total_cpu: f32 = self._sys.cpus().iter().map(|c| c.cpu_usage()).sum();
        self.cpu_percent = total_cpu / self._sys.cpus().len().max(1) as f32;

        // Disk info
        let mut disks = sysinfo::Disks::new_with_refreshed_list();
        if let Some(disk) = disks.iter().next() {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            self.disk_total_gb = total as f32 / (1024.0 * 1024.0 * 1024.0);
            self.disk_used_gb = used as f32 / (1024.0 * 1024.0 * 1024.0);
            self.disk_percent = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else { 0.0 };
            self.disk_mount = disk.name().to_string_lossy().to_string();
        }

        // Network info
        let networks = sysinfo::Networks::new_with_refreshed_list();
        for (name, data) in networks.iter() {
            self.net_iface = name.clone();
            self.net_rx_bytes = data.total_received();
            self.net_tx_bytes = data.total_transmitted();
            break;
        }

        self.uptime_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.temperature_c = Self::cpu_temperature();
        self.last_update = Instant::now();
    }

    fn kernel_version() -> String {
        if std::env::var("ANDROID_ROOT").is_ok() {
            if let Ok(content) = std::fs::read_to_string("/proc/version") {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() > 2 {
                    return parts[2].to_string();
                }
            }
            return "Android".to_string();
        }
        if let Ok(content) = std::fs::read_to_string("/proc/version") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() > 2 {
                return parts[2].to_string();
            }
        }
        "unknown".to_string()
    }

    fn hostname() -> String {
        std::env::var("HOSTNAME")
            .or_else(|_| std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string()))
            .unwrap_or_else(|_| "localhost".to_string())
    }

    fn cpu_temperature() -> f32 {
        if std::env::var("ANDROID_ROOT").is_ok() {
            for path in &["/sys/class/thermal/thermal_zone0/temp", "/sys/class/thermal/thermal_zone1/temp"] {
                if let Ok(content) = std::fs::read_to_string(path) {
                    if let Ok(temp) = content.trim().parse::<f32>() {
                        return temp / 1000.0;
                    }
                }
            }
            return 0.0;
        }

        for path in &["/sys/class/thermal/thermal_zone0/temp", "/sys/class/thermal/thermal_zone1/temp"] {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(temp) = content.trim().parse::<f32>() {
                    return temp / 1000.0;
                }
            }
        }
        0.0
    }

    pub fn format_uptime(&self) -> String {
        let h = self.uptime_secs / 3600;
        let m = (self.uptime_secs % 3600) / 60;
        let s = self.uptime_secs % 60;
        format!("{}h {}m {}s", h, m, s)
    }

    pub fn format_bytes(bytes: u64) -> String {
        if bytes >= 1_073_741_824 {
            format!("{}GB", bytes / 1_073_741_824)
        } else if bytes >= 1_048_576 {
            format!("{}MB", bytes / 1_048_576)
        } else if bytes >= 1024 {
            format!("{}KB", bytes / 1024)
        } else {
            format!("{}B", bytes)
        }
    }
}

pub fn ascii_bar(percent: f32, width: usize) -> String {
    let clamped = percent.clamp(0.0, 100.0);
    let filled = (clamped / 100.0 * width as f32).floor() as usize;
    let empty = width.saturating_sub(filled);
    let bar = format!("{}{}", "█".repeat(filled), "▱".repeat(empty));
    let pct = format!("[{:.0}%]", clamped);
    format!("{} {}", bar, pct)
}
