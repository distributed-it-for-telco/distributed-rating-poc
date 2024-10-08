#!/bin/sh
set -e

cosmo build -p ../../api-gateway
cosmo build -p ../../rating-coordinator
cosmo build -p ../../rating-agents/prepaid-orange-vod-oneshot-cqrs-agent
cosmo build -p ../../balance-management/event-sourced-balance/eventcatalog/actor
cosmo build -p ../../balance-management/event-sourced-balance/projector
cosmo build -p ../../balance-management/event-sourced-balance/aggregate