#!/bin/bash

steps=50
repeat=3

binary="./target/release/node-ark-demo"
# binary="./target/debug/node-ark-demo"

# results_dir="./results/$(date +'%Y%m%d-%H%M%S')"
results_dir="./results"
rm -rf $results_dir
# Create results folder if not exists
mkdir -p "$results_dir"

if [[ $1 == "" ]]; then
    $binary benchmark pallet --list
    exit 0
fi
    
pallet="$1"
extrinsic='*'

if [[ $2 != "" ]]; then
    extrinsic="$2"
fi

# Do the benchmarking
$binary benchmark pallet \
    --chain=dev \
    --pallet=$pallet \
    --extrinsic="$extrinsic" \
    --steps=$steps \
    --repeat=$repeat \
    --no-storage-info \
    --template=frame-weight-template.hbs \
    --json-file=$results_dir/results.json \
    --output=$results_dir/weights.rs
