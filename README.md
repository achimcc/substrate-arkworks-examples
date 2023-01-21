# Substrate Node Template

We provide example implementations of ellipic curve operations for arkworks curves using the curve from [ark-substrate](https://github.com/paritytech/ark-substrate) as well as the native arkworks [curves](https://github.com/arkworks-rs/curves). This allows to compare their benchmarks and to measure the preformance improvements of `ark-substrate` curves over `ark-curves` which are obtained by replacing costly arithmetic operations by host function calls into native code.

## build

`cargo build --release`

## benachmark

`cargo build --package node-template --profile production --features runtime-benchmarks`

`./target/production/node-template benchmark pallet pallet_template --extrinsic "*"`
