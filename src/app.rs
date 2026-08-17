use crate::system::{ProcessInfo, SystemMetrics};
use crate::theme::{Theme, ThemeVariant};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Tab {
    Overview = 0,
    Processes = 1,
    StorageNet = 2,
    CpuDetail = 3,
}

impl Tab {
    pub fn from_index(index: usize) -> Self {
        match index % 4 {
            0 => Tab::Overview,
            1 => Tab::Processes,
            2 => Tab::StorageNet,
            3 => Tab::CpuDetail,
            _ => Tab::Overview,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Tab::Overview => "[1] Overview",
            Tab::Processes => "[2] Processes",
            Tab::StorageNet => "[3] Storage & Net",
            Tab::CpuDetail => "[4] CPU Detail",
        }
    }

    pub fn compact_title(&self) -> &'static str {
        match self {
            Tab::Overview => "1:Over",
            Tab::Processes => "2:Proc",
            Tab::StorageNet => "3:Disk",
            Tab::CpuDetail => "4:CPU",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SortColumn {
    Cpu,
    Memory,
    Pid,
    Name,
}

impl SortColumn {
    pub fn next(&self) -> Self {
        match self {
            SortColumn::Cpu => SortColumn::Memory,
            SortColumn::Memory => SortColumn::Pid,
            SortColumn::Pid => SortColumn::Name,
            SortColumn::Name => SortColumn::Cpu,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SortColumn::Cpu => "CPU%",
            SortColumn::Memory => "MEM%",
            SortColumn::Pid => "PID",
            SortColumn::Name => "Name",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn reverse(&self) -> Self {
        match self {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum InputMode {
    Normal,
    Searching,
    KillModal,
    HelpModal,
}

pub struct App {
    pub active_tab: Tab,
    pub metrics: SystemMetrics,
    pub selected_process_index: usize,
    pub sort_column: SortColumn,
    pub sort_order: SortOrder,
    pub search_query: String,
    pub input_mode: InputMode,
    pub selected_kill_process: Option<ProcessInfo>,
    pub status_message: Option<(String, std::time::Instant)>,
    pub theme: Theme,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Overview,
            metrics: SystemMetrics::new(60),
            selected_process_index: 0,
            sort_column: SortColumn::Cpu,
            sort_order: SortOrder::Descending,
            search_query: String::new(),
            input_mode: InputMode::Normal,
            selected_kill_process: None,
            status_message: None,
            theme: Theme::from_variant(ThemeVariant::CyberCyan),
            should_quit: false,
        }
    }

    pub fn cycle_theme(&mut self) {
        let next_var = self.theme.variant.next();
        self.theme = Theme::from_variant(next_var);
        self.set_status(format!("Tema cambiado a '{}'", next_var.name()));
    }


    pub fn update(&mut self, elapsed_secs: f64) {
        self.metrics.refresh(elapsed_secs);
        self.clamp_process_selection();

        // Clear status message after 3 seconds
        if let Some((_, time)) = &self.status_message {
            if time.elapsed().as_secs() >= 3 {
                self.status_message = None;
            }
        }
    }

    pub fn filtered_sorted_processes(&self) -> Vec<ProcessInfo> {
        let mut list: Vec<ProcessInfo> = self
            .metrics
            .processes
            .iter()
            .filter(|p| {
                if self.search_query.is_empty() {
                    true
                } else {
                    let q = self.search_query.to_lowercase();
                    p.name.to_lowercase().contains(&q)
                        || p.pid.to_string().contains(&q)
                        || p.command.to_lowercase().contains(&q)
                }
            })
            .cloned()
            .collect();

        list.sort_by(|a, b| {
            let cmp = match self.sort_column {
                SortColumn::Cpu => a
                    .cpu_usage
                    .partial_cmp(&b.cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Equal),
                SortColumn::Memory => a.memory.cmp(&b.memory),
                SortColumn::Pid => a.pid.cmp(&b.pid),
                SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            };

            match self.sort_order {
                SortOrder::Ascending => cmp,
                SortOrder::Descending => cmp.reverse(),
            }
        });

        list
    }

    pub fn clamp_process_selection(&mut self) {
        let count = self.filtered_sorted_processes().len();
        if count == 0 {
            self.selected_process_index = 0;
        } else if self.selected_process_index >= count {
            self.selected_process_index = count.saturating_sub(1);
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = Tab::from_index(self.active_tab as usize + 1);
    }

    pub fn previous_tab(&mut self) {
        let idx = (self.active_tab as usize + 3) % 4;
        self.active_tab = Tab::from_index(idx);
    }

    pub fn select_tab(&mut self, tab: Tab) {
        self.active_tab = tab;
    }

    pub fn select_next_process(&mut self) {
        let count = self.filtered_sorted_processes().len();
        if count > 0 && self.selected_process_index < count - 1 {
            self.selected_process_index += 1;
        }
    }

    pub fn select_previous_process(&mut self) {
        if self.selected_process_index > 0 {
            self.selected_process_index -= 1;
        }
    }

    pub fn cycle_sort(&mut self) {
        self.sort_column = self.sort_column.next();
    }

    pub fn reverse_sort(&mut self) {
        self.sort_order = self.sort_order.reverse();
    }

    pub fn open_kill_modal(&mut self) {
        let procs = self.filtered_sorted_processes();
        if let Some(proc) = procs.get(self.selected_process_index) {
            self.selected_kill_process = Some(proc.clone());
            self.input_mode = InputMode::KillModal;
        }
    }

    pub fn confirm_kill(&mut self) {
        if let Some(proc) = self.selected_kill_process.take() {
            match self.metrics.kill_process(proc.pid) {
                Ok(()) => {
                    self.set_status(format!("Proceso '{}' (PID {}) terminado.", proc.name, proc.pid));
                }
                Err(e) => {
                    self.set_status(format!("Error: {}", e));
                }
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn cancel_modal(&mut self) {
        self.selected_kill_process = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some((msg, std::time::Instant::now()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_navigation() {
        let mut app = App::new();
        assert_eq!(app.active_tab, Tab::Overview);
        app.next_tab();
        assert_eq!(app.active_tab, Tab::Processes);
        app.next_tab();
        assert_eq!(app.active_tab, Tab::StorageNet);
        app.next_tab();
        assert_eq!(app.active_tab, Tab::CpuDetail);
        app.next_tab();
        assert_eq!(app.active_tab, Tab::Overview);
        app.previous_tab();
        assert_eq!(app.active_tab, Tab::CpuDetail);
    }

    #[test]
    fn test_theme_cycling() {
        let mut app = App::new();
        let initial = app.theme.variant;
        app.cycle_theme();
        assert_ne!(app.theme.variant, initial);
    }

    #[test]
    fn test_sort_cycling() {
        let mut app = App::new();
        assert_eq!(app.sort_column, SortColumn::Cpu);
        app.cycle_sort();
        assert_eq!(app.sort_column, SortColumn::Memory);
        app.reverse_sort();
        assert_eq!(app.sort_order, SortOrder::Ascending);
    }
}

