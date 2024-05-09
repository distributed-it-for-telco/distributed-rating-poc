#!/bin/sh
set -e

WADM_FILE=$1:"wadm-from-file.yaml"
WADM_APP_VERSION=$2:0.0.1

wash down

echo "Waiting for host to stop"
sleep 5

wash up --nats-websocket-port 4001 --experimental -d

# wash up --wasmcloud-version v0.78.0-rc6 --nats-websocket-port 4001 --experimental -d

echo "Waiting for everything to start"
sleep 10

wash app put $WADM_FILE
wash app deploy rating-poc $WADM_APP_VERSION

sleep 10

echo "Rating Poc started successfully and apis avaliable at endpoint http://localhost:8070"