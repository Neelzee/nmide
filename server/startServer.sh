#!/bin/bash
rm *.js
cd ../core && node build.js
cd ../server
cp ../core/build/index.js .
cp ../core/build/app/setup.js .
npm run dev &
cargo run
