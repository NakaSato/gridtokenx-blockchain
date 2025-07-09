# Development Setup Guide

## Prerequisites

Before you begin developing with GridTokenX, ensure you have the following installed:

### Required Software

1. **Rust** (version 1.88+)
2. **Cargo** (comes with Rust)
3. **Node.js** (version 14+)
4. **Git**
5. **Docker** (optional, for containerized development)

### Substrate-specific Requirements

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure current shell
source ~/.cargo/env

# Install nightly toolchain
rustup update nightly

# Add WebAssembly target
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Project Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-org/gridtokenx-blockchain.git
cd gridtokenx-blockchain
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# For development with faster builds
cargo build --dev
```

### 3. Build the Project

```bash
# Production build
cargo build --release

# Development build (faster compilation)
cargo build
```

## Development Environment

### IDE Setup

#### VS Code (Recommended)

Install the following extensions:

1. **rust-analyzer** - Rust language server
2. **Better TOML** - TOML file support
3. **GitLens** - Git integration
4. **Substrate** - Substrate-specific features

#### VS Code Settings

Create `.vscode/settings.json`:

```json
{
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.procMacro.enable": true,
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

### Environment Variables

Create a `.env` file in the project root:

```bash
# Development settings
RUST_LOG=debug
RUST_BACKTRACE=1

# Node configuration
NODE_ENV=development
CHAIN_SPEC=dev

# Database
DATABASE_URL=substrate_db
```

## Running the Node

### Development Mode

```bash
# Run node in development mode
cargo run -- --dev

# Or use the built binary
./target/release/solar-grid-node --dev
```

### Custom Chain Specification

```bash
# Generate chain specification
cargo run -- build-spec --chain dev > chain-spec.json

# Convert to raw format
cargo run -- build-spec --chain chain-spec.json --raw > chain-spec-raw.json

# Run with custom chain spec
cargo run -- --chain chain-spec-raw.json
```

### Network Configuration

#### Local Network

```bash
# Alice (validator)
cargo run -- --alice --validator --rpc-cors all

# Bob (validator)
cargo run -- --bob --validator --rpc-cors all --port 30334 --rpc-port 9945
```

#### Multi-node Setup

```bash
# Node 1
cargo run -- --chain chain-spec-raw.json --alice --validator --rpc-cors all

# Node 2
cargo run -- --chain chain-spec-raw.json --bob --validator --rpc-cors all --port 30334 --rpc-port 9945

# Node 3 (non-validator)
cargo run -- --chain chain-spec-raw.json --charlie --rpc-cors all --port 30335 --rpc-port 9946
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run tests for specific pallet
cargo test -p pallet-energy-token

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_mint_tokens_works
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration

# Run with specific features
cargo test --features runtime-benchmarks
```

### Test Coverage

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out html

# Generate coverage for specific package
cargo tarpaulin -p pallet-energy-token --out html
```

## Debugging

### Debug Mode

```bash
# Build with debug symbols
cargo build --dev

# Run with debug logging
RUST_LOG=debug cargo run -- --dev
```

### Logging Configuration

```rust
// Add to main.rs or service.rs
use log::{info, debug, error};

// Configure logger
env_logger::init();

// Use in code
info!("Node starting...");
debug!("Debug information: {:?}", data);
error!("Error occurred: {}", error);
```

### Profiling

```bash
# Install profiling tools
cargo install flamegraph
cargo install cargo-profiler

# Generate flame graph
cargo flamegraph -- --dev

# Profile specific function
cargo profiler callgrind --bin solar-grid-node -- --dev
```

## Code Quality

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Run clippy with specific target
cargo clippy --target wasm32-unknown-unknown
```

### Security Audit

```bash
# Install cargo-audit
cargo install cargo-audit

# Run security audit
cargo audit

# Update vulnerable dependencies
cargo audit fix
```

## Benchmarking

### Runtime Benchmarks

```bash
# Enable benchmarking features
cargo build --release --features runtime-benchmarks

# Run benchmarks
cargo run --release --features runtime-benchmarks -- benchmark pallet --pallet pallet_energy_token --extrinsic "*"
```

### Performance Testing

```bash
# Install criterion for benchmarks
cargo install cargo-criterion

# Run performance tests
cargo criterion
```

## Database Management

### Substrate Database

```bash
# Clear database
rm -rf /tmp/substrate-node

# Backup database
cp -r /tmp/substrate-node ~/substrate-backup

# Restore database
cp -r ~/substrate-backup /tmp/substrate-node
```

### Database Inspection

```bash
# Install substrate-inspect
cargo install substrate-inspect

# Inspect database
substrate-inspect --db-path /tmp/substrate-node
```

## Deployment

### Docker Development

Create `Dockerfile.dev`:

```dockerfile
FROM rust:1.88

WORKDIR /app

# Install dependencies
RUN rustup update nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

# Copy source code
COPY . .

# Build project
RUN cargo build --release

# Expose ports
EXPOSE 9944 9933 30333

# Run node
CMD ["./target/release/solar-grid-node", "--dev", "--ws-external"]
```

Build and run:

```bash
# Build Docker image
docker build -f Dockerfile.dev -t gridtokenx-dev .

# Run container
docker run -p 9944:9944 -p 9933:9933 -p 30333:30333 gridtokenx-dev
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  node:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - "9944:9944"
      - "9933:9933"
      - "30333:30333"
    environment:
      - RUST_LOG=debug
    volumes:
      - ./:/app
      - cargo-cache:/usr/local/cargo/registry
      - target-cache:/app/target

volumes:
  cargo-cache:
  target-cache:
```

Run with Docker Compose:

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## Frontend Development

### Connect to Node

```javascript
// Using Polkadot.js API
import { ApiPromise, WsProvider } from '@polkadot/api';

const provider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider });

// Query energy token balance
const balance = await api.query.energyToken.tokenBalance(account);
console.log('Token balance:', balance.toString());
```

### Frontend Setup

```bash
# Create React app
npx create-react-app gridtokenx-frontend
cd gridtokenx-frontend

# Install Polkadot.js dependencies
npm install @polkadot/api @polkadot/extension-dapp @polkadot/ui-keyring

# Start development server
npm start
```

## Continuous Integration

### GitHub Actions

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Cache cargo
      uses: actions/cache@v3
      with:
        path: ~/.cargo
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check format
      run: cargo fmt -- --check
    
    - name: Clippy
      run: cargo clippy -- -D warnings
    
    - name: Test
      run: cargo test --all
    
    - name: Build
      run: cargo build --release
```

### Pre-commit Hooks

Install pre-commit hooks:

```bash
# Install pre-commit
pip install pre-commit

# Create .pre-commit-config.yaml
cat > .pre-commit-config.yaml << EOF
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --
        language: system
        types: [rust]
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --
        language: system
        types: [rust]
        args: ['-D', 'warnings']
EOF

# Install hooks
pre-commit install
```

## Troubleshooting

### Common Issues

#### 1. WebAssembly Target Missing

**Error**: `error: the 'wasm32-unknown-unknown' target may not be installed`

**Solution**:
```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
```

#### 2. Compilation Errors

**Error**: `error: linking with 'cc' failed`

**Solution**:
```bash
# On Ubuntu/Debian
sudo apt-get install build-essential

# On macOS
xcode-select --install

# On Windows
# Install Visual Studio Build Tools
```

#### 3. Node Connection Issues

**Error**: `Unable to connect to node`

**Solution**:
```bash
# Check if node is running
ps aux | grep solar-grid-node

# Check port availability
netstat -tulpn | grep 9944

# Restart node with correct flags
cargo run -- --dev --ws-external --rpc-cors all
```

#### 4. Database Corruption

**Error**: `Database corruption detected`

**Solution**:
```bash
# Remove database and restart
rm -rf /tmp/substrate-node
cargo run -- --dev
```

### Performance Issues

#### Slow Compilation

```bash
# Use faster linker
echo '[target.x86_64-unknown-linux-gnu]' >> ~/.cargo/config
echo 'linker = "clang"' >> ~/.cargo/config
echo 'rustflags = ["-C", "link-arg=-fuse-ld=lld"]' >> ~/.cargo/config

# Use cargo-chef for Docker builds
# Enable incremental compilation
export CARGO_INCREMENTAL=1
```

#### Memory Issues

```bash
# Increase memory limit
export CARGO_BUILD_JOBS=2
export RUSTC_WRAPPER=sccache

# Use release profile for testing
cargo test --release
```

## Development Workflow

### Branch Strategy

```bash
# Create feature branch
git checkout -b feature/new-pallet

# Regular commits
git add .
git commit -m "feat: add new pallet functionality"

# Push to remote
git push origin feature/new-pallet

# Create pull request
# ... (use GitHub/GitLab interface)
```

### Code Review Process

1. **Self Review**: Check code quality before PR
2. **Automated Tests**: Ensure all tests pass
3. **Peer Review**: Request review from team members
4. **Integration Tests**: Run on staging environment
5. **Merge**: Merge to main branch

### Release Process

```bash
# Create release branch
git checkout -b release/v1.0.0

# Update version numbers
# ... update Cargo.toml files

# Build release
cargo build --release

# Tag release
git tag -a v1.0.0 -m "Release version 1.0.0"

# Push tag
git push origin v1.0.0
```

## Resources

### Documentation

- [Substrate Developer Hub](https://substrate.dev/)
- [Polkadot.js Documentation](https://polkadot.js.org/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)

### Community

- [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- [Polkadot Discord](https://discord.gg/polkadot)
- [Substrate GitHub](https://github.com/paritytech/substrate)

### Tools

- [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
- [Polkadot.js Apps](https://polkadot.js.org/apps/)
- [Substrate Front-end Template](https://github.com/substrate-developer-hub/substrate-front-end-template)

---

This setup guide should get you started with GridTokenX development. For specific issues, refer to the troubleshooting section or reach out to the development team.
