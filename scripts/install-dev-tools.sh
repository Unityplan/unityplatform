#!/bin/bash

set -e

echo "🔧 UnityPlan Development Tools Installation"
echo "==========================================="
echo ""
echo "This script will install all necessary development tools for the UnityPlan platform:"
echo "  - Rust toolchain (rustup, cargo)"
echo "  - SQLx CLI (database migrations)"
echo "  - Node.js & npm (for frontend development)"
echo "  - Go 1.24+ (for Forgejo MCP server)"
echo "  - Forgejo MCP server (AI development integration)"
echo "  - Docker & Docker Compose (if not installed)"
echo "  - PostgreSQL client tools"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_section() {
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    print_error "Please do not run this script as root"
    echo "Run without sudo: ./scripts/install-dev-tools.sh"
    exit 1
fi

# ============================================================================
# 1. Docker & Docker Compose
# ============================================================================
print_section "1. Docker & Docker Compose"

if command_exists docker; then
    DOCKER_VERSION=$(docker --version)
    print_success "Docker already installed: $DOCKER_VERSION"
else
    print_warning "Docker not found. Installing Docker..."
    
    # Install Docker using official script
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    rm get-docker.sh
    
    # Add current user to docker group
    sudo usermod -aG docker $USER
    
    print_success "Docker installed successfully"
    print_warning "You need to log out and back in for group changes to take effect"
fi

if command_exists docker && docker compose version >/dev/null 2>&1; then
    COMPOSE_VERSION=$(docker compose version)
    print_success "Docker Compose already installed: $COMPOSE_VERSION"
else
    print_warning "Docker Compose not found or outdated"
    print_warning "Please install Docker Compose v2: https://docs.docker.com/compose/install/"
fi

# ============================================================================
# 2. Build Tools (Required for Rust)
# ============================================================================
print_section "2. Build Tools (gcc, g++, make)"

if command_exists gcc && command_exists g++; then
    print_success "Build tools already installed"
else
    print_warning "Build tools not found. Installing build-essential..."
    sudo apt-get update
    sudo apt-get install -y build-essential
    print_success "Build tools installed"
fi

# ============================================================================
# 3. Rust Toolchain
# ============================================================================
print_section "3. Rust Toolchain (rustup, cargo)"

if command_exists rustc && command_exists cargo; then
    RUST_VERSION=$(rustc --version)
    CARGO_VERSION=$(cargo --version)
    print_success "Rust already installed: $RUST_VERSION"
    print_success "Cargo already installed: $CARGO_VERSION"
else
    print_warning "Rust not found. Installing via rustup..."
    
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo environment
    source "$HOME/.cargo/env"
    
    print_success "Rust installed successfully"
    rustc --version
    cargo --version
fi

# Ensure cargo environment is available
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# ============================================================================
# 4. SQLx CLI
# ============================================================================
print_section "4. SQLx CLI (Database Migrations)"

if command_exists sqlx; then
    SQLX_VERSION=$(sqlx --version)
    print_success "SQLx CLI already installed: $SQLX_VERSION"
else
    print_warning "SQLx CLI not found. Installing..."
    
    cargo install sqlx-cli --no-default-features --features postgres
    
    print_success "SQLx CLI installed successfully"
    sqlx --version
fi

# ============================================================================
# 5. PostgreSQL Client Tools
# ============================================================================
print_section "5. PostgreSQL Client Tools"

if command_exists psql; then
    PSQL_VERSION=$(psql --version)
    print_success "PostgreSQL client already installed: $PSQL_VERSION"
else
    print_warning "PostgreSQL client not found. Installing..."
    
    sudo apt-get update
    sudo apt-get install -y postgresql-client
    
    print_success "PostgreSQL client installed successfully"
fi

# ============================================================================
# 6. Node.js & npm
# ============================================================================
print_section "6. Node.js & npm (Frontend Development)"

if command_exists node && command_exists npm; then
    NODE_VERSION=$(node --version)
    NPM_VERSION=$(npm --version)
    print_success "Node.js already installed: $NODE_VERSION"
    print_success "npm already installed: $NPM_VERSION"
    
    # Check if Node version is at least v20
    NODE_MAJOR=$(node --version | cut -d'.' -f1 | sed 's/v//')
    if [ "$NODE_MAJOR" -lt 20 ]; then
        print_warning "Node.js version is below v20. Consider upgrading to v20+"
        echo "  Install nvm: https://github.com/nvm-sh/nvm"
        echo "  Then run: nvm install 20 && nvm use 20"
    fi
else
    print_warning "Node.js not found. Installing via NodeSource..."
    
    # Install Node.js 20.x from NodeSource
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
    
    print_success "Node.js installed successfully"
    node --version
    npm --version
fi

# ============================================================================
# 7. Go (for Forgejo MCP server)
# ============================================================================
print_section "7. Go Programming Language (for Forgejo MCP)"

if command_exists go; then
    GO_VERSION=$(go version)
    print_success "Go already installed: $GO_VERSION"
    
    # Check if Go version is at least 1.24
    GO_VERSION_NUM=$(go version | awk '{print $3}' | sed 's/go//')
    GO_MAJOR=$(echo $GO_VERSION_NUM | cut -d'.' -f1)
    GO_MINOR=$(echo $GO_VERSION_NUM | cut -d'.' -f2)
    
    if [ "$GO_MAJOR" -lt 1 ] || ([ "$GO_MAJOR" -eq 1 ] && [ "$GO_MINOR" -lt 24 ]); then
        print_warning "Go version is below 1.24. Upgrading recommended for Forgejo MCP..."
        echo "  Manual upgrade: https://go.dev/dl/"
    fi
else
    print_warning "Go not found. Installing Go 1.24.0..."
    
    # Download and install Go
    cd /tmp
    wget https://go.dev/dl/go1.24rc2.linux-amd64.tar.gz
    sudo rm -rf /usr/local/go
    sudo tar -C /usr/local -xzf go1.24rc2.linux-amd64.tar.gz
    rm go1.24rc2.linux-amd64.tar.gz
    
    # Add to PATH for current session
    export PATH=$PATH:/usr/local/go/bin
    
    # Add to shell profile
    if ! grep -q "/usr/local/go/bin" ~/.bashrc; then
        echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
    fi
    
    print_success "Go installed successfully"
    go version
fi

# ============================================================================
# 8. Forgejo MCP Server
# ============================================================================
print_section "8. Forgejo MCP Server (AI Development Integration)"

if command_exists forgejo-mcp; then
    print_success "Forgejo MCP server already installed"
    forgejo-mcp --version 2>&1 | head -n 1
else
    print_warning "Forgejo MCP server not found. Building from source..."
    
    if ! command_exists go; then
        print_error "Go is required to build Forgejo MCP. Install Go first."
    else
        # Clone and build
        cd /tmp
        if [ -d "forgejo-mcp" ]; then
            rm -rf forgejo-mcp
        fi
        
        git clone https://codeberg.org/goern/forgejo-mcp.git
        cd forgejo-mcp
        make build
        
        # Install to system PATH
        sudo cp forgejo-mcp /usr/local/bin/
        
        # Clean up
        cd /tmp
        rm -rf forgejo-mcp
        
        print_success "Forgejo MCP server installed successfully"
        forgejo-mcp --version 2>&1 | head -n 1
        
        echo ""
        echo "  ℹ️  To configure Forgejo MCP:"
        echo "     1. Generate access token at http://192.168.60.133:3000/user/settings/applications"
        echo "     2. Create ~/.config/mcp/forgejo.json with server configuration"
        echo "     3. See docs-archived/forgejo-mcp-setup.md for details"
    fi
fi

# ============================================================================
# 9. Additional Development Tools
# ============================================================================
print_section "9. Additional Development Tools"

# Git
if command_exists git; then
    GIT_VERSION=$(git --version)
    print_success "Git already installed: $GIT_VERSION"
else
    print_warning "Git not found. Installing..."
    sudo apt-get install -y git
    print_success "Git installed"
fi

# curl
if command_exists curl; then
    print_success "curl already installed"
else
    print_warning "curl not found. Installing..."
    sudo apt-get install -y curl
    print_success "curl installed"
fi

# jq (JSON processor)
if command_exists jq; then
    print_success "jq already installed"
else
    print_warning "jq not found. Installing..."
    sudo apt-get install -y jq
    print_success "jq installed"
fi

# ============================================================================
# 10. Optional Tools
# ============================================================================
print_section "10. Optional Development Tools"

echo ""
echo "The following tools are optional but recommended:"
echo ""

# bat (better cat)
if command_exists bat || command_exists batcat; then
    print_success "bat (better cat) already installed"
else
    echo "  📦 bat - Better 'cat' with syntax highlighting"
    echo "     Install: sudo apt-get install -y bat"
fi

# ripgrep (better grep)
if command_exists rg; then
    print_success "ripgrep already installed"
else
    echo "  📦 ripgrep - Better 'grep' for code search"
    echo "     Install: sudo apt-get install -y ripgrep"
fi

# fd (better find)
if command_exists fd; then
    print_success "fd already installed"
else
    echo "  📦 fd - Better 'find' for files"
    echo "     Install: sudo apt-get install -y fd-find"
fi

# ============================================================================
# Summary
# ============================================================================
print_section "Installation Summary"

echo ""
echo "Core Tools:"
echo "  Docker:       $(if command_exists docker; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Docker Compose: $(if docker compose version >/dev/null 2>&1; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Rust:         $(if command_exists rustc; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Cargo:        $(if command_exists cargo; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  SQLx CLI:     $(if command_exists sqlx; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  PostgreSQL:   $(if command_exists psql; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Node.js:      $(if command_exists node; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  npm:          $(if command_exists npm; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Go:           $(if command_exists go; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Forgejo MCP:  $(if command_exists forgejo-mcp; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo "  Git:          $(if command_exists git; then echo "✅ Installed"; else echo "❌ Not installed"; fi)"
echo ""

print_section "Next Steps"

echo ""
echo "1. If Docker was just installed, log out and back in to apply group changes:"
echo "   logout"
echo ""
echo "2. Verify Rust installation:"
echo "   source \$HOME/.cargo/env"
echo "   rustc --version"
echo "   cargo --version"
echo ""
echo "3. Start the development environment:"
echo "   ./scripts/start-dev.sh"
echo ""
echo "4. Start the Denmark pod:"
echo "   docker compose -f docker-compose.pod.yml -p pod-dk --env-file pods/denmark/.env up -d"
echo ""
echo "5. Run database migrations:"
echo "   cd services"
echo "   export DATABASE_URL=\"postgres://unityplan:unityplan_dev_password@localhost:5432/unityplan_dk\""
echo "   sqlx migrate run"
echo ""
echo "6. Build and test Rust services:"
echo "   cd services"
echo "   cargo build"
echo "   cargo test"
echo ""
echo "7. Read the documentation:"
echo "   docs/README.md"
echo "   docs/guides/development/"
echo ""

print_success "Development tools installation complete!"
echo ""
