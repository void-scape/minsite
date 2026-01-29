#!/bin/bash

set -e

GALLERY='static/mandelbrot-gallery'

rm -rf $GALLERY/frames
cargo run -p fract --release --bin collage -- -c $GALLERY/configs -d $GALLERY/frames
