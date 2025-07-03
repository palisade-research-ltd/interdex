#!/bin/bash

# Project Structure Analyzer for Solana/Anchor Projects
# This script generates a detailed project structure with dependency information

PROJECT_NAME=$(basename "$PWD")
OUTPUT_FILE="project_structure_analysis.txt"

echo "=== PROJECT STRUCTURE ANALYSIS FOR: $PROJECT_NAME ===" > "$OUTPUT_FILE"
echo "Generated on: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Function to print section headers
print_section() {
    echo "" >> "$OUTPUT_FILE"
    echo "========================================" >> "$OUTPUT_FILE"
    echo "$1" >> "$OUTPUT_FILE"
    echo "========================================" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
}

# 1. Basic project structure using tree
print_section "1. PROJECT DIRECTORY STRUCTURE"
if command -v tree &> /dev/null; then
    tree -a -I 'target|node_modules|.git' -L 4 >> "$OUTPUT_FILE"
else
    echo "Tree command not available. Using find instead:" >> "$OUTPUT_FILE"
    find . -type d -name "target" -prune -o -name "node_modules" -prune -o -name ".git" -prune -o -type f -print | head -50 >> "$OUTPUT_FILE"
fi

# 2. Find all Cargo.toml files
print_section "2. CARGO.TOML FILES FOUND"
find . -name "Cargo.toml" -not -path "./target/*" >> "$OUTPUT_FILE"

# 3. Analyze each Cargo.toml for Solana-related dependencies
print_section "3. SOLANA/ANCHOR DEPENDENCIES ANALYSIS"
find . -name "Cargo.toml" -not -path "./target/*" | while read -r cargo_file; do
    echo "--- File: $cargo_file ---" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # Extract package info
    if grep -q "^\[package\]" "$cargo_file"; then
        echo "Package Information:" >> "$OUTPUT_FILE"
        sed -n '/^\[package\]/,/^\[/p' "$cargo_file" | grep -E "^(name|version|edition)" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi
    
    # Extract workspace info if present
    if grep -q "^\[workspace\]" "$cargo_file"; then
        echo "Workspace Configuration:" >> "$OUTPUT_FILE"
        sed -n '/^\[workspace\]/,/^\[/p' "$cargo_file" | head -20 >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi
    
    # Extract Solana/Anchor related dependencies
    echo "Solana/Anchor Dependencies:" >> "$OUTPUT_FILE"
    grep -E "(anchor-|solana-)" "$cargo_file" | grep -v "^#" >> "$OUTPUT_FILE" 2>/dev/null || echo "No Solana/Anchor dependencies found" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # Extract dev-dependencies
    if grep -q "^\[dev-dependencies\]" "$cargo_file"; then
        echo "Dev Dependencies (Solana/Anchor related):" >> "$OUTPUT_FILE"
        sed -n '/^\[dev-dependencies\]/,/^\[/p' "$cargo_file" | grep -E "(anchor-|solana-)" >> "$OUTPUT_FILE" 2>/dev/null || echo "No Solana/Anchor dev dependencies found" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi
    
    echo "---" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
done

# 4. Check Cargo.lock for actual resolved versions
print_section "4. RESOLVED DEPENDENCY VERSIONS (from Cargo.lock)"
if [ -f "Cargo.lock" ]; then
    echo "Solana-related packages in Cargo.lock:" >> "$OUTPUT_FILE"
    grep -A 2 -E "^name = \"(anchor-|solana-)" Cargo.lock | grep -E "^(name|version)" >> "$OUTPUT_FILE"
else
    echo "Cargo.lock not found" >> "$OUTPUT_FILE"
fi

# 5. Check for Anchor.toml configuration
print_section "5. ANCHOR CONFIGURATION"
if [ -f "Anchor.toml" ]; then
    echo "Anchor.toml contents:" >> "$OUTPUT_FILE"
    cat Anchor.toml >> "$OUTPUT_FILE"
else
    echo "Anchor.toml not found" >> "$OUTPUT_FILE"
fi

# 6. Check installed versions of tools
print_section "6. INSTALLED TOOL VERSIONS"
echo "Anchor CLI version:" >> "$OUTPUT_FILE"
anchor --version >> "$OUTPUT_FILE" 2>&1 || echo "Anchor CLI not installed" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "Solana CLI version:" >> "$OUTPUT_FILE"
solana --version >> "$OUTPUT_FILE" 2>&1 || echo "Solana CLI not installed" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "Rust version:" >> "$OUTPUT_FILE"
rustc --version >> "$OUTPUT_FILE" 2>&1 || echo "Rust not installed" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "Cargo version:" >> "$OUTPUT_FILE"
cargo --version >> "$OUTPUT_FILE" 2>&1 || echo "Cargo not installed" >> "$OUTPUT_FILE"

# 7. Check for common problematic patterns
print_section "7. POTENTIAL ISSUES DETECTED"
echo "Checking for common version conflict patterns..." >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Check for mixed Solana versions
if [ -f "Cargo.lock" ]; then
    echo "Different Solana SDK versions found:" >> "$OUTPUT_FILE"
    grep "^name = \"solana-" Cargo.lock | sort | uniq >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    echo "Different Anchor versions found:" >> "$OUTPUT_FILE"
    grep "^name = \"anchor-" Cargo.lock | sort | uniq >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
fi

# Check for workspace issues
workspace_files=$(find . -name "Cargo.toml" -not -path "./target/*" -exec grep -l "workspace" {} \;)
if [ -n "$workspace_files" ]; then
    echo "Files with workspace configuration:" >> "$OUTPUT_FILE"
    echo "$workspace_files" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
fi

# 8. Generate dependency tree if available
print_section "8. DEPENDENCY TREE (if available)"
if command -v cargo &> /dev/null; then
    echo "Generating dependency tree for Solana/Anchor crates..." >> "$OUTPUT_FILE"
    cargo tree --package-id solana 2>/dev/null | head -50 >> "$OUTPUT_FILE" || echo "Could not generate dependency tree" >> "$OUTPUT_FILE"
fi

echo "" >> "$OUTPUT_FILE"
echo "=== ANALYSIS COMPLETE ===" >> "$OUTPUT_FILE"
echo "Output saved to: $OUTPUT_FILE" >> "$OUTPUT_FILE"

# Display completion message
echo "Project structure analysis complete!"
echo "Results saved to: $OUTPUT_FILE"
echo ""
echo "You can now share this file to get help with version conflicts."
echo "Key sections to review:"
echo "- Section 3: Dependency versions in Cargo.toml files"
echo "- Section 4: Resolved versions in Cargo.lock"
echo "- Section 7: Potential issues detected"
