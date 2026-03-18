#!/bin/sh
set -e

REPO="Entropy-Financial-Technologies/entropyfa-cli"
BINARY="entropyfa"
DEFAULT_INSTALL_DIR="${HOME}/.entropyfa/bin"
INSTALL_DIR="${ENTROPYFA_INSTALL:-${DEFAULT_INSTALL_DIR}}"
SYSTEM_INSTALL=0

while [ $# -gt 0 ]; do
  case "$1" in
    --system)
      INSTALL_DIR="/usr/local/bin"
      SYSTEM_INSTALL=1
      ;;
    --install-dir)
      shift
      if [ -z "$1" ]; then
        echo "--install-dir requires a path"
        exit 1
      fi
      INSTALL_DIR="$1"
      ;;
    *)
      echo "Unknown argument: $1"
      echo "Usage: sh install.sh [--system] [--install-dir PATH]"
      exit 1
      ;;
  esac
  shift
done

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
  linux) OS="unknown-linux-musl" ;;
  darwin) OS="apple-darwin" ;;
  *) echo "Unsupported OS: ${OS}"; exit 1 ;;
esac

case "${ARCH}" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"

# Get latest release tag
TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$TAG" ]; then
  echo "Failed to get latest release"
  exit 1
fi

URL="https://github.com/${REPO}/releases/download/${TAG}/${BINARY}-${TARGET}.tar.gz"
echo "Downloading ${BINARY} ${TAG} for ${TARGET}..."

TMP_DIR=$(mktemp -d)
trap 'rm -rf "${TMP_DIR}"' EXIT INT TERM
TMP_BINARY="${TMP_DIR}/${BINARY}-${TARGET}"

curl -fsSL "${URL}" | tar xz -C "${TMP_DIR}"

if [ "${SYSTEM_INSTALL}" -eq 1 ]; then
  if [ -w "${INSTALL_DIR}" ]; then
    install -m 755 "${TMP_BINARY}" "${INSTALL_DIR}/${BINARY}"
  else
    sudo mkdir -p "${INSTALL_DIR}"
    sudo install -m 755 "${TMP_BINARY}" "${INSTALL_DIR}/${BINARY}"
  fi
else
  mkdir -p "${INSTALL_DIR}"
  install -m 755 "${TMP_BINARY}" "${INSTALL_DIR}/${BINARY}"
fi

echo "Installed ${BINARY} ${TAG} to ${INSTALL_DIR}/${BINARY}"

# Add the default user-local install dir to PATH when needed.
path_contains_install_dir=0
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) path_contains_install_dir=1 ;;
esac

PROFILE_PATH_ENTRY="${INSTALL_DIR}"
if [ "${INSTALL_DIR}" = "${DEFAULT_INSTALL_DIR}" ]; then
  PROFILE_PATH_ENTRY='$HOME/.entropyfa/bin'
fi
PATH_EXPORT="export PATH=\"${PROFILE_PATH_ENTRY}:\$PATH\""

if [ "${SYSTEM_INSTALL}" -eq 0 ] && [ "${path_contains_install_dir}" -eq 0 ]; then
  PROFILE="${HOME}/.profile"
  case "$(basename "${SHELL:-}")" in
    zsh) PROFILE="${HOME}/.zshrc" ;;
    bash) PROFILE="${HOME}/.bashrc" ;;
  esac

  if [ -f "${PROFILE}" ] && grep -F "${PATH_EXPORT}" "${PROFILE}" >/dev/null 2>&1; then
    :
  else
    {
      echo ""
      echo "# Added by entropyfa installer"
      echo "${PATH_EXPORT}"
    } >> "${PROFILE}"
  fi

  echo "Added ${INSTALL_DIR} to PATH in ${PROFILE}"
  echo "Run 'source ${PROFILE}' or open a new shell to use ${BINARY}"
fi

# Warn about a stale cargo-installed binary that could shadow the new install.
CARGO_BIN="${HOME}/.cargo/bin/${BINARY}"
if [ -f "${CARGO_BIN}" ] && [ "${CARGO_BIN}" != "${INSTALL_DIR}/${BINARY}" ]; then
  echo "Warning: ${CARGO_BIN} may shadow ${INSTALL_DIR}/${BINARY} on PATH"
fi

"${INSTALL_DIR}/${BINARY}" --version
