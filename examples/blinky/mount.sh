# !/bin/bash

docker run --rm -it -v "$PWD"/../..:/app -w /app/examples/blinky hanhossain/rusty-edison:latest