#!/bin/bash
set -e
current=$PWD
cd api-gateway
wash build
cd ..
cd rating-coordinator
wash build
cd ..
for PASSED in ./rating-agents/*; do
   if [[ -d $PASSED ]]; then
    cd $PASSED
    wash build
    cd ..
    fi
done
cd $current
pwd
