# ContainerTyrant

**Rule your Docker containers with an iron fist.**

A powerful container management utility built with Rust and Iced. ContainerTyrant gives you absolute command over your container empire with a clean, efficient interface.

## Features

- **Absolute Control**: Start, stop, and restart containers with a single click
- **Real-time Intelligence**: Live status updates and container metrics
- **Search & Filter**: Quickly locate subjects in your container kingdom
- **Export Capabilities**: Export container data as JSON or CSV
- **Compose Generation**: Generate docker-compose.yml from running containers
- **Dark Theme**: Easy on the eyes during long campaigns

## Requirements

- Linux (tested on Fedora)
- Docker daemon running
- Rust 1.70+ (for building)

## Installation

### From RPM (Fedora)

```bash
sudo dnf install ./containertyrant-0.1.0-1.fc43.x86_64.rpm
```

### From Source

```bash
cargo build --release
sudo install -m 755 target/release/containertyrant /usr/local/bin/
```

## Usage

```bash
containertyrant
```

## Configuration

Configuration is stored at `~/.config/containertyrant/config.toml`:

```toml
theme = "dark"
refresh_interval_ms = 2000
show_system_containers = false
```

## Building the RPM

```bash
# Create tarball
tar --transform "s,^,containertyrant-0.1.0/," \
    --exclude='target' --exclude='.git' --exclude='*.tar.gz' \
    -czf containertyrant-0.1.0.tar.gz \
    src Cargo.toml Cargo.lock README.md LICENSE containertyrant-packaging

# Build
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cp containertyrant-0.1.0.tar.gz ~/rpmbuild/SOURCES/
cp containertyrant-packaging/containertyrant.spec ~/rpmbuild/SPECS/
rpmbuild -bb ~/rpmbuild/SPECS/containertyrant.spec
```

## License

MIT

---

*"All containers shall bow before the Tyrant."*
