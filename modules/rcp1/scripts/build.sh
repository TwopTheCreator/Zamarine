#!/bin/bash

# build script bro..
set -e

# Configuration
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${PROJECT_DIR}/build"
DIST_DIR="${PROJECT_DIR}/dist"
JAVA_HOME=${JAVA_HOME:-/usr/lib/jvm/java-11-openjdk}
MAVEN_HOME=${MAVEN_HOME:-/usr/share/maven}

# Create necessary directories
mkdir -p "${BUILD_DIR}" "${DIST_DIR}"

# Clean build artifacts
clean() {
    echo "Cleaning build artifacts..."
    rm -rf "${BUILD_DIR}" "${DIST_DIR}" "${PROJECT_DIR}/target"
}

# Package the application
package() {
    echo "Building RCP1 module..."
    
    # Build Java code with Maven
    echo "Compiling Java sources..."
    "${MAVEN_HOME}/bin/mvn" clean package -DskipTests
    
    # Create distribution structure
    echo "Creating distribution package..."
    mkdir -p "${DIST_DIR}/lib"
    mkdir -p "${DIST_DIR}/config"
    mkdir -p "${DIST_DIR}/lua"
    
    # Copy artifacts
    cp "${PROJECT_DIR}/target/rcp1-1.0.0.jar" "${DIST_DIR}/lib/rcp1.jar"
    cp "${PROJECT_DIR}/config/rcp1.ini" "${DIST_DIR}/config/"
    cp "${PROJECT_DIR}/lua/"*.lua "${DIST_DIR}/lua/"
    
    # Create run script
    cat > "${DIST_DIR}/rcp1" << 'EOF'
#!/bin/bash
# RCP1 Launcher Script

# Resolve script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JAVA_OPTS="-Xms128m -Xmx256m"

# Run the application
exec java ${JAVA_OPTS} -jar "${SCRIPT_DIR}/lib/rcp1.jar" "$@"
EOF

    chmod +x "${DIST_DIR}/rcp1"
    
    echo "Build completed successfully. Distribution package created in ${DIST_DIR}"
}

# Install the application
install() {
    echo "Installing RCP1 module..."
    
    # Build first if needed
    if [ ! -d "${DIST_DIR}" ]; then
        package
    fi
    
    # Create installation directories
    INSTALL_DIR="/opt/zamarine/rcp1"
    CONFIG_DIR="/etc/zamarine/rcp1"
    
    echo "Installing to ${INSTALL_DIR}..."
    sudo mkdir -p "${INSTALL_DIR}" "${CONFIG_DIR}"
    
    # Copy files
    sudo cp -r "${DIST_DIR}/"* "${INSTALL_DIR}/"
    sudo cp "${PROJECT_DIR}/config/rcp1.ini" "${CONFIG_DIR}/"
    
    # Create symlink in /usr/local/bin
    echo "Creating symlink in /usr/local/bin..."
    sudo ln -sf "${INSTALL_DIR}/rcp1" "/usr/local/bin/rcp1"
    
    echo "Installation completed successfully. Run 'rcp1 --help' to get started."
}

# Main script
case "$1" in
    clean)
        clean
        ;;
    package)
        package
        ;;
    install)
        install
        ;;
    *)
        echo "Usage: $0 {clean|package|install}"
        exit 1
        ;;
esac

exit 0
