#!/bin/sh
set -e

cosmo build -p ./api-gateway
cosmo build -p ./rating-coordinator
cosmo build -p ./rating-agents/postpaid-orange-vod-bucket-rating-agent
cosmo build -p ./usage-collectors/usage-collector-orange