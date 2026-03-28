#!/bin/sh
set -eu

REPO="Muonagi/cider-cli"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"

    case "$platform" in
        Darwin)
            case "$arch" in
                arm64|aarch64) target="aarch64-apple-darwin" ;;
                x86_64)        target="x86_64-apple-darwin" ;;
                *)             error "Unsupported architecture: $arch" ;;
            esac
            ;;
        Linux)
            case "$arch" in
                x86_64) target="x86_64-unknown-linux-gnu" ;;
                *)      error "Unsupported architecture: $arch" ;;
            esac
            ;;
        *)
            error "Unsupported platform: $platform. Use 'cargo install' or download manually."
            ;;
    esac

    version="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"v\(.*\)".*/\1/')"
    if [ -z "$version" ]; then
        error "Failed to determine latest version"
    fi

    url="https://github.com/${REPO}/releases/download/v${version}/cider-${target}.tar.gz"

    echo "Installing cider v${version} (${target})..."

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    curl -fsSL "$url" -o "${tmpdir}/cider.tar.gz"
    tar xzf "${tmpdir}/cider.tar.gz" -C "$tmpdir"

    if [ -w "$INSTALL_DIR" ]; then
        mv "${tmpdir}/cider" "${INSTALL_DIR}/cider"
    else
        echo "Elevated permissions required to install to ${INSTALL_DIR}"
        sudo mv "${tmpdir}/cider" "${INSTALL_DIR}/cider"
    fi

    chmod +x "${INSTALL_DIR}/cider"
    echo "Installed cider to ${INSTALL_DIR}/cider"
}

error() {
    echo "Error: $1" >&2
    exit 1
}

main
