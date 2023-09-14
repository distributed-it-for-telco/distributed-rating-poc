#!/bin/sh
set -e

WADM_FILE=wadm.yaml
WADM_APP_VERSION=$1

wash down

echo "Waiting for host to stop"
sleep 5

wash up --wasmcloud-version v0.78.0-rc6 --nats-websocket-port 4001 --experimental -d

echo "Waiting for everything to start"
sleep 10

wash app get rating-poc $WADM_APP_VERSION 2>&1 > /dev/null || wash app put $WADM_FILE
wash app deploy rating-poc $WADM_APP_VERSION

sleep 10

echo "Rating Poc started successfully and apis avaliable at endpoint http://localhost:8070"