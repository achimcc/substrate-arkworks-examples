#!/bin/bash

steps=2
repeat=1

pallet="pallet_ark_demo"

binary="./target/release/node-ark-demo"
# binary="./target/debug/node-ark-demo"

# results_dir="./results/$(date +'%Y%m%d-%H%M%S')"
results_dir="./results"
rm -rf $results_dir
# Create results folder if not exists
mkdir -p "$results_dir"

extrinsic="$1"

if [[ $extrinsic == "" ]]; then
    echo "Usage ./benchmark.sh <extrinsic>"
    echo ""
    echo "Available extrinsics for '$pallet':"
    $binary benchmark pallet --pallet $pallet --list | grep $pallet | awk '{ print "- " $2 }'
    echo ""
    echo "Use '*' to run all benchmarks"
    exit 0
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
