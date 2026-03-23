## Skills Manager Marketing Site

### Goal

Ship a static single-page marketing site for `skill.sugeflow.com` that prioritizes downloads and visually echoes the warm, refined, editorial feel of Tenor without copying its layout.

### Scope

- Dedicated static site under `site/`
- Hero with primary download CTAs
- Product value explanation
- Feature section
- Supported agent section
- Download matrix
- FAQ with macOS Gatekeeper note
- VPS deployment to `/var/www/skills-manager-site`
- Nginx site config for `skill.sugeflow.com`

### Delivery Plan

1. Build a standalone static site with no coupling to the Tauri app frontend.
2. Reuse the product logo and GitHub release assets as the canonical download links.
3. Verify the site locally with a static file server.
4. Upload the built static files to the VPS.
5. Add an nginx server block and request a certificate with Certbot if available.
