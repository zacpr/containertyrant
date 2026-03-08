Name:           containertyrant
Version:        0.1.0
Release:        1%{?dist}
Summary:        Rule your Docker containers with an iron fist

License:        MIT
URL:            https://github.com/zac/containertyrant
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust >= 1.70
BuildRequires:  openssl-devel
BuildRequires:  pkgconfig(openssl)

Requires:       docker
Requires:       hicolor-icon-theme

%description
ContainerTyrant is a powerful container management utility that puts you 
in command. Built with Rust for performance and reliability, it provides 
a clean interface for controlling your Docker empire.

Features include:
- View all containers with status, image, and port information
- Start, stop, and restart containers
- Real-time status updates
- Search and filter containers
- Export container data as JSON or CSV
- Generate docker-compose.yml files

%prep
%setup -q

%build
cargo build --release

%install
install -Dpm0755 target/release/containertyrant %{buildroot}%{_bindir}/containertyrant
install -Dpm0644 packaging/containertyrant.desktop %{buildroot}%{_datadir}/applications/containertyrant.desktop
install -Dpm0644 packaging/containertyrant.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/containertyrant.svg
install -Dpm0644 packaging/containertyrant.metainfo.xml %{buildroot}%{_metainfodir}/containertyrant.metainfo.xml

%files
%license LICENSE
%doc README.md
%{_bindir}/containertyrant
%{_datadir}/applications/containertyrant.desktop
%{_datadir}/icons/hicolor/scalable/apps/containertyrant.svg
%{_metainfodir}/containertyrant.metainfo.xml

%changelog
* Sat Mar 08 2026 Zac <zac@ashurtech.net> - 0.1.0-1
- Initial release: The Tyranny Begins
