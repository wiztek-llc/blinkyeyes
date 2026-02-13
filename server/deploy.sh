#!/usr/bin/env bash
# Deploy website to Cloudflare Pages
# Usage: ./server/deploy.sh
#
# Prerequisites:
#   - wrangler installed (npx wrangler works too)
#   - Authenticated via `wrangler login` or CLOUDFLARE_API_TOKEN env var
set -e

echo "Deploying website to Cloudflare Pages..."

npx wrangler pages deploy website/ --project-name=blinky

echo "Deploy complete!"
