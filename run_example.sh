#!/bin/sh
EXAMPLE_DIRECTORY="$1"
cd examples/$EXAMPLE_DIRECTORY; cargo run
