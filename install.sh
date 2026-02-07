#!/bin/bash
# Sentinel Installer for Linux/macOS
# Version: 1.0.0

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://github.com/sergiogswv/sentinel-rust.git"
INSTALL_DIR="$HOME/.sentinel"
BIN_NAME="sentinel"
BIN_PATH="/usr/local/bin/$BIN_NAME"

# Functions
print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  🛡️  Sentinel Installer${NC}"
    echo -e "${BLUE}================================${NC}\n"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${YELLOW}ℹ️  $1${NC}"
}

check_dependencies() {
    print_info "Verificando dependencias..."

    # Check Git
    if ! command -v git &> /dev/null; then
        print_error "Git no está instalado"
        echo "Por favor instala Git primero:"
        echo "  - Ubuntu/Debian: sudo apt-get install git"
        echo "  - macOS: brew install git"
        exit 1
    fi

    # Check Rust
    if ! command -v cargo &> /dev/null; then
        print_info "Rust no está instalado. Instalando Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust instalado correctamente"
    else
        print_success "Rust ya está instalado"
    fi
}

clone_or_update() {
    if [ -d "$INSTALL_DIR" ]; then
        print_info "Directorio de instalación encontrado. Actualizando..."
        cd "$INSTALL_DIR"

        # Save current branch
        CURRENT_BRANCH=$(git branch --show-current)

        # Fetch latest changes
        git fetch origin

        # Check if there are updates
        LOCAL=$(git rev-parse @)
        REMOTE=$(git rev-parse @{u})

        if [ "$LOCAL" = "$REMOTE" ]; then
            print_info "Ya tienes la última versión"
        else
            print_info "Nuevas actualizaciones disponibles. Descargando..."
            git pull origin "$CURRENT_BRANCH"
            print_success "Código actualizado correctamente"
        fi
    else
        print_info "Clonando repositorio..."
        git clone "$REPO_URL" "$INSTALL_DIR"
        cd "$INSTALL_DIR"
        print_success "Repositorio clonado correctamente"
    fi
}

build_project() {
    print_info "Compilando Sentinel (esto puede tomar unos minutos)..."
    cd "$INSTALL_DIR"
    cargo build --release
    print_success "Compilación exitosa"
}

install_binary() {
    print_info "Instalando binario..."

    # Check if we need sudo
    if [ -w "/usr/local/bin" ]; then
        cp "$INSTALL_DIR/target/release/sentinel-rust" "$BIN_PATH"
    else
        sudo cp "$INSTALL_DIR/target/release/sentinel-rust" "$BIN_PATH"
        sudo chmod +x "$BIN_PATH"
    fi

    print_success "Binario instalado en $BIN_PATH"
}

create_update_script() {
    print_info "Creando script de actualización..."

    UPDATE_SCRIPT="$INSTALL_DIR/update.sh"
    cat > "$UPDATE_SCRIPT" << 'EOF'
#!/bin/bash
# Sentinel Update Script

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}🔄 Actualizando Sentinel...${NC}\n"

cd "$HOME/.sentinel"
git pull origin master
cargo build --release

if [ -w "/usr/local/bin" ]; then
    cp target/release/sentinel-rust /usr/local/bin/sentinel
else
    sudo cp target/release/sentinel-rust /usr/local/bin/sentinel
fi

echo -e "\n${GREEN}✅ Sentinel actualizado correctamente${NC}"
echo -e "${GREEN}Ejecuta: sentinel${NC}"
EOF

    chmod +x "$UPDATE_SCRIPT"
    print_success "Script de actualización creado en $UPDATE_SCRIPT"
}

add_alias() {
    # Add convenient alias for updates
    SHELL_RC=""
    if [ -f "$HOME/.bashrc" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ -f "$HOME/.zshrc" ]; then
        SHELL_RC="$HOME/.zshrc"
    fi

    if [ -n "$SHELL_RC" ]; then
        if ! grep -q "alias sentinel-update" "$SHELL_RC"; then
            echo "" >> "$SHELL_RC"
            echo "# Sentinel aliases" >> "$SHELL_RC"
            echo "alias sentinel-update='$INSTALL_DIR/update.sh'" >> "$SHELL_RC"
            print_success "Alias 'sentinel-update' agregado a $SHELL_RC"
        fi
    fi
}

print_completion() {
    echo ""
    echo -e "${GREEN}================================${NC}"
    echo -e "${GREEN}  ✅ Instalación Completada${NC}"
    echo -e "${GREEN}================================${NC}\n"
    echo -e "${BLUE}📋 Próximos pasos:${NC}\n"
    echo -e "  1️⃣  Ejecuta: ${YELLOW}sentinel${NC}"
    echo -e "  2️⃣  Para actualizar: ${YELLOW}sentinel-update${NC}"
    echo -e "  3️⃣  O manualmente: ${YELLOW}$INSTALL_DIR/update.sh${NC}\n"
    echo -e "${BLUE}📖 Documentación:${NC} https://github.com/sergiogswv/sentinel-rust\n"
}

# Main Installation Flow
main() {
    print_header
    check_dependencies
    clone_or_update
    build_project
    install_binary
    create_update_script
    add_alias
    print_completion
}

main
