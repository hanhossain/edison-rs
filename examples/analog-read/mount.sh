# !/bin/bash

docker run --rm -it -v "$PWD"/../..:/app -w /app/examples/analog-read hanhossain/rusty-edison:latest