#!/usr/bin/env bash
# Deploy the Functional Rust site to Hostinger.
# Usage: bash site/deploy-rust-site.sh [--no-videos]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE="$SCRIPT_DIR/.."
SSH_KEY="$WORKSPACE/.credentials/hostinger-ssh-key"
REMOTE="u508071997@185.224.137.204"
REMOTE_DIR="~/domains/hightechmind.io/public_html/rust"
SSH_PORT=65002
SSH_OPTS=(-i "$SSH_KEY" -P "$SSH_PORT" -o StrictHostKeyChecking=no)
VIDEOS=1

for arg in "$@"; do
  [[ "$arg" == "--no-videos" ]] && VIDEOS=0
done

# 1. Generate HTML
echo "▶ Generating site..."
cd "$WORKSPACE"
python3 site/generate-rust-site.py
echo ""

# 2. Deploy HTML + sitemap + robots.txt
echo "▶ Deploying HTML files..."
scp "${SSH_OPTS[@]}" \
  /tmp/rust-site/rust/*.html \
  /tmp/rust-site/rust/sitemap.xml \
  /tmp/rust-site/rust/robots.txt \
  "$REMOTE:$REMOTE_DIR/"
echo ""

# 3. Deploy videos via tar|ssh pipe (single connection, no rsync, no disk staging)
if [[ $VIDEOS -eq 1 ]]; then
  echo "▶ Syncing videos via tar pipe..."
  count=$(find "$WORKSPACE/examples" -name "video.mp4" | wc -l)
  echo "  Sending $count videos..."
  (
    cd "$WORKSPACE/examples"
    find . -name "video.mp4" | sed 's|^\./||' | \
      tar -cf - --files-from=- \
          --transform 's|^\([^/]*\)/video\.mp4|\1-video.mp4|'
  ) | ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=no "$REMOTE" "cd $REMOTE_DIR && tar -xf -"
  echo ""
fi

echo "✅ Deploy complete → https://hightechmind.io/rust/"
