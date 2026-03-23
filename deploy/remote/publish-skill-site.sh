#!/usr/bin/env bash
set -euo pipefail

SITE_NAME="skill.sugeflow.com"
UPLOAD_DIR="$HOME/skill-upload"
NGINX_DRAFT="$HOME/$SITE_NAME.nginx"
WEB_ROOT="/var/www/skills-manager-site"
NGINX_AVAILABLE="/etc/nginx/sites-available/$SITE_NAME"
NGINX_ENABLED="/etc/nginx/sites-enabled/$SITE_NAME"

if [[ ! -d "$UPLOAD_DIR" ]]; then
  echo "missing upload directory: $UPLOAD_DIR" >&2
  exit 1
fi

if [[ ! -f "$NGINX_DRAFT" ]]; then
  echo "missing nginx config draft: $NGINX_DRAFT" >&2
  exit 1
fi

sudo mkdir -p "$WEB_ROOT"
sudo rsync -av --delete "$UPLOAD_DIR"/ "$WEB_ROOT"/
sudo cp "$NGINX_DRAFT" "$NGINX_AVAILABLE"

if [[ ! -L "$NGINX_ENABLED" ]]; then
  sudo ln -s "$NGINX_AVAILABLE" "$NGINX_ENABLED"
fi

sudo nginx -t
sudo systemctl reload nginx
sudo certbot --nginx -d "$SITE_NAME"

sudo nginx -t
sudo systemctl reload nginx

echo "Published https://$SITE_NAME"
