#!/usr/bin/env bash

rm -f dieharder
rm -f 'dieharder-3.31.1.tgz'
rm -rf 'dieharder-3.31.1'
wget --no-check-certificate 'https://webhome.phy.duke.edu/~rgb/General/dieharder/dieharder-3.31.1.tgz' && \
tar xf dieharder-3.31.1.tgz && \
cd 'dieharder-3.31.1' && \
./configure --prefix="$(pwd)" && \
make install LDFLAGS="-Wl,--allow-multiple-definition -no-undefined"
cp ./bin/dieharder ../
