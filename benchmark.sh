#!/bin/bash

repeat=20
steps=50

pallet="pallet_ark"

binary="./target/release/node-ark"
# binary="./target/debug/node-ark"

results_dir="./results"
# results_dir="./results/$(date +'%Y%m%d-%H%M%S')"

# Create results folder if not exists
mkdir -p "$results_dir"

extrinsic="$1"

if [[ $extrinsic == "" ]]; then
    echo "Usage ./benchmark.sh <extrinsic>"
    echo ""
    echo "Available extrinsics for '$pallet':"
    $binary benchmark pallet --pallet $pallet --list | grep $pallet | awk '{ print "- " $2 }'
    echo ""
    echo "Use 'all' to run all benchmarks"
    exit 0
fi

if [[ $extrinsic == "all" ]]; then
    extrinsic='*'
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
