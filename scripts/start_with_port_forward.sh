#!/bin/bash

docker rm -f brocade-rest-simulator
docker run -d -p 443:443 --name brocade-rest-simulator mijinko17/brocade-rest-server:develop