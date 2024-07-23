#!/bin/bash

docker rm -f brocade-rest-simulator
docker run -d --name brocade-rest-simulator mijinko17/brocade-rest-server:develop
