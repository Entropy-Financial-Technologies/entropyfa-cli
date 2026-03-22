#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
INSTALLER="${ROOT_DIR}/install.sh"

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

assert_file_contains() {
  if ! grep -F -- "$2" "$1" >/dev/null 2>&1; then
    echo "missing expected content in file: $1" >&2
    exit 1
  fi
}

make_system_shims() {
  shim_dir="$1"
  mkdir -p "${shim_dir}"

  cat > "${shim_dir}/path-shim.py" <<'EOF'
#!/usr/bin/env python3
import os
import sys

tool = sys.argv[1]
root = os.environ["SYSTEM_SHIM_ROOT"]

def map_arg(arg: str) -> str:
    pairs = (
        ("/usr/local/bin", f"{root}/usr/local/bin"),
        ("/opt/entropyfa/reference", f"{root}/opt/entropyfa/reference"),
    )
    for source, destination in pairs:
        if arg == source or arg.startswith(source + "/"):
            return destination + arg[len(source):]
    return arg

args = [map_arg(arg) for arg in sys.argv[2:]]
if tool == "sudo":
    if not args:
        sys.exit(0)
    os.execvp(args[0], args)

real = os.environ[f"REAL_{tool.upper()}"]
os.execv(real, [real, *args])
EOF
  chmod 755 "${shim_dir}/path-shim.py"

  for tool in install mkdir cp rm sudo; do
    cat > "${shim_dir}/${tool}" <<EOF
#!/bin/sh
exec python3 "${shim_dir}/path-shim.py" ${tool} "\$@"
EOF
    chmod 755 "${shim_dir}/${tool}"
  done
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
ASSET_DIR_V1="${TMP_ROOT}/assets-v1"
ASSET_DIR_V2="${TMP_ROOT}/assets-v2"
HOME_DIR="${TMP_ROOT}/home"
mkdir -p "${ASSET_DIR_V1}" "${ASSET_DIR_V2}" "${HOME_DIR}"

FAKE_BINARY="${TMP_ROOT}/entropyfa-${TARGET}"
cat > "${FAKE_BINARY}" <<'EOF'
#!/bin/sh
echo entropyfa 0.0.0-smoke
EOF
chmod 755 "${FAKE_BINARY}"

make_assets() {
  asset_dir="$1"
  bundle_version="$2"
  include_legacy="$3"
  full_dir="${TMP_ROOT}/${bundle_version}-full"

  mkdir -p "${asset_dir}" "${full_dir}/bin" "${full_dir}/reference/tax/2026"
  cp "${FAKE_BINARY}" "${asset_dir}/entropyfa-${TARGET}"
  tar czf "${asset_dir}/entropyfa-${TARGET}.tar.gz" -C "${asset_dir}" "entropyfa-${TARGET}"
  cp "${FAKE_BINARY}" "${full_dir}/bin/entropyfa"
  printf '%s\n' "entropyfa-managed" > "${full_dir}/reference/.entropyfa-managed"
  printf '%s\n' "bundle ${bundle_version}" > "${full_dir}/reference/tax/2026/example.md"
  cat > "${full_dir}/reference/manifest.json" <<EOF
{"bundle_version":"${bundle_version}","pack_count":1}
EOF
  if [ "${include_legacy}" -eq 1 ]; then
    mkdir -p "${full_dir}/reference/obsolete"
    printf '%s\n' "legacy ${bundle_version}" > "${full_dir}/reference/obsolete/old.txt"
  fi
  tar czf "${asset_dir}/entropyfa-full-${TARGET}.tar.gz" -C "${full_dir}" bin reference
}

make_assets "${ASSET_DIR_V1}" "v1" 1
make_assets "${ASSET_DIR_V2}" "v2" 0

HELP_OUT="${TMP_ROOT}/help.txt"
if sh "${INSTALLER}" --help >"${HELP_OUT}" 2>&1; then
  :
else
  cat "${HELP_OUT}" >&2
  exit 1
fi
grep -F -- "--profile PROFILE" "${HELP_OUT}" >/dev/null 2>&1 || {
  echo "missing expected help text: --profile PROFILE" >&2
  exit 1
}
grep -F -- "--reference-dir PATH" "${HELP_OUT}" >/dev/null 2>&1 || {
  echo "missing expected help text: --reference-dir PATH" >&2
  exit 1
}

if sh "${INSTALLER}" --install-dir --system >"${TMP_ROOT}/bad-arg.txt" 2>&1; then
  echo "expected --install-dir --system to fail" >&2
  exit 1
fi
grep -F -- "--install-dir requires a value" "${TMP_ROOT}/bad-arg.txt" >/dev/null 2>&1 || {
  echo "missing expected missing-value error for --install-dir" >&2
  exit 1
}

UNMANAGED_REFERENCE_ROOT="${TMP_ROOT}/unmanaged-reference"
mkdir -p "${UNMANAGED_REFERENCE_ROOT}"
printf '%s\n' "sentinel" > "${UNMANAGED_REFERENCE_ROOT}/sentinel.txt"
printf '%s\n' "{\"bundle_version\":\"fake\"}" > "${UNMANAGED_REFERENCE_ROOT}/manifest.json"
UNMANAGED_BINARY_TARGET="${TMP_ROOT}/safety-bin/entropyfa"
mkdir -p "$(dirname "${UNMANAGED_BINARY_TARGET}")"
printf '%s\n' "sentinel-binary" > "${UNMANAGED_BINARY_TARGET}"
if HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --profile full --install-dir "${TMP_ROOT}/safety-bin" --reference-dir "${UNMANAGED_REFERENCE_ROOT}" >"${TMP_ROOT}/unmanaged.log" 2>&1; then
  echo "expected unmanaged reference root install to fail" >&2
  exit 1
fi
grep -F -- "Refusing to replace unmanaged reference root" "${TMP_ROOT}/unmanaged.log" >/dev/null 2>&1 || {
  echo "missing expected unmanaged-root error" >&2
  exit 1
}
assert_file "${UNMANAGED_BINARY_TARGET}"
assert_file_contains "${UNMANAGED_BINARY_TARGET}" "sentinel-binary"
assert_file "${UNMANAGED_REFERENCE_ROOT}/sentinel.txt"
assert_file "${UNMANAGED_REFERENCE_ROOT}/manifest.json"

UNREADABLE_REFERENCE_ROOT="${TMP_ROOT}/unreadable-reference"
mkdir -p "${UNREADABLE_REFERENCE_ROOT}"
printf '%s\n' "locked" > "${UNREADABLE_REFERENCE_ROOT}/sentinel.txt"
UNREADABLE_BINARY_TARGET="${TMP_ROOT}/unreadable-bin/entropyfa"
mkdir -p "$(dirname "${UNREADABLE_BINARY_TARGET}")"
printf '%s\n' "sentinel-binary" > "${UNREADABLE_BINARY_TARGET}"
chmod 000 "${UNREADABLE_REFERENCE_ROOT}"
if HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --profile full --install-dir "${TMP_ROOT}/unreadable-bin" --reference-dir "${UNREADABLE_REFERENCE_ROOT}" >"${TMP_ROOT}/unreadable.log" 2>&1; then
  chmod 755 "${UNREADABLE_REFERENCE_ROOT}"
  echo "expected unreadable reference root install to fail" >&2
  exit 1
fi
chmod 755 "${UNREADABLE_REFERENCE_ROOT}"
grep -F -- "Refusing to replace unmanaged reference root" "${TMP_ROOT}/unreadable.log" >/dev/null 2>&1 || {
  echo "missing expected unreadable-root error" >&2
  exit 1
}
assert_file "${UNREADABLE_BINARY_TARGET}"
assert_file_contains "${UNREADABLE_BINARY_TARGET}" "sentinel-binary"
assert_file "${UNREADABLE_REFERENCE_ROOT}/sentinel.txt"
assert_not_exists "${UNREADABLE_REFERENCE_ROOT}/manifest.json"

DEFAULT_HOME_DIR="${TMP_ROOT}/default-home"
mkdir -p "${DEFAULT_HOME_DIR}"
HOME="${DEFAULT_HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" >"${TMP_ROOT}/default.log" 2>&1
assert_file "${DEFAULT_HOME_DIR}/.entropyfa/bin/entropyfa"
assert_file "${DEFAULT_HOME_DIR}/.entropyfa/reference/manifest.json"
assert_file "${DEFAULT_HOME_DIR}/.entropyfa/reference/tax/2026/example.md"
assert_file "${DEFAULT_HOME_DIR}/.zshrc"

BINARY_ROOT="${TMP_ROOT}/binary-only"
HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --profile binary-only --install-dir "${BINARY_ROOT}/bin" >"${TMP_ROOT}/binary.log" 2>&1
assert_file "${BINARY_ROOT}/bin/entropyfa"
assert_not_exists "${BINARY_ROOT}/reference"

FULL_ROOT="${TMP_ROOT}/full-install"
HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --profile full --install-dir "${FULL_ROOT}/bin" --reference-dir "${FULL_ROOT}/reference" >"${TMP_ROOT}/full.log" 2>&1
assert_file "${FULL_ROOT}/bin/entropyfa"
assert_file "${FULL_ROOT}/reference/manifest.json"
assert_file "${FULL_ROOT}/reference/tax/2026/example.md"
assert_file_contains "${FULL_ROOT}/reference/tax/2026/example.md" "bundle v1"

REINSTALL_ROOT="${TMP_ROOT}/reinstall-install"
HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --profile full --install-dir "${REINSTALL_ROOT}/bin" --reference-dir "${REINSTALL_ROOT}/reference" >"${TMP_ROOT}/reinstall-v1.log" 2>&1
assert_file "${REINSTALL_ROOT}/reference/.entropyfa-managed"
assert_file "${REINSTALL_ROOT}/reference/obsolete/old.txt"
assert_file_contains "${REINSTALL_ROOT}/reference/tax/2026/example.md" "bundle v1"

HOME="${HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v2 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V2}" \
  sh "${INSTALLER}" --profile full --install-dir "${REINSTALL_ROOT}/bin" --reference-dir "${REINSTALL_ROOT}/reference" >"${TMP_ROOT}/reinstall-v2.log" 2>&1
assert_not_exists "${REINSTALL_ROOT}/reference/obsolete/old.txt"
assert_file_contains "${REINSTALL_ROOT}/reference/tax/2026/example.md" "bundle v2"

PLATFORM_ROOT="${TMP_ROOT}/platform-install"
PLATFORM_HOME_DIR="${TMP_ROOT}/platform-home"
mkdir -p "${PLATFORM_HOME_DIR}"
HOME="${PLATFORM_HOME_DIR}" SHELL=/bin/zsh ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --profile platform --install-dir "${PLATFORM_ROOT}/bin" --reference-dir "${PLATFORM_ROOT}/reference" >"${TMP_ROOT}/platform.log" 2>&1
assert_file "${PLATFORM_ROOT}/bin/entropyfa"
assert_file "${PLATFORM_ROOT}/reference/manifest.json"
assert_not_exists "${PLATFORM_HOME_DIR}/.zshrc"

SYSTEM_ROOT="${TMP_ROOT}/system-root"
SYSTEM_HOME_DIR="${TMP_ROOT}/system-home"
SHIM_DIR="${TMP_ROOT}/system-shims"
mkdir -p "${SYSTEM_ROOT}" "${SYSTEM_HOME_DIR}"
make_system_shims "${SHIM_DIR}"
REAL_INSTALL=$(command -v install)
REAL_MKDIR=$(command -v mkdir)
REAL_CP=$(command -v cp)
REAL_RM=$(command -v rm)
PATH="${SHIM_DIR}:${PATH}" HOME="${SYSTEM_HOME_DIR}" SHELL=/bin/zsh SYSTEM_SHIM_ROOT="${SYSTEM_ROOT}" \
  REAL_INSTALL="${REAL_INSTALL}" REAL_MKDIR="${REAL_MKDIR}" REAL_CP="${REAL_CP}" REAL_RM="${REAL_RM}" \
  ENTROPYFA_INSTALL_TAG=v1 ENTROPYFA_INSTALL_BASE_URL="file://${ASSET_DIR_V1}" \
  sh "${INSTALLER}" --system >"${TMP_ROOT}/system.log" 2>&1
assert_file "${SYSTEM_ROOT}/usr/local/bin/entropyfa"
assert_file "${SYSTEM_ROOT}/opt/entropyfa/reference/manifest.json"
assert_file "${SYSTEM_ROOT}/opt/entropyfa/reference/tax/2026/example.md"
assert_not_exists "${SYSTEM_HOME_DIR}/.zshrc"

echo "smoke-install.sh: PASS"
