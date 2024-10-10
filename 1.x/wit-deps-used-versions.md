# Objective
to list all used wit dependencies used in deps.toml by wit-deps utility, as fixed version by commit id
# Motivation
- at the time of creating this document, many dependencies are not stable, and even some are having github repos that does not have neither tags or specific branches for versions
- subsequent releases somtimes are not compatible, thus fixed version need to be set to ensure stability from our side
- we will attempt use URLs by (whenever found) these according to priority
    - github release
    - git tag
    - git branch (must be named to a version, otherwise should be discarded)
    - git commit (most fixed, most reliable, but not readable)
# Currently used versions
- wasi-http:     https://github.com/WebAssembly/wasi-http/archive/v0.2.0.tar.gz
- wasi-keyvalue: https://github.com/WebAssembly/wasi-keyvalue/archive/53ff9402194704cb68b1599c4e35e9182f4c57c4.tar.gz
- wasi-logging:  https://github.com/WebAssembly/wasi-logging/archive/d31c41d0d9eed81aabe02333d0025d42acf3fb75.tar.gz
- wasmcloud bus: https://github.com/wasmCloud/wasmCloud/releases/download/wit-wasmcloud-bus-v1.0.0/wit-wasmcloud-bus-1.0.0.tar.gz
