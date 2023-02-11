# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [groth16](#groth16)
    - [pairing_bls12_381](#pairing_bls12_381)
    - [msm_g1_bls12_381](#msm_g1_bls12_381)
    - [msm_g2_bls12_381](#msm_g2_bls12_381)

## Benchmark Results

### groth16

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `6.39 ms` (✅ **1.00x**) | `7.38 ms` (❌ *1.16x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.05 ms` (✅ **1.00x**) | `2.32 ms` (❌ *1.13x slower*)    |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.64 s` (✅ **1.00x**) | `2.75 s` (✅ **1.04x slower**)  |

### msm_g2_bls12_381

|        | `normal`                |
|:-------|:----------------------- |
|        | `7.99 s` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

