#!/bin/sh
set -e

WADM_FILE_LOCAL=wadm-balance.yaml
WADM_APP_VERSION=${1:-v0.1.0}

cosmo app put $WADM_FILE_LOCAL
cosmo app deploy balance-management $WADM_APP_VERSION