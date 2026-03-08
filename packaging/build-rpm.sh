#!/bin/bash
set -e

PROJECT="containertyrant"
VERSION="0.1.0"
ARCHIVE="${PROJECT}-${VERSION}.tar.gz"

# Get the project root directory (parent of packaging/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_ROOT}"

echo "=== Building RPM for ${PROJECT} ${VERSION} ==="
echo "Project root: ${PROJECT_ROOT}"

# Create source tarball
echo "Creating source tarball..."
tar --transform "s,^,${PROJECT}-${VERSION}/," \
    --exclude='target' \
    --exclude='.git' \
    --exclude='*.tar.gz' \
    --exclude='packaging/rpmbuild' \
    -czf "${ARCHIVE}" \
    src Cargo.toml Cargo.lock README.md LICENSE packaging

# Setup RPM build directories
echo "Setting up RPM build environment..."
RPMBUILD="${PROJECT_ROOT}/packaging/rpmbuild"
mkdir -p "${RPMBUILD}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Copy source tarball to SOURCES
cp "${ARCHIVE}" "${RPMBUILD}/SOURCES/"

# Copy spec file to SPECS
cp packaging/containertyrant.spec "${RPMBUILD}/SPECS/"

# Build the RPM
echo "Building RPM..."
rpmbuild -bb \
    --define "_topdir ${RPMBUILD}" \
    "${RPMBUILD}/SPECS/containertyrant.spec"

# Find and display the built RPM
RPM_PATH=$(find "${RPMBUILD}/RPMS" -name "*.rpm" | head -1)
if [ -n "$RPM_PATH" ]; then
    echo ""
    echo "=== Build successful! ==="
    echo "RPM location: ${RPM_PATH}"
    cp "${RPM_PATH}" "${PROJECT_ROOT}/"
    echo "Copied to: ${PROJECT_ROOT}/$(basename ${RPM_PATH})"
    ls -lh "${PROJECT_ROOT}/$(basename ${RPM_PATH})"
else
    echo "ERROR: RPM not found!"
    exit 1
fi
