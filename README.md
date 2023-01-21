# Substrate Arkworks Examples

We provide example implementations of ellipic curve operations for arkworks curves using the curve from [ark-substrate](https://github.com/paritytech/ark-substrate) as well as the native arkworks [curves](https://github.com/arkworks-rs/curves). This allows to compare their benchmarks and to measure the preformance improvements of `ark-substrate` curves over `ark-curves` which are obtained by replacing costly arithmetic operations by host function calls into native code.

Wel also implement the verification of a [groth16](https://eprint.iacr.org/2016/260.pdf) proof which we computed and serialized in native code and which gets deserialized and verifyed on-chain in the Substrate pallet using both the native Substrate `BLS12_381` curve and `ark-substrate` `BLS12_381` curve.

## build

Run cargo build command:

```shell
cargo build --release
```

## benachmark

First build for benachmark:

```shell
cargo build --package node-template --profile production --features runtime-benchmarks
```

Then run:

```shell
./target/production/node-template benchmark pallet pallet_template --extrinsic "*"
```
