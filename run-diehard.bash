#!/usr/bin/env bash

cargo build -r && \
cp -v ./target/release/suckhard .

for number_test in $(seq 0 16); do
  ./suckhard >output-ctr-drbg.bin && \
  ./dieharder -g 200 -d "${number_test}" <output-ctr-drbg.bin | tail -1
done

rm -f output-ctr-drbg.bin
