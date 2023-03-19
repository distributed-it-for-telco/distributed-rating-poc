# Makefile for orange-rating-agent

PROJECT  = distributed_rating_poc
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
