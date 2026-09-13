#!/usr/bin/env bash
# aeon installer - Lightning Fast Intelligence Substrate Onboarding
set -e

GLOBAL_AEON_DIR="$HOME/.aeon"
GLOBAL_BIN_DIR="$GLOBAL_AEON_DIR/bin"
mkdir -p "$GLOBAL_BIN_DIR"
mkdir -p "$GLOBAL_AEON_DIR/models"

AEON_REPO="${AEON_REPO:-intellibitz/aeon}"

# Try to detect repo from git if available
if command -v git >/dev/null 2>&1 && [ -d ".git" ]; then
    GIT_REMOTE=$(git remote get-url origin 2>/dev/null || true)
    if [[ "$GIT_REMOTE" == *"github.com"* ]]; then
        # Extract owner/repo from https://github.com/owner/repo.git or git@github.com:owner/repo.git
        DETECTED_REPO=$(echo "$GIT_REMOTE" | sed -E 's/.*github\.com[:\/](.*)\.git/\1/' | sed -E 's/.*github\.com[:\/](.*)/\1/')
        if [[ -n "$DETECTED_REPO" ]]; then
            AEON_REPO="$DETECTED_REPO"
        fi
    fi
fi

echo "Initializing aeon environment (Repo: $AEON_REPO)..."

# 1. Detect Environment
OS_TYPE="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH_TYPE="$(uname -m)"

case "$OS_TYPE" in
    linux*)  PLATFORM="linux" ;;
    darwin*) PLATFORM="macos" ;;
    mingw*|cygwin*|msys*) PLATFORM="windows" ;;
    *)       PLATFORM="unknown" ;;
esac

case "$ARCH_TYPE" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)      ARCH="unknown" ;;
esac

INSTALLED=0

HAS_LOCAL_SOURCE=0
if [ -f "Cargo.toml" ]; then
    HAS_LOCAL_SOURCE=1
elif [[ -n "${BASH_SOURCE[0]}" && -f "$(dirname "${BASH_SOURCE[0]}")/Cargo.toml" ]]; then
    HAS_LOCAL_SOURCE=1
fi

# 2. Try Binary Download First (Lightning Fast) if no local source exists
if [ "$HAS_LOCAL_SOURCE" = "0" ] && [[ "$PLATFORM" != "unknown" && "$ARCH" != "unknown" ]]; then
    # Try downloading both launcher and engine
    LAUNCHER_BINARY="aeon-$PLATFORM-$ARCH"
    ENGINE_BINARY="aeon-engine-$PLATFORM-$ARCH"

    if [[ "$PLATFORM" == "windows" ]]; then
        LAUNCHER_BINARY="${LAUNCHER_BINARY}.exe"
        ENGINE_BINARY="${ENGINE_BINARY}.exe"
    fi

    BASE_URL="https://github.com/$AEON_REPO/releases/latest/download"

    echo "Attempting to download pre-compiled binaries from $AEON_REPO..."

    DEPLOYED=0
    if command -v curl >/dev/null 2>&1; then
        # Download Launcher
        echo "  [1/2] Downloading launcher: $LAUNCHER_BINARY..."
        if curl -sSfL "$BASE_URL/$LAUNCHER_BINARY" -o "$GLOBAL_BIN_DIR/aeon-new"; then
            # Download Engine
            echo "  [2/2] Downloading engine: $ENGINE_BINARY..."
            if curl -sSfL "$BASE_URL/$ENGINE_BINARY" -o "$GLOBAL_BIN_DIR/aeon-engine-new"; then
                DEPLOYED=1
            fi
        fi
    elif command -v wget >/dev/null 2>&1; then
        # Download Launcher
        echo "  [1/2] Downloading launcher: $LAUNCHER_BINARY..."
        if wget -q "$BASE_URL/$LAUNCHER_BINARY" -O "$GLOBAL_BIN_DIR/aeon-new"; then
            # Download Engine
            echo "  [2/2] Downloading engine: $ENGINE_BINARY..."
            if wget -q "$BASE_URL/$ENGINE_BINARY" -O "$GLOBAL_BIN_DIR/aeon-engine-new"; then
                DEPLOYED=1
            fi
        fi
    fi

    if [ "$DEPLOYED" = "1" ]; then
        pkill -f aeon || true

        BIN_EXE=""
        ENGINE_EXE="-engine"
        if [[ "$PLATFORM" == "windows" ]]; then
            BIN_EXE=".exe"
            ENGINE_EXE="-engine.exe"
        fi

        rm -f "$GLOBAL_BIN_DIR/aeon${BIN_EXE}" "$GLOBAL_BIN_DIR/aeon${ENGINE_EXE}" 2>/dev/null || true
        mv "$GLOBAL_BIN_DIR/aeon-new" "$GLOBAL_BIN_DIR/aeon${BIN_EXE}"
        mv "$GLOBAL_BIN_DIR/aeon-engine-new" "$GLOBAL_BIN_DIR/aeon${ENGINE_EXE}"
        chmod +x "$GLOBAL_BIN_DIR/aeon${BIN_EXE}" "$GLOBAL_BIN_DIR/aeon${ENGINE_EXE}"
        INSTALLED=1
        echo "Successfully deployed binaries from GitHub ($AEON_REPO)."
    else
        echo "Binary download unavailable or failed. Falling back to build."
        rm -f "$GLOBAL_BIN_DIR/aeon-new" "$GLOBAL_BIN_DIR/aeon-engine-new" 2>/dev/null || true
    fi
else
    if [ "$HAS_LOCAL_SOURCE" = "1" ]; then
        echo "Local source repository detected. Skipping remote binary download and building from source."
    fi
fi

# 3. Fallback to Local Source or Clone & Build
if [ "$INSTALLED" = "0" ]; then
    echo "Binary download unavailable or failed. Falling back to build from source..."

    # Detect if we are running from a local file or piped
    SCRIPT_DIR_DETECT=""
    if [[ -n "${BASH_SOURCE[0]}" && -f "${BASH_SOURCE[0]}" ]]; then
        SCRIPT_DIR_DETECT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd 2>/dev/null || echo "")"
    fi

    if [[ -n "$SCRIPT_DIR_DETECT" && -f "$SCRIPT_DIR_DETECT/Cargo.toml" ]]; then
        SCRIPT_DIR="$SCRIPT_DIR_DETECT"
        echo "Using local source directory: $SCRIPT_DIR"
    else
        echo "Downloading aeon source archive ($AEON_REPO)..."
        TEMP_DIR=$(mktemp -d)
        SOURCE_URL="https://github.com/$AEON_REPO/archive/refs/heads/main.tar.gz"

        # Move to temp dir to avoid CWD errors if the user is in a deleted directory
        cd "$TEMP_DIR" || { echo "Failed to enter temporary directory."; exit 1; }

        if command -v curl >/dev/null 2>&1 && command -v tar >/dev/null 2>&1; then
            curl -sSfL "$SOURCE_URL" | tar -xzC "$TEMP_DIR" --strip-components=1 || { echo "Source download failed."; exit 1; }
            SCRIPT_DIR="$TEMP_DIR"
        elif command -v wget >/dev/null 2>&1 && command -v tar >/dev/null 2>&1; then
            wget -qO- "$SOURCE_URL" | tar -xzC "$TEMP_DIR" --strip-components=1 || { echo "Source download failed."; exit 1; }
            SCRIPT_DIR="$TEMP_DIR"
        else
            echo "Error: 'tar' and either 'curl' or 'wget' are required for source fallback."
            exit 1
        fi
    fi

    if command -v cargo >/dev/null 2>&1 && [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
        echo "Building release binaries (this may take a moment)..."

        # 100% GPU Hardware Interrogation Build Strategy
        BUILD_FEATURES=""
        if [[ "$PLATFORM" == "macos" ]]; then
            BUILD_FEATURES="--features metal"
        elif command -v nvcc >/dev/null 2>&1 || [ -d "/usr/local/cuda" ]; then
            CUDA_VERSION=$(nvcc --version 2>/dev/null | grep "release" | sed 's/.*release //;s/,.*//' || echo "0")
            if [[ "$CUDA_VERSION" == "11."* ]] || [[ "$CUDA_VERSION" == "12."* ]]; then
                BUILD_FEATURES="--features cuda"
            fi
        fi

        # Build Engine
        echo "  Building engine..."
        (cd "$SCRIPT_DIR" && cargo build --release $BUILD_FEATURES)
        # Build Launcher
        echo "  Building launcher..."
        (cd "$SCRIPT_DIR/src/native/aeon" && cargo build --release)

        ENGINE_SRC="$SCRIPT_DIR/target/release/aeon-engine"
        LAUNCHER_SRC="$SCRIPT_DIR/src/native/aeon/target/release/aeon"

        if [[ "$PLATFORM" == "windows" ]]; then
            ENGINE_SRC="${ENGINE_SRC}.exe"
            LAUNCHER_SRC="${LAUNCHER_SRC}.exe"
        fi

        if [ -f "$ENGINE_SRC" ] && [ -f "$LAUNCHER_SRC" ]; then
            pkill -f aeon || true
            rm -f "$GLOBAL_BIN_DIR/aeon-engine" "$GLOBAL_BIN_DIR/aeon" 2>/dev/null || true
            cp "$ENGINE_SRC" "$GLOBAL_BIN_DIR/aeon-engine"
            cp "$LAUNCHER_SRC" "$GLOBAL_BIN_DIR/aeon"
            chmod +x "$GLOBAL_BIN_DIR/aeon-engine" "$GLOBAL_BIN_DIR/aeon"
            INSTALLED=1
            echo "Deployed engine and launcher binaries to $GLOBAL_BIN_DIR"
        fi
    fi
fi

if [ "$INSTALLED" = "0" ]; then
    echo "Error: Installation failed. Ensure 'cargo' or 'curl' is available and you have internet access."
    exit 1
fi

# 4. Engine Initialization
if [ -x "$GLOBAL_BIN_DIR/aeon" ]; then
    "$GLOBAL_BIN_DIR/aeon" install >/dev/null 2>&1 || true
fi

# 5. Model Substrate Check (Optional but recommended for offline mode)
MODEL_DIR="$GLOBAL_AEON_DIR/models"
ALPHA_MODEL="$MODEL_DIR/aeon-alpha.safetensors"
if [ ! -f "$ALPHA_MODEL" ]; then
    if [ -n "$AEON_WEIGHTS_URL" ]; then
        echo "Downloading aeon-alpha intelligence substrate from custom URL..."
        if command -v curl >/dev/null 2>&1; then
            curl -sSfL "$AEON_WEIGHTS_URL" -o "$ALPHA_MODEL"
        elif command -v wget >/dev/null 2>&1; then
            wget -q "$AEON_WEIGHTS_URL" -O "$ALPHA_MODEL"
        fi
    else
        echo "Intelligence substrate (aeon-alpha.safetensors) missing."
        echo "Note: Native reflex weights can be downloaded later using '/scout_model aeon-alpha'."
    fi
fi

# 6. PATH Management
if [[ ":$PATH:" != *":$GLOBAL_BIN_DIR:"* ]]; then
    CONFIG_FILES=("$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile")
    for config in "${CONFIG_FILES[@]}"; do
        if [ -f "$config" ] && ! grep -q ".aeon/bin" "$config"; then
            echo -e "\n# aeon path initialization\nexport PATH=\"\$HOME/.aeon/bin:\$PATH\"" >> "$config"
        fi
    done

    FISH_CONFIG="$HOME/.config/fish/config.fish"
    if [ -d "$HOME/.config/fish" ] || command -v fish >/dev/null 2>&1; then
        mkdir -p "$HOME/.config/fish"
        if [ -f "$FISH_CONFIG" ] && ! grep -q ".aeon/bin" "$FISH_CONFIG"; then
            echo -e "\n# aeon path initialization\nfish_add_path \$HOME/.aeon/bin" >> "$FISH_CONFIG"
        elif [ ! -f "$FISH_CONFIG" ]; then
            echo -e "fish_add_path \$HOME/.aeon/bin" > "$FISH_CONFIG"
        fi
        if command -v fish >/dev/null 2>&1; then
            fish -c "fish_add_path $GLOBAL_BIN_DIR" >/dev/null 2>&1 || true
        fi
    fi
fi

# 6. Finalize
if [ -t 0 ] && [ -t 1 ] && [ -z "$NONINTERACTIVE" ] && [ -x "$GLOBAL_BIN_DIR/aeon" ]; then
    echo "Installation complete. Starting interactive aeon session..."
    echo ""
    exec "$GLOBAL_BIN_DIR/aeon"
else
    echo "Installation complete. Run 'aeon' to start."
fi
