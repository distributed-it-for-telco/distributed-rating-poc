PROJECT  = distributed_rating_poc
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0

subdirs = api-gateway rating-agents/* rating-coordinator rating-interface mocks/* usage-collectors/*

include ../build/makefiles/common.mk
