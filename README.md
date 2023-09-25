# Substrate Arkworks Examples

Examples using Arkworks curve arithmetic via [Substrate](https://github.com/paritytech/polkadot-sdk/) framework.

The benchmarks are offered in two flavors:
- Using the upstream Arkworks curves configuration: in this way the full set of curves arithmetic is built
  and run in wasm directly by the pallet code.
- Using the optimized [Substrate host functions](https://github.com/paritytech/
  substrate-curves): in this way some of the more computationally expensive
  curves arithmetic operations are delegated to the host to be run at native
  speed.

Host functions are provided for:
- Pairings
- Multi scalar multiplication.
- Point scalar multiplication.

## Benchmarking

You can run the included benchmarks of the Substrate extrinsics on your local machine with:

```shell
cargo build -p node-ark-demo --features runtime benchmarks

# List all avaiable benchmarks
./benchmark.sh

# Run all 'ark-pallet' benchmarks
./benchmarks.sh ark-pallet

# Run a specific 'ark-pallet' benchmark (e.g. 'ed_on_bls12_377_mul_affine_opt')
./benchmarks.sh ark-pallet ed_on_bls12_377_mul_affine_opt
```

The results are then written in the `results/<datetime>/results.csv` file.
