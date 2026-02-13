# Block 5: Server Configuration (Nginx, SSL, Security)

## Files Created
- `server/nginx/blinkyeyes.com.conf` — Full Nginx config with HTTPS, www redirect, security headers, gzip, caching, custom 404, and dotfile blocking
- `server/setup.sh` — One-time server setup script (certbot, nginx config install, auto-renewal)
- `server/deploy.sh` — Website deployment script using rsync to px server

## Files Modified
- None

## Deviations from Spec
- None. All items implemented exactly as specified.

## Acceptance Criteria Results
- [x] `server/nginx/blinkyeyes.com.conf` is syntactically valid Nginx config (manual review — nginx not installed on dev machine)
- [x] HTTP → HTTPS redirect is configured (port 80 → 301 to https://blinkyeyes.com)
- [x] www → non-www redirect is configured (both HTTP and HTTPS www blocks redirect)
- [x] SSL certificate paths are correct for Let's Encrypt (/etc/letsencrypt/live/blinkyeyes.com/)
- [x] Security headers include: X-Frame-Options, X-Content-Type-Options, HSTS, CSP, Referrer-Policy (plus X-XSS-Protection and Permissions-Policy)
- [x] Gzip compression is enabled for text-based assets (text, css, js, json, xml, svg)
- [x] Static file caching is configured with appropriate expiry (30d, public, immutable)
- [x] Custom 404 page is wired up (error_page 404 /404.html with internal directive)
- [x] Hidden files (dotfiles) are blocked (location ~ /\. with deny all + return 404)
- [x] `server/setup.sh` is executable and documents prerequisites (DNS A/CNAME records, TTL)
- [x] `server/deploy.sh` is executable and uses rsync (-avz --delete)
- [x] DNS requirements are documented (in setup.sh header comments)

## Known Issues / TODOs for Later Blocks
- `YOUR_EMAIL` placeholder in setup.sh line 29 must be replaced with the actual admin email before running on the server.
- The CSP in the Nginx config allows `https://cdn.tailwindcss.com` for the Tailwind CDN play script used in Block 3. If Tailwind is later compiled to a static CSS file (optimization noted in Block 3 decisions), the CSP should be tightened to remove this external source.
- `nginx -t` could not be run locally to validate config syntax — the config was manually reviewed and follows Nginx documentation conventions.
