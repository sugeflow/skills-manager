#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SITE_DIR="$ROOT_DIR/site"
NGINX_CONF="$ROOT_DIR/deploy/nginx/skill.sugeflow.com.conf"
REMOTE="zhichu"
REMOTE_DIR="~/skill-upload"

if [[ ! -d "$SITE_DIR" ]]; then
  echo "site directory not found: $SITE_DIR" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

rsync -a --delete "$SITE_DIR"/ "$TMP_DIR"/
cp "$ROOT_DIR/src-tauri/icons/icon.png" "$TMP_DIR/icon.png"

rsync -av --delete "$TMP_DIR"/ "$REMOTE:$REMOTE_DIR"/
scp "$NGINX_CONF" "$REMOTE:~/skill.sugeflow.com.nginx"

echo "Deployed static site to $REMOTE:$REMOTE_DIR"
echo "Uploaded nginx config draft to $REMOTE:~/skill.sugeflow.com.nginx"
