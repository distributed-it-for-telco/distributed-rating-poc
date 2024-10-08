#!/bin/sh
set -e

WADM_FILE="${1:-wadm-metaverse-oncosmonic.yaml}"
WADM_APP_VERSION="${2:-0.0.1}"

echo $WADM_FILE
echo $WADM_APP_VERSION
cosmo app put $WADM_FILE
cosmo app deploy rating-poc $WADM_APP_VERSION