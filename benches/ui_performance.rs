//! UI performance benchmarks for tree rendering

use criterion::{Criterion, criterion_group, criterion_main};
use fsprompt::ui::tree::TreeNode;
use std::path::PathBuf;
use std::time::Duration;

/// Generate a large tree structure for benchmarking
fn generate_large_tree(depth: usize, files_per_dir: usize) -> TreeNode {
    fn generate_subtree(
        path: PathBuf,
        current_depth: usize,
        max_depth: usize,
        files_per_dir: usize,
    ) -> TreeNode {
        let mut node = TreeNode::new(path.clone());
        node.is_dir = true;
        node.children_loaded = true;

        if current_depth < max_depth {
            // Add subdirectories
            for i in 0..3 {
                let subdir_path = path.join(format!("subdir{}", i));
                let mut subdir =
                    generate_subtree(subdir_path, current_depth + 1, max_depth, files_per_dir);
                subdir.name = format!("subdir{}", i);
                node.children.push(subdir);
            }

            // Add files
            for i in 0..files_per_dir {
                let file_path = path.join(format!("file{}.rs", i));
                let mut file_node = TreeNode::new(file_path);
                file_node.name = format!("file{}.rs", i);
                file_node.is_dir = false;
                node.children.push(file_node);
            }
        }

        node
    }

    generate_subtree(PathBuf::from("/benchmark"), 0, depth, files_per_dir)
}

/// Count total nodes in tree
fn count_nodes(node: &TreeNode) -> usize {
    let mut count = 1;
    for child in &node.children {
        count += count_nodes(child);
    }
    count
}

/// Expand all directories in tree
fn expand_all(node: &mut TreeNode) {
    if node.is_dir {
        node.expanded = true;
        for child in &mut node.children {
            expand_all(child);
        }
    }
}

fn benchmark_tree_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_rendering");
    group.measurement_time(Duration::from_secs(10));

    // We can't directly benchmark UI rendering without a running egui context,
    // but we can benchmark the tree traversal logic

    // Small tree benchmark
    group.bench_function("traverse_small_tree", |b| {
        let tree = generate_large_tree(3, 10); // ~120 nodes
        b.iter(|| {
            let mut count = 0;
            traverse_tree(&tree, &mut count);
            count
        });
    });

    // Medium tree benchmark
    group.bench_function("traverse_medium_tree", |b| {
        let tree = generate_large_tree(4, 20); // ~1,200 nodes
        b.iter(|| {
            let mut count = 0;
            traverse_tree(&tree, &mut count);
            count
        });
    });

    // Large tree benchmark
    group.bench_function("traverse_large_tree", |b| {
        let tree = generate_large_tree(5, 30); // ~12,000 nodes
        b.iter(|| {
            let mut count = 0;
            traverse_tree(&tree, &mut count);
            count
        });
    });

    // Benchmark with expanded trees (worst case for viewport culling)
    group.bench_function("traverse_expanded_small_culled", |b| {
        let mut tree = generate_large_tree(3, 10);
        expand_all(&mut tree);
        let nodes = count_nodes(&tree);
        println!("Small expanded tree nodes: {}", nodes);

        b.iter(|| {
            let mut count = 0;
            traverse_visible_with_culling(&tree, &mut count, 0.0, 0.0, 600.0);
            count
        });
    });

    group.bench_function("traverse_expanded_medium_culled", |b| {
        let mut tree = generate_large_tree(4, 20);
        expand_all(&mut tree);
        let nodes = count_nodes(&tree);
        println!("Medium expanded tree nodes: {}", nodes);

        b.iter(|| {
            let mut count = 0;
            traverse_visible_with_culling(&tree, &mut count, 0.0, 0.0, 600.0);
            count
        });
    });

    group.bench_function("traverse_expanded_large_culled", |b| {
        let mut tree = generate_large_tree(5, 30);
        expand_all(&mut tree);
        let nodes = count_nodes(&tree);
        println!("Large expanded tree nodes: {}", nodes);

        b.iter(|| {
            let mut count = 0;
            traverse_visible_with_culling(&tree, &mut count, 0.0, 0.0, 600.0);
            count
        });
    });

    group.finish();
}

/// Simulate tree traversal without culling
fn traverse_tree(node: &TreeNode, count: &mut usize) {
    *count += 1;

    if node.is_dir && node.expanded {
        for child in &node.children {
            traverse_tree(child, count);
        }
    }
}

/// Simulate tree traversal with viewport culling
fn traverse_visible_with_culling(
    node: &TreeNode,
    count: &mut usize,
    current_y: f32,
    viewport_top: f32,
    viewport_bottom: f32,
) -> f32 {
    let item_height = 24.0; // Typical row height
    let item_bottom = current_y + item_height;

    // Check if visible
    if item_bottom >= viewport_top && current_y <= viewport_bottom {
        *count += 1;
    }

    let mut y = current_y + item_height;

    if node.is_dir && node.expanded {
        for child in &node.children {
            y = traverse_visible_with_culling(child, count, y, viewport_top, viewport_bottom);
        }
    }

    y
}

criterion_group!(benches, benchmark_tree_rendering);
criterion_main!(benches);
