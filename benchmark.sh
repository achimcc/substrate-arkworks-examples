#!/bin/bash

binary="./target/release/node-ark-demo"
# binary="./target/debug/node-ark-demo"

NO_DETAILS="--no-min-squares"
# NO_DETAILS="--no-median-slopes --no-min-squares"

steps=3
repeat=3

if [[ $1 == "" ]]; then
    $binary benchmark pallet --list
    exit 0
fi
    
pallet="$1"
extrinsic='*'

if [[ $2 != "" ]]; then
    extrinsic="$2"
fi

$binary benchmark pallet \
    $NO_DETAILS \
    --chain dev \
    --pallet $pallet \
    --extrinsic "$extrinsic" \
    --steps $steps \
    --repeat $repeat \
    --output weights.rs
