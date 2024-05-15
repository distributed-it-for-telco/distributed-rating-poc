#!/bin/bash
set -e
PROJECT_ROOT=$PWD

# Building Api gateway
cd api-gateway
wash build
cd ..

# Building rating coordinator
cd rating-coordinator
wash build
cd ..

# Building rating agents
cd rating-agents
for item in ./*; do
    if [[ -d "$item" ]]; then
        cd $item
        wash build
        cd ..
    fi
done
cd $PROJECT_ROOT
