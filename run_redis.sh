#!/bin/bash

sudo kill -9 $(sudo lsof -t -i:6379)
docker rm -f rust-redis
docker run -d --name rust-redis -p 6379:6379 redis
