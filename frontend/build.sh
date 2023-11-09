#!/bin/bash

# just to build all the files we'll need

tailwind -o ./css/tailwind.css

# start the wasm build
trunk build
