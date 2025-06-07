#!/bin/bash
# Generate a test directory tree with many files for performance testing

TEST_DIR="test_tree_5000"

echo "Generating test directory tree with ~5000 files..."

# Remove existing test directory if it exists
rm -rf "$TEST_DIR"

# Create root directory
mkdir -p "$TEST_DIR"

# Function to create files in a directory
create_files() {
    local dir=$1
    local count=$2
    for i in $(seq 1 $count); do
        echo "// Test file $i" > "$dir/file_$i.rs"
    done
}

# Create a balanced tree structure
# Level 1: 20 directories
for i in $(seq 1 20); do
    mkdir -p "$TEST_DIR/module_$i"
    create_files "$TEST_DIR/module_$i" 50
    
    # Level 2: 10 subdirectories per directory
    for j in $(seq 1 10); do
        mkdir -p "$TEST_DIR/module_$i/submodule_$j"
        create_files "$TEST_DIR/module_$i/submodule_$j" 20
        
        # Level 3: 2 subdirectories per subdirectory
        for k in $(seq 1 2); do
            mkdir -p "$TEST_DIR/module_$i/submodule_$j/component_$k"
            create_files "$TEST_DIR/module_$i/submodule_$j/component_$k" 5
        done
    done
done

# Count total files
total_files=$(find "$TEST_DIR" -type f | wc -l)
total_dirs=$(find "$TEST_DIR" -type d | wc -l)

echo "Created test tree:"
echo "  - Total files: $total_files"
echo "  - Total directories: $total_dirs"
echo "  - Total nodes: $((total_files + total_dirs))"
echo ""
echo "Test directory created at: $TEST_DIR"
echo "You can now test fsPrompt with this directory!"