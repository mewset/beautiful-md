#!/bin/bash
# Generate a large markdown file (10MB+) for performance testing

OUTPUT_FILE="${1:-large_test.md}"
TARGET_SIZE_MB="${2:-10}"

echo "Generating large markdown file: $OUTPUT_FILE"
echo "Target size: ${TARGET_SIZE_MB}MB"

# Clear file
> "$OUTPUT_FILE"

# Calculate iterations needed (approximate)
# Each iteration generates ~420 bytes, so for 10MB we need ~25000 iterations
ITERATIONS=$((TARGET_SIZE_MB * 2500))

echo "Generating content with $ITERATIONS iterations..."

# Header
cat >> "$OUTPUT_FILE" << 'EOF'
# Large Markdown Test File

This file was auto-generated for performance testing of beautiful-md.

**Table of Contents:**
- Tables with various structures
- Headings at different levels
- Lists (ordered and unordered)
- Code blocks in multiple languages
- Mixed content

---

EOF

# Generate varied content
for i in $(seq 1 $ITERATIONS); do
    SECTION=$((i % 10))

    case $SECTION in
        0)
            # Large table
            cat >> "$OUTPUT_FILE" << EOF

## Section $i - Data Table

| ID | Name | Email | Age | City | Country | Status |
|---|---|---|---|---|---|---|
| $i | User $i | user$i@example.com | $((20 + i % 50)) | City$((i % 100)) | Country$((i % 20)) | Active |
| $((i+1)) | User $((i+1)) | user$((i+1))@example.com | $((25 + i % 45)) | City$((i % 95)) | Country$((i % 18)) | Pending |
| $((i+2)) | User $((i+2)) | user$((i+2))@example.com | $((30 + i % 40)) | City$((i % 90)) | Country$((i % 16)) | Inactive |

EOF
            ;;
        1)
            # Code block with comments
            cat >> "$OUTPUT_FILE" << EOF

## Section $i - Code Example

\`\`\`bash
#!/bin/bash
# This is iteration $i
# Testing code block preservation

export VAR_$i="value"

function process_data_$i() {
    # Process data with iteration $i
    echo "Processing batch $i"

    # Multiple comment lines to test
    # Comment line 2
    # Comment line 3

    for j in {1..10}; do
        echo "Item \$j in iteration $i"
    done
}

process_data_$i
\`\`\`

EOF
            ;;
        2)
            # Python code
            cat >> "$OUTPUT_FILE" << EOF

### Section $i - Python Implementation

\`\`\`python
class DataProcessor$i:
    """Process data for iteration $i"""

    def __init__(self):
        # Initialize with iteration $i
        self.iteration = $i
        self.data = []

    def process(self, items):
        # Process each item
        for item in items:
            # Transform data
            result = self.transform(item)
            self.data.append(result)

    def transform(self, item):
        # Apply transformation logic
        return item * 2
\`\`\`

EOF
            ;;
        3)
            # Lists
            cat >> "$OUTPUT_FILE" << EOF

### Section $i - Task List

**Completed Tasks:**
- Task $i.1: Initial setup
- Task $i.2: Configuration
- Task $i.3: Implementation
- Task $i.4: Testing
- Task $i.5: Documentation

**Pending Tasks:**
1. Review iteration $i
2. Validate results for batch $((i + 1))
3. Update documentation section $i
4. Run performance tests
5. Deploy to staging

EOF
            ;;
        4)
            # Mixed content with inline code
            cat >> "$OUTPUT_FILE" << EOF

## Section $i - Technical Overview

This section covers iteration **$i** of the processing pipeline.

Key points:
- The \`process_$i()\` function handles data transformation
- Configuration is stored in \`config_$i.toml\`
- Output files: \`output_$i.json\`, \`logs_$i.txt\`
- Performance target: <100ms for iteration $i

**Important:** Always validate input before calling \`execute_$i()\`.

EOF
            ;;
        5)
            # Nested lists
            cat >> "$OUTPUT_FILE" << EOF

### Section $i - Hierarchical Structure

- Level 1 - Item $i
  - Level 2 - Subitem $i.1
    - Level 3 - Details $i.1.1
    - Level 3 - Details $i.1.2
  - Level 2 - Subitem $i.2
- Level 1 - Item $((i+1))
  - Level 2 - Subitem $((i+1)).1

EOF
            ;;
        6)
            # JavaScript code
            cat >> "$OUTPUT_FILE" << EOF

## Section $i - JavaScript Example

\`\`\`javascript
// Iteration $i implementation
class Handler$i {
  constructor() {
    // Initialize handler for iteration $i
    this.id = $i;
    this.data = new Map();
  }

  async process(items) {
    // Process items asynchronously
    for (const item of items) {
      // Transform and store
      const result = await this.transform(item);
      this.data.set(item.id, result);
    }
  }

  // Helper method for iteration $i
  transform(item) {
    return {
      ...item,
      iteration: $i,
      timestamp: Date.now()
    };
  }
}
\`\`\`

EOF
            ;;
        7)
            # Multiple headings
            cat >> "$OUTPUT_FILE" << EOF

# Major Section $i

## Subsection $i.1

### Details $i.1.1

Content for section $i with multiple heading levels.

#### Deep Heading $i.1.1.1

Even deeper content here in iteration $i.

##### Very Deep $i.1.1.1.1

This is rarely used but should still format correctly.

###### Maximum Depth $i.1.1.1.1.1

The deepest heading level in iteration $i.

EOF
            ;;
        8)
            # Complex table
            cat >> "$OUTPUT_FILE" << EOF

## Section $i - Complex Data

| Metric | Q1 | Q2 | Q3 | Q4 | Total | Change |
|--------|----|----|----|----|-------|--------|
| Revenue ($i) | \$${i}00 | \$${i}50 | \$${i}75 | \$${i}99 | \$${i}324 | +$((i % 30))% |
| Users | ${i}000 | $((i+1))000 | $((i+2))000 | $((i+3))000 | $((i+6))000 | +$((i % 50))% |
| Conversion | $((i % 10)).$((i % 99))% | $((i % 11)).$((i % 89))% | $((i % 12)).$((i % 79))% | $((i % 13)).$((i % 69))% | $((i % 14)).$((i % 59))% | +$((i % 5))% |

EOF
            ;;
        9)
            # Rust code
            cat >> "$OUTPUT_FILE" << EOF

### Section $i - Rust Implementation

\`\`\`rust
/// Process data for iteration $i
pub fn process_iteration_$i(data: &[u8]) -> Result<Vec<u8>> {
    // Validate input for iteration $i
    if data.is_empty() {
        return Err(Error::EmptyInput);
    }

    // Transform data
    let mut result = Vec::new();
    for byte in data {
        // Apply transformation logic
        result.push(byte.wrapping_add($((i % 256)) as u8));
    }

    Ok(result)
}

#[cfg(test)]
mod tests_$i {
    use super::*;

    #[test]
    fn test_iteration_$i() {
        let input = vec![1, 2, 3];
        let result = process_iteration_$i(&input);
        assert!(result.is_ok());
    }
}
\`\`\`

---

EOF
            ;;
    esac

    # Progress indicator
    if [ $((i % 100)) -eq 0 ]; then
        SIZE=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE")
        SIZE_MB=$((SIZE / 1024 / 1024))
        echo "Progress: $i/$ITERATIONS iterations, ${SIZE_MB}MB generated"
    fi

    # Stop if we've reached target size
    SIZE=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE")
    if [ $SIZE -gt $((TARGET_SIZE_MB * 1024 * 1024)) ]; then
        echo "Target size reached!"
        break
    fi
done

# Footer
cat >> "$OUTPUT_FILE" << 'EOF'

---

## Summary

This large markdown file was generated for performance testing.

**Purpose:**
- Test beautiful-md with large files (10MB+)
- Verify all formatters work at scale
- Measure performance characteristics
- Identify potential memory issues

**Content Includes:**
- Thousands of tables
- Multiple heading levels
- Nested lists
- Code blocks in bash, python, javascript, rust
- Mixed inline formatting

Generated automatically by generate_large_test.sh
EOF

# Final stats
SIZE=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE")
SIZE_MB=$(echo "scale=2; $SIZE / 1024 / 1024" | bc)
LINES=$(wc -l < "$OUTPUT_FILE")

echo ""
echo "✅ Generation complete!"
echo "   File: $OUTPUT_FILE"
echo "   Size: ${SIZE_MB}MB ($SIZE bytes)"
echo "   Lines: $LINES"
echo ""
echo "Test with:"
echo "  time ./target/release/beautiful-md --check $OUTPUT_FILE"
echo "  time ./target/release/beautiful-md --dry-run $OUTPUT_FILE"
echo "  time ./target/release/beautiful-md $OUTPUT_FILE > /tmp/formatted.md"
