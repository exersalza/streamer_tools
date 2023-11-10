#!/bin/bash

cargo watch -q -w . -s "tailwind -o ./css/tailwind.css && trunk build"

###  When you want the old method of this script

# just to build all the files we'll need
# tailwind -o ./css/tailwind.css

# start the wasm build
# trunk build
