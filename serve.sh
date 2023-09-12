#! /usr/bin/env bash

docker build -t server .

docker run -p 3000:3000 --rm --name server_docker server
