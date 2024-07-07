#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)

cd $SCRIPT_DIR/..

docker build -t mijinko17/brocade-rest-server:develop --network host .

docker run -d -p 443:3000 --name brocade-rest-simulator mijinko17/brocade-rest-server:develop
