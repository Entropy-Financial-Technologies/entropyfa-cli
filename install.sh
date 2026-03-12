#!/bin/sh
set -e

REPO="Entropy-Financial-Technologies/entropyfa-cli"
BINARY="entropyfa"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
  linux) OS="unknown-linux-gnu" ;;
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
curl -fsSL "${URL}" | tar xz -C /tmp
sudo mv "/tmp/${BINARY}-${TARGET}" /usr/local/bin/${BINARY}
chmod +x /usr/local/bin/${BINARY}
echo "Installed ${BINARY} ${TAG} to /usr/local/bin/${BINARY}"
${BINARY} --version
