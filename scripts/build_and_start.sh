#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)

cd $SCRIPT_DIR/..

docker build -t mijinko17/brocade-rest-server:develop \
    --build-arg HTTP_PROXY=$HTTP_PROXY \
    --build-arg HTTPS_PROXY=$HTTPS_PROXY \
    --network host .

docker rm -f brocade-rest-simulator

docker run -d -p 443:443 --name brocade-rest-simulator mijinko17/brocade-rest-server:develop
