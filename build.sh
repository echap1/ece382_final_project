#!/bin/bash

cargo rustc -p main --target thumbv7em-none-eabihf -- -C link-arg=-Tlink.x