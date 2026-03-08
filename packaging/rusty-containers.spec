Name:           rusty-containers
Version:        0.1.0
Release:        1%{?dist}
Summary:        A modern Linux container management utility with GUI

License:        MIT
URL:            https://github.com/zac/rusty-containers
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust >= 1.70
BuildRequires:  openssl-devel
BuildRequires:  pkgconfig(openssl)

# Runtime dependencies
Requires:       docker
Requires:       hicolor-icon-theme

%description
Rusty Containers is a modern Linux container management utility built with Rust.
It provides a GUI for managing Docker containers with features including:
- View, start, stop, and restart containers
- Real-time status updates
- Search and filter containers
- Export container data as JSON or CSV
- Generate docker-compose.yml files

%prep
%setup -q

%build
cargo build --release

%install
# Install binary
install -Dpm0755 target/release/rusty-containers %{buildroot}%{_bindir}/rusty-containers

# Install desktop file
install -Dpm0644 packaging/rusty-containers.desktop %{buildroot}%{_datadir}/applications/rusty-containers.desktop

# Install icon (SVG scalable)
install -Dpm0644 packaging/rusty-containers.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/rusty-containers.svg

# Install appstream metadata
install -Dpm0644 packaging/rusty-containers.metainfo.xml %{buildroot}%{_metainfodir}/rusty-containers.metainfo.xml

%files
%license LICENSE
%doc README.md
%{_bindir}/rusty-containers
%{_datadir}/applications/rusty-containers.desktop
%{_datadir}/icons/hicolor/scalable/apps/rusty-containers.svg
%{_metainfodir}/rusty-containers.metainfo.xml

%changelog
* Sat Mar 08 2026 Zac <zac@ashurtech.net> - 0.1.0-1
- Initial package
