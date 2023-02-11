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
|        | `6.04 ms` (✅ **1.00x**) | `6.91 ms` (❌ *1.14x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.88 ms` (✅ **1.00x**) | `2.12 ms` (❌ *1.12x slower*)    |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.46 s` (✅ **1.00x**) | `2.56 s` (✅ **1.04x slower**)  |

### msm_g2_bls12_381

|        | `normal`                |
|:-------|:----------------------- |
|        | `7.30 s` (✅ **1.00x**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

