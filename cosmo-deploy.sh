#!/bin/sh
set -e

WADM_FILE_LOCAL=wadm-local-to-cosmonic.yaml
WADM_APP_VERSION=$1

cosmo app put $WADM_FILE_LOCAL
cosmo app deploy rating-poc $WADM_APP_VERSION

echo "Rating Poc started successfully and apis avaliable at endpoint http://localhost:8070"