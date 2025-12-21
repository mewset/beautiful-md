#!/usr/bin/env bash
# Local CI checks - runs the same checks as GitHub Actions

set -e

echo "🔍 Running local CI checks..."
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print section headers
print_header() {
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}$1${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
    echo ""
}

# Function to print error
print_error() {
    echo -e "${RED}✗ $1${NC}"
    echo ""
}

# Check 1: Format check
print_header "1. Checking code formatting (cargo fmt)"
if cargo fmt --all -- --check; then
    print_success "Code formatting is correct"
else
    print_error "Code formatting failed"
    echo "Run 'cargo fmt --all' to fix formatting issues"
    exit 1
fi

# Check 2: Clippy lints
print_header "2. Running clippy lints"
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy checks passed"
else
    print_error "Clippy checks failed"
    echo "Fix the clippy warnings above"
    exit 1
fi

# Check 3: Tests
print_header "3. Running tests"
if cargo test --verbose; then
    print_success "All tests passed"
else
    print_error "Tests failed"
    exit 1
fi

# Check 4: Documentation build
print_header "4. Building documentation"
if RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features; then
    print_success "Documentation built successfully"
else
    print_error "Documentation build failed"
    exit 1
fi

# Check 5: Build
print_header "5. Building release binary"
if cargo build --release; then
    print_success "Release build successful"
else
    print_error "Release build failed"
    exit 1
fi

# All checks passed
echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ All checks passed! Ready to push to CI${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
