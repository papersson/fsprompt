//! Stress test for UI performance with large directory trees

use eframe::egui;
use fsprompt::ui::tree::DirectoryTree;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("fsPrompt Stress Test - 5000+ files"),
        ..Default::default()
    };

    eframe::run_native(
        "fsPrompt Stress Test",
        native_options,
        Box::new(|cc| Ok(Box::new(StressTestApp::new(cc)))),
    )
}

struct StressTestApp {
    tree: DirectoryTree,
    frame_times: Vec<f64>,
    last_frame: Instant,
    show_stats: bool,
}

impl StressTestApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut tree = DirectoryTree::new();

        // Use the current directory or a large directory
        let test_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/Users"));

        println!("Loading directory tree from: {}", test_path.display());
        tree.set_root(test_path);

        Self {
            tree,
            frame_times: Vec::with_capacity(1000),
            last_frame: Instant::now(),
            show_stats: true,
        }
    }

    fn calculate_stats(&self) -> (f64, f64, f64, f64, usize) {
        if self.frame_times.is_empty() {
            return (0.0, 0.0, 0.0, 0.0, 0);
        }

        let mut sorted = self.frame_times.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let sum: f64 = sorted.iter().sum();
        let avg_ms = sum / sorted.len() as f64;
        let avg_fps = if avg_ms > 0.0 { 1000.0 / avg_ms } else { 0.0 };

        let p50 = sorted[sorted.len() / 2];
        let p95 = sorted[sorted.len() * 95 / 100];
        let p99 = sorted[sorted.len() * 99 / 100];

        // Count visible nodes approximately
        let node_count = self.estimate_visible_nodes();

        (avg_fps, p50, p95, p99, node_count)
    }

    fn estimate_visible_nodes(&self) -> usize {
        // This is a rough estimate - in real app we'd count during render
        let mut count = 0;
        if !self.tree.roots.is_empty() {
            Self::count_expanded_nodes(&self.tree.roots[0], &mut count);
        }
        count
    }

    fn count_expanded_nodes(node: &fsprompt::ui::tree::TreeNode, count: &mut usize) {
        *count += 1;
        if node.is_dir && node.expanded {
            for child in &node.children {
                Self::count_expanded_nodes(child, count);
            }
        }
    }
}

impl eframe::App for StressTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Measure frame time
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame).as_secs_f64() * 1000.0;
        self.last_frame = now;

        if self.frame_times.len() < 1000 {
            self.frame_times.push(frame_time);
        } else {
            // Rolling window
            self.frame_times.remove(0);
            self.frame_times.push(frame_time);
        }

        // Top panel with controls
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🔬 fsPrompt Stress Test");
                ui.separator();

                if ui.button("Expand All").clicked() {
                    if !self.tree.roots.is_empty() {
                        expand_all(&mut self.tree.roots[0]);
                    }
                }

                if ui.button("Collapse All").clicked() {
                    if !self.tree.roots.is_empty() {
                        collapse_all(&mut self.tree.roots[0]);
                    }
                }

                ui.separator();
                ui.checkbox(&mut self.show_stats, "Show Stats");

                if ui.button("Clear Frame Times").clicked() {
                    self.frame_times.clear();
                }
            });
        });

        // Stats panel
        if self.show_stats {
            egui::TopBottomPanel::top("stats").show(ctx, |ui| {
                let (avg_fps, p50, p95, p99, node_count) = self.calculate_stats();

                ui.horizontal(|ui| {
                    // Color code the FPS
                    let fps_color = if avg_fps >= 120.0 {
                        egui::Color32::GREEN
                    } else if avg_fps >= 60.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };

                    ui.colored_label(fps_color, format!("FPS: {:.1}", avg_fps));
                    ui.separator();

                    ui.label(format!("Frame P50: {:.1}ms", p50));
                    ui.label(format!("P95: {:.1}ms", p95));
                    ui.label(format!("P99: {:.1}ms", p99));
                    ui.separator();

                    ui.label(format!("Visible nodes: ~{}", node_count));
                    ui.separator();

                    ui.label(format!("Samples: {}", self.frame_times.len()));
                });
            });
        }

        // Main tree view
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Directory Tree");
            ui.separator();

            self.tree.show(ui);
        });

        // Request continuous repaints for accurate frame timing
        ctx.request_repaint();
    }
}

fn expand_all(node: &mut fsprompt::ui::tree::TreeNode) {
    if node.is_dir {
        node.expanded = true;
        if !node.children_loaded {
            node.load_children();
        }
        for child in &mut node.children {
            expand_all(child);
        }
    }
}

fn collapse_all(node: &mut fsprompt::ui::tree::TreeNode) {
    if node.is_dir {
        node.expanded = false;
        for child in &mut node.children {
            collapse_all(child);
        }
    }
}
