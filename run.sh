#!/bin/bash

./build.sh
openocd -f board/ti_msp432_launchpad.cfg -c "program ./target/thumbv7em-none-eabihf/debug/main verify reset exit"