#!/usr/bin/env bash
set -euo pipefail

# 1. Rust
if ! command -v rustup &>/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
else
  echo "rustup already installed: $(rustc --version)"
fi
rustup default stable
rustup update

# 2. Node 22 LTS via Volta
if ! command -v volta &>/dev/null; then
  curl https://get.volta.sh | bash
  export VOLTA_HOME="$HOME/.volta"
  export PATH="$VOLTA_HOME/bin:$PATH"
else
  echo "volta already installed: $(volta --version)"
fi
volta install node@22

# 3. pnpm via Corepack
corepack enable
corepack prepare pnpm@latest --activate

echo ""
echo "Dev environment ready:"
echo "  rustc  $(rustc --version)"
echo "  node   $(node --version)"
echo "  pnpm   $(pnpm --version)"
