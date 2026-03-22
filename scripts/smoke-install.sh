#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
INSTALLER="${ROOT_DIR}/install.sh"

assert_contains() {
  needle="$1"
  haystack_file="$2"
  if ! grep -F -- "$needle" "$haystack_file" >/dev/null 2>&1; then
    echo "missing expected text: $needle" >&2
    exit 1
  fi
}

assert_file() {
  if [ ! -f "$1" ]; then
    echo "missing expected file: $1" >&2
    exit 1
  fi
}

assert_not_exists() {
  if [ -e "$1" ]; then
    echo "unexpected path exists: $1" >&2
    exit 1
  fi
}

TMP_ROOT=$(mktemp -d)
trap 'rm -rf "${TMP_ROOT}"' EXIT INT TERM

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
  linux) OS="unknown-linux-musl" ;;
  darwin) OS="apple-darwin" ;;
  *) echo "unsupported OS: ${OS}" >&2; exit 1 ;;
esac

case "${ARCH}" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "unsupported architecture: ${ARCH}" >&2; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"
ASSET_DIR="${TMP_ROOT}/assets"
HOME_DIR="${TMP_ROOT}/home"
mkdir -p "${ASSET_DIR}" "${HOME_DIR}"

FAKE_BINARY="${ASSET_DIR}/entropyfa-${TARGET}"
cat > "${FAKE_BINARY}" <<'EOF'
#!/bin/sh
echo entropyfa 0.0.0-smoke
EOF
chmod 755 "${FAKE_BINARY}"
tar czf "${ASSET_DIR}/entropyfa-${TARGET}.tar.gz" -C "${ASSET_DIR}" "entropyfa-${TARGET}"

FULL_DIR="${TMP_ROOT}/full"
mkdir -p "${FULL_DIR}/bin" "${FULL_DIR}/reference/tax/2026"
cp "${FAKE_BINARY}" "${FULL_DIR}/bin/entropyfa"
printf 'bundle\n' > "${FULL_DIR}/reference/tax/2026/example.md"
cat > "${FULL_DIR}/reference/manifest.json" <<'EOF'
{"bundle_version":"vsmoke","pack_count":1}
EOF
tar czf "${ASSET_DIR}/entropyfa-full-${TARGET}.tar.gz" -C "${FULL_DIR}" bin reference

HELP_OUT="${TMP_ROOT}/help.txt"
if sh "${INSTALLER}" --help >"${HELP_OUT}" 2>&1; then
  :
else
  cat "${HELP_OUT}" >&2
  exit 1
fi
assert_contains "--profile PROFILE" "${HELP_OUT}"
assert_contains "--reference-dir PATH" "${HELP_OUT}"
assert_contains "default install behaves like --profile full" "${INSTALLER}"

BINARY_ROOT="${TMP_ROOT}/binary-only"
HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=vsmoke ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR}" \
  sh "${INSTALLER}" --profile binary-only --install-dir "${BINARY_ROOT}/bin" >"${TMP_ROOT}/binary.log" 2>&1
assert_file "${BINARY_ROOT}/bin/entropyfa"
assert_not_exists "${BINARY_ROOT}/reference"

FULL_ROOT="${TMP_ROOT}/full-install"
HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=vsmoke ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR}" \
  sh "${INSTALLER}" --profile full --install-dir "${FULL_ROOT}/bin" --reference-dir "${FULL_ROOT}/reference" >"${TMP_ROOT}/full.log" 2>&1
assert_file "${FULL_ROOT}/bin/entropyfa"
assert_file "${FULL_ROOT}/reference/manifest.json"
assert_file "${FULL_ROOT}/reference/tax/2026/example.md"

PLATFORM_ROOT="${TMP_ROOT}/platform-install"
PLATFORM_HOME_DIR="${TMP_ROOT}/platform-home"
mkdir -p "${PLATFORM_HOME_DIR}"
HOME="${PLATFORM_HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=vsmoke ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR}" \
  sh "${INSTALLER}" --profile platform --install-dir "${PLATFORM_ROOT}/bin" --reference-dir "${PLATFORM_ROOT}/reference" >"${TMP_ROOT}/platform.log" 2>&1
assert_file "${PLATFORM_ROOT}/bin/entropyfa"
assert_file "${PLATFORM_ROOT}/reference/manifest.json"
assert_not_exists "${PLATFORM_HOME_DIR}/.zshrc"

echo "smoke-install.sh: PASS"
