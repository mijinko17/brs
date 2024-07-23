#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)
cd $SCRIPT_DIR

./build.sh
./start_with_port_forward.sh
