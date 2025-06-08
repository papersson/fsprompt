//! Performance measurement utilities

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Frame time tracker for UI performance
#[derive(Debug, Clone)]
pub struct FrameTimer {
    /// Rolling buffer of last N frame times
    frame_times: Arc<[AtomicU64; 120]>, // 2 seconds at 60 FPS
    /// Current position in buffer
    position: Arc<AtomicUsize>,
    /// Last frame timestamp
    last_frame: Arc<AtomicU64>,
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self {
            frame_times: Arc::new([(); 120].map(|()| AtomicU64::new(0))),
            position: Arc::new(AtomicUsize::new(0)),
            last_frame: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl FrameTimer {
    /// Record the start of a new frame
    pub fn frame_start(&self) {
        // Use a stable epoch for timing
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::ZERO)
            .as_micros()
            .try_into()
            .unwrap_or(u64::MAX);

        let last = self.last_frame.swap(now, Ordering::Relaxed);

        if last > 0 && now > last {
            let frame_time = now - last;
            let pos = self.position.fetch_add(1, Ordering::Relaxed) % 120;
            self.frame_times[pos].store(frame_time, Ordering::Relaxed);
        }
    }

    /// Get frame time statistics
    pub fn stats(&self) -> FrameStats {
        let mut times: Vec<u64> = self
            .frame_times
            .iter()
            .map(|t| t.load(Ordering::Relaxed))
            .filter(|&t| t > 0)
            .collect();

        if times.is_empty() {
            return FrameStats::default();
        }

        times.sort_unstable();

        let sum: u64 = times.iter().sum();
        let count = times.len();

        FrameStats {
            avg_fps: if sum > 0 {
                #[allow(clippy::cast_precision_loss)]
                {
                    (count as f64 * 1_000_000.0) / (sum as f64)
                }
            } else {
                0.0
            },
            #[allow(clippy::cast_precision_loss)]
            p50_ms: times[count / 2] as f64 / 1000.0,
            #[allow(clippy::cast_precision_loss)]
            p95_ms: times[count * 95 / 100] as f64 / 1000.0,
            #[allow(clippy::cast_precision_loss)]
            p99_ms: times[count * 99 / 100] as f64 / 1000.0,
            #[allow(clippy::cast_precision_loss)]
            max_ms: times[count - 1] as f64 / 1000.0,
        }
    }
}

/// Frame timing statistics
#[derive(Debug, Default)]
pub struct FrameStats {
    /// Average frames per second
    pub avg_fps: f64,
    /// 50th percentile frame time in milliseconds
    pub p50_ms: f64,
    /// 95th percentile frame time in milliseconds
    pub p95_ms: f64,
    /// 99th percentile frame time in milliseconds
    pub p99_ms: f64,
    /// Maximum frame time in milliseconds
    pub max_ms: f64,
}

/// Scoped timer for measuring specific operations
pub struct ScopedTimer<'a> {
    name: &'a str,
    start: Instant,
    budget: Option<Duration>,
}

impl<'a> ScopedTimer<'a> {
    /// Create a new scoped timer
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            start: Instant::now(),
            budget: None,
        }
    }

    /// Create a timer with a performance budget
    pub fn with_budget(name: &'a str, budget: Duration) -> Self {
        Self {
            name,
            start: Instant::now(),
            budget: Some(budget),
        }
    }
}

impl std::fmt::Debug for ScopedTimer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopedTimer")
            .field("name", &self.name)
            .field("elapsed", &self.start.elapsed())
            .field("budget", &self.budget)
            .finish()
    }
}

impl Drop for ScopedTimer<'_> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();

        if let Some(budget) = self.budget {
            if elapsed > budget {
                eprintln!(
                    "⚠️  Performance WARNING: {} took {:?}, budget was {:?} ({}x over)",
                    self.name,
                    elapsed,
                    budget,
                    elapsed.as_secs_f64() / budget.as_secs_f64()
                );
            }
        }

        #[cfg(debug_assertions)]
        {
            println!("⏱️  {}: {:?}", self.name, elapsed);
        }
    }
}

/// Measure and enforce performance budgets
#[macro_export]
macro_rules! perf_budget {
    ($name:expr, $budget_ms:expr, $code:block) => {{
        let _timer = $crate::utils::perf::ScopedTimer::with_budget(
            $name,
            std::time::Duration::from_millis($budget_ms),
        );
        $code
    }};
}

/// Quick performance measurement
#[macro_export]
macro_rules! perf_measure {
    ($name:expr, $code:block) => {{
        let _timer = $crate::utils::perf::ScopedTimer::new($name);
        $code
    }};
}

/// Memory usage tracker
#[derive(Debug)]
pub struct MemoryTracker {
    initial_rss: usize,
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            initial_rss: Self::current_rss(),
        }
    }

    /// Get current resident set size in bytes
    #[cfg(target_os = "macos")]
    fn current_rss() -> usize {
        use std::mem;
        use std::os::raw::c_int;

        unsafe {
            let mut info: libc::mach_task_basic_info = mem::zeroed();
            #[allow(clippy::cast_possible_truncation)]
            let mut count = mem::size_of::<libc::mach_task_basic_info>() as u32;

            #[allow(deprecated)]
            let result = libc::task_info(
                libc::mach_task_self(),
                libc::MACH_TASK_BASIC_INFO,
                (&raw mut info).cast::<c_int>(),
                &mut count,
            );

            if result == libc::KERN_SUCCESS {
                info.resident_size.try_into().unwrap_or(usize::MAX)
            } else {
                0
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn current_rss() -> usize {
        // Fallback for other platforms
        0
    }

    /// Get memory growth since creation
    pub fn growth_mb(&self) -> f64 {
        let current = Self::current_rss();
        #[allow(clippy::cast_precision_loss)]
        {
            current.saturating_sub(self.initial_rss) as f64 / 1_048_576.0
        }
    }
}

/// Performance overlay for egui
#[derive(Debug)]
pub struct PerfOverlay {
    frame_timer: FrameTimer,
    memory_tracker: MemoryTracker,
    show: bool,
}

impl Default for PerfOverlay {
    fn default() -> Self {
        Self {
            frame_timer: FrameTimer::default(),
            memory_tracker: MemoryTracker::new(),
            show: cfg!(debug_assertions), // Show in debug builds
        }
    }
}

impl PerfOverlay {
    /// Toggle overlay visibility
    pub const fn toggle(&mut self) {
        self.show = !self.show;
    }

    /// Update frame timing
    pub fn frame_start(&self) {
        self.frame_timer.frame_start();
    }

    /// Render the overlay
    pub fn show(&self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        let stats = self.frame_timer.stats();
        let mem_growth = self.memory_tracker.growth_mb();

        // Position at bottom right
        let screen_rect = ctx.screen_rect();
        let panel_width = 200.0;
        let panel_height = 150.0; // Approximate height
        let margin = 10.0;

        let pos = egui::pos2(
            screen_rect.max.x - panel_width - margin,
            screen_rect.max.y - panel_height - margin,
        );

        egui::Area::new(egui::Id::new("perf_overlay"))
            .fixed_pos(pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                // Semi-transparent background
                let frame = egui::Frame::window(ui.style())
                    .fill(egui::Color32::from_rgba_premultiplied(30, 30, 30, 220))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)));

                frame.show(ui, |ui| {
                    ui.set_min_width(panel_width);
                    ui.label("Performance (Dev)");
                    ui.separator();

                    // FPS with color coding
                    let fps_color = if stats.avg_fps >= 120.0 {
                        egui::Color32::GREEN
                    } else if stats.avg_fps >= 60.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };

                    ui.colored_label(fps_color, format!("FPS: {:.0}", stats.avg_fps));

                    // Frame times
                    ui.label(format!("Frame P50: {:.1}ms", stats.p50_ms));
                    ui.label(format!("Frame P95: {:.1}ms", stats.p95_ms));
                    ui.label(format!("Frame P99: {:.1}ms", stats.p99_ms));

                    let max_color = if stats.max_ms > 16.7 {
                        egui::Color32::RED
                    } else if stats.max_ms > 8.3 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::GREEN
                    };

                    ui.colored_label(max_color, format!("Frame Max: {:.1}ms", stats.max_ms));

                    ui.separator();

                    // Memory usage
                    let mem_color = if mem_growth > 100.0 {
                        egui::Color32::RED
                    } else if mem_growth > 50.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::GREEN
                    };

                    ui.colored_label(mem_color, format!("Mem Growth: {mem_growth:.1}MB"));
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_frame_timer() {
        let timer = FrameTimer::default();

        // Simulate frames with controlled timing
        // Use a smaller sleep to avoid timing issues in CI
        for _ in 0..30 {
            timer.frame_start();
            // Sleep a tiny amount just to ensure some time passes
            thread::sleep(Duration::from_micros(100));
        }

        let stats = timer.stats();
        // Just verify that we got some frames recorded
        assert!(stats.avg_fps > 0.0);
        assert!(stats.p50_ms >= 0.0);
        assert!(stats.max_ms >= stats.p50_ms);
    }

    #[test]
    fn test_scoped_timer() {
        let _timer = ScopedTimer::with_budget("test", Duration::from_millis(10));
        thread::sleep(Duration::from_millis(5));
        // Should not print warning
    }
}
