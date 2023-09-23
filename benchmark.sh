#!/bin/bash

binary="./target/debug/node-ark-demo"

action=$1

# export RUST_LOG="sassafras=debug"

case $action in
    "list")
        $binary benchmark pallet --list
        ;;
    "run")
        pallet='*'
        extrinsic='*'
        if [[ $2 != "" ]]; then
            pallet=$2
            if [[ $3 != "" ]]; then
                extrinsic=$3
            fi
        fi
        $binary benchmark pallet \
            --chain dev \
            --pallet $pallet \
            --extrinsic "$extrinsic" \
            --steps 3 \
            --repeat 3 \
            --output weights.rs
            ;;

    *)
        echo "Usage: benchmark <command>"
        echo "  Commands:"
        echo "  - list"
        echo "  - run [pallet] [extrinsic]"
        ;;
esac
