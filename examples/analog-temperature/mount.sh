# !/bin/bash
docker run --rm -it -v "$PWD"/../..:/app -w /app/examples/analog-temperature hanhossain/rusty-edison:latest