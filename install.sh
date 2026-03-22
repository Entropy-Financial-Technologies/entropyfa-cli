#!/bin/sh
set -e

REPO="Entropy-Financial-Technologies/entropyfa-cli"
BINARY="entropyfa"

# Installer behavior matrix:
# - default install behaves like --profile full
# - --profile binary-only installs only the target binary to --install-dir
# - --profile full installs the target binary plus reference packs
# - --profile platform installs the full bundle without shell profile mutation
# - --reference-dir selects the reference-pack destination for full/platform installs
# - existing --system switches defaults to /usr/local/bin and /opt/entropyfa/reference
DEFAULT_PROFILE="full"
DEFAULT_USER_INSTALL_DIR="${HOME}/.entropyfa/bin"
DEFAULT_USER_REFERENCE_DIR="${HOME}/.entropyfa/reference"
DEFAULT_SYSTEM_INSTALL_DIR="/usr/local/bin"
DEFAULT_SYSTEM_REFERENCE_DIR="/opt/entropyfa/reference"
MANAGED_REFERENCE_MARKER=".entropyfa-managed"

PROFILE="${DEFAULT_PROFILE}"
PROFILE_EXPLICIT=0
DEFAULT_INSTALL_DIR="${DEFAULT_USER_INSTALL_DIR}"
DEFAULT_REFERENCE_DIR="${DEFAULT_USER_REFERENCE_DIR}"
INSTALL_DIR="${ENTROPYFA_INSTALL:-${DEFAULT_INSTALL_DIR}}"
REFERENCE_DIR="${ENTROPYFA_REFERENCE_ROOT:-${DEFAULT_REFERENCE_DIR}}"
INSTALL_DIR_EXPLICIT=0
REFERENCE_DIR_EXPLICIT=0
SYSTEM_INSTALL=0

usage() {
  cat <<EOF
Usage: sh install.sh [--profile PROFILE] [--install-dir PATH] [--reference-dir PATH] [--system]

Profiles:
  binary-only  Install only the entropyfa executable.
  full         Install the executable and reference packs. Default.
  platform     Install for shared/platform environments without shell profile changes.

Options:
  --install-dir PATH    Install the executable to PATH.
  --reference-dir PATH  Install reference packs to PATH.
  --system              Use platform-style system defaults.
  --help                Show this help text.
EOF
}

require_arg() {
  if [ -z "$2" ] || [ "${2#-}" != "$2" ]; then
    echo "$1 requires a value" >&2
    exit 1
  fi
}

set_install_defaults() {
  if [ "${INSTALL_DIR_EXPLICIT}" -eq 0 ]; then
    INSTALL_DIR="${ENTROPYFA_INSTALL:-${DEFAULT_INSTALL_DIR}}"
  fi
  if [ "${REFERENCE_DIR_EXPLICIT}" -eq 0 ]; then
    REFERENCE_DIR="${ENTROPYFA_REFERENCE_ROOT:-${DEFAULT_REFERENCE_DIR}}"
  fi
}

while [ $# -gt 0 ]; do
  case "$1" in
    --help)
      usage
      exit 0
      ;;
    --system)
      SYSTEM_INSTALL=1
      DEFAULT_INSTALL_DIR="${DEFAULT_SYSTEM_INSTALL_DIR}"
      DEFAULT_REFERENCE_DIR="${DEFAULT_SYSTEM_REFERENCE_DIR}"
      if [ "${PROFILE_EXPLICIT}" -eq 0 ]; then
        PROFILE="platform"
      fi
      if [ "${INSTALL_DIR_EXPLICIT}" -eq 0 ]; then
        INSTALL_DIR="${ENTROPYFA_INSTALL:-${DEFAULT_INSTALL_DIR}}"
      fi
      if [ "${REFERENCE_DIR_EXPLICIT}" -eq 0 ]; then
        REFERENCE_DIR="${ENTROPYFA_REFERENCE_ROOT:-${DEFAULT_REFERENCE_DIR}}"
      fi
      ;;
    --profile)
      shift
      require_arg "--profile" "$1"
      PROFILE="$1"
      PROFILE_EXPLICIT=1
      case "${PROFILE}" in
        binary-only|full|platform) ;;
        *)
          echo "Unknown profile: ${PROFILE}" >&2
          usage >&2
          exit 1
          ;;
      esac
      if [ "${PROFILE}" = "platform" ] && [ "${INSTALL_DIR_EXPLICIT}" -eq 0 ] && [ "${SYSTEM_INSTALL}" -eq 0 ]; then
        DEFAULT_INSTALL_DIR="${DEFAULT_SYSTEM_INSTALL_DIR}"
      fi
      if [ "${PROFILE}" = "platform" ] && [ "${REFERENCE_DIR_EXPLICIT}" -eq 0 ] && [ "${SYSTEM_INSTALL}" -eq 0 ]; then
        DEFAULT_REFERENCE_DIR="${DEFAULT_SYSTEM_REFERENCE_DIR}"
      fi
      set_install_defaults
      ;;
    --install-dir)
      shift
      require_arg "--install-dir" "$1"
      INSTALL_DIR="$1"
      INSTALL_DIR_EXPLICIT=1
      ;;
    --reference-dir)
      shift
      require_arg "--reference-dir" "$1"
      REFERENCE_DIR="$1"
      REFERENCE_DIR_EXPLICIT=1
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
  shift
done

if [ "${PROFILE}" = "platform" ] && [ "${SYSTEM_INSTALL}" -eq 0 ] && [ "${INSTALL_DIR_EXPLICIT}" -eq 0 ] && [ "${REFERENCE_DIR_EXPLICIT}" -eq 0 ]; then
  echo "Warning: --profile platform works best with explicit --install-dir and --reference-dir; using ${INSTALL_DIR} and ${REFERENCE_DIR}" >&2
fi

if [ "${PROFILE}" = "binary-only" ] && [ "${REFERENCE_DIR_EXPLICIT}" -eq 1 ]; then
  echo "Warning: --reference-dir is ignored for --profile binary-only" >&2
fi

# Detect platform.
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
  linux) OS="unknown-linux-musl" ;;
  darwin) OS="apple-darwin" ;;
  *) echo "Unsupported OS: ${OS}" >&2; exit 1 ;;
esac

case "${ARCH}" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: ${ARCH}" >&2; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"

API_URL="${ENTROPYFA_INSTALL_API_URL:-https://api.github.com/repos/${REPO}/releases/latest}"
TAG="${ENTROPYFA_INSTALL_TAG:-}"
if [ -z "${TAG}" ]; then
  TAG=$(curl -fsSL "${API_URL}" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
fi
if [ -z "${TAG}" ]; then
  echo "Failed to get latest release" >&2
  exit 1
fi

BASE_URL="${ENTROPYFA_INSTALL_BASE_URL:-https://github.com/${REPO}/releases/download/${TAG}}"
BASE_URL=${BASE_URL%/}
BINARY_URL="${BASE_URL}/${BINARY}-${TARGET}.tar.gz"
FULL_URL="${BASE_URL}/${BINARY}-full-${TARGET}.tar.gz"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "${TMP_DIR}"' EXIT INT TERM

run_with_optional_sudo() {
  if [ -w "$1" ]; then
    shift
    "$@"
  else
    shift
    sudo "$@"
  fi
}

nearest_existing_dir() {
  candidate="$1"
  while [ ! -d "${candidate}" ]; do
    next_candidate=$(dirname "${candidate}")
    if [ "${next_candidate}" = "${candidate}" ]; then
      break
    fi
    candidate="${next_candidate}"
  done
  printf '%s\n' "${candidate}"
}

ensure_parent_dir() {
  ensure_parent_dir_path=$(dirname "$1")
  ensure_parent_dir_ancestor=$(nearest_existing_dir "${ensure_parent_dir_path}")
  run_with_optional_sudo "${ensure_parent_dir_ancestor}" mkdir -p "${ensure_parent_dir_path}"
}

ensure_dir() {
  ensure_parent_dir "$1/.keep"
  ensure_dir_ancestor=$(nearest_existing_dir "$1")
  run_with_optional_sudo "${ensure_dir_ancestor}" mkdir -p "$1"
}

reference_root_is_managed() {
  reference_root="$1"

  if [ ! -e "${reference_root}" ]; then
    return 0
  fi

  if [ ! -d "${reference_root}" ]; then
    echo "Refusing to replace unmanaged reference root: ${reference_root}" >&2
    exit 1
  fi

  if [ -f "${reference_root}/${MANAGED_REFERENCE_MARKER}" ]; then
    return 0
  fi

  if managed_entry=$(find "${reference_root}" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null); then
    if [ -z "${managed_entry}" ]; then
      return 0
    fi
  else
    echo "Refusing to replace unmanaged reference root: ${reference_root}" >&2
    exit 1
  fi

  echo "Refusing to replace unmanaged reference root: ${reference_root}" >&2
  exit 1
}

install_file() {
  destination_dir="$1"
  source_file="$2"
  destination_file="$3"
  ensure_dir "${destination_dir}"
  run_with_optional_sudo "${destination_dir}" install -m 755 "${source_file}" "${destination_file}"
}

replace_tree() {
  source_dir="$1"
  destination_dir="$2"
  replace_tree_parent_dir=$(dirname "${destination_dir}")
  reference_root_is_managed "${destination_dir}"
  ensure_parent_dir "${destination_dir}/.keep"
  replace_tree_ancestor_dir=$(nearest_existing_dir "${replace_tree_parent_dir}")
  run_with_optional_sudo "${replace_tree_ancestor_dir}" rm -rf "${destination_dir}"
  run_with_optional_sudo "${replace_tree_ancestor_dir}" mkdir -p "${destination_dir}"
  run_with_optional_sudo "${destination_dir}" cp -R "${source_dir}/." "${destination_dir}/"
}

install_binary_only() {
  echo "Downloading ${BINARY} ${TAG} for ${TARGET}..."
  curl -fsSL "${BINARY_URL}" | tar xz -C "${TMP_DIR}"
  install_file "${INSTALL_DIR}" "${TMP_DIR}/${BINARY}-${TARGET}" "${INSTALL_DIR}/${BINARY}"
}

install_full_bundle() {
  echo "Downloading ${BINARY} ${TAG} full bundle for ${TARGET}..."
  mkdir -p "${TMP_DIR}/full"
  curl -fsSL "${FULL_URL}" | tar xz -C "${TMP_DIR}/full"
  reference_root_is_managed "${REFERENCE_DIR}"
  install_file "${INSTALL_DIR}" "${TMP_DIR}/full/bin/${BINARY}" "${INSTALL_DIR}/${BINARY}"
  replace_tree "${TMP_DIR}/full/reference" "${REFERENCE_DIR}"
}

case "${PROFILE}" in
  binary-only)
    install_binary_only
    ;;
  full|platform)
    install_full_bundle
    ;;
esac

echo "Installed ${BINARY} ${TAG} to ${INSTALL_DIR}/${BINARY}"
if [ "${PROFILE}" != "binary-only" ]; then
  echo "Installed reference packs to ${REFERENCE_DIR}"
fi

# Add the default user-local install dir to PATH when needed.
path_contains_install_dir=0
case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) path_contains_install_dir=1 ;;
esac

PROFILE_PATH_ENTRY="${INSTALL_DIR}"
if [ "${INSTALL_DIR}" = "${DEFAULT_USER_INSTALL_DIR}" ]; then
  PROFILE_PATH_ENTRY='$HOME/.entropyfa/bin'
fi
PATH_EXPORT="export PATH=\"${PROFILE_PATH_ENTRY}:\$PATH\""

if [ "${PROFILE}" != "platform" ] && [ "${SYSTEM_INSTALL}" -eq 0 ] && [ "${path_contains_install_dir}" -eq 0 ]; then
  PROFILE_FILE="${HOME}/.profile"
  case "$(basename "${SHELL:-}")" in
    zsh) PROFILE_FILE="${HOME}/.zshrc" ;;
    bash) PROFILE_FILE="${HOME}/.bashrc" ;;
  esac

  if [ -f "${PROFILE_FILE}" ] && grep -F "${PATH_EXPORT}" "${PROFILE_FILE}" >/dev/null 2>&1; then
    :
  else
    {
      echo ""
      echo "# Added by entropyfa installer"
      echo "${PATH_EXPORT}"
    } >> "${PROFILE_FILE}"
  fi

  echo "Added ${INSTALL_DIR} to PATH in ${PROFILE_FILE}"
  echo "Run 'source ${PROFILE_FILE}' or open a new shell to use ${BINARY}"
fi

# Warn about a stale cargo-installed binary that could shadow the new install.
CARGO_BIN="${HOME}/.cargo/bin/${BINARY}"
if [ -f "${CARGO_BIN}" ] && [ "${CARGO_BIN}" != "${INSTALL_DIR}/${BINARY}" ]; then
  echo "Warning: ${CARGO_BIN} may shadow ${INSTALL_DIR}/${BINARY} on PATH"
fi

"${INSTALL_DIR}/${BINARY}" --version
