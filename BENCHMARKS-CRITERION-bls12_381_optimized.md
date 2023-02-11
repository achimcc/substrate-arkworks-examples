# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381_optimized](#sample_bls12_381_optimized)
    - [arithmetic_for_bls12_381_optimized](#arithmetic_for_bls12_381_optimized)
    - [serialization_for_bls12_381_optimized](#serialization_for_bls12_381_optimized)
    - [msm_for_bls12_381_optimized](#msm_for_bls12_381_optimized)

## Benchmark Results

### sample_bls12_381_optimized

|        | `g1projectivebls12_381_elements`           |
|:-------|:------------------------------------------ |
|        | `219.91 us` (✅ **1.00x**)                  |

### arithmetic_for_bls12_381_optimized

|                             | `g1projectivebls12_381`           |
|:----------------------------|:--------------------------------- |
| **`addition`**              | `1.25 us` (✅ **1.00x**)           |
| **`subtraction`**           | `1.29 us` (✅ **1.00x**)           |
| **`mixed_addition`**        | `899.25 ns` (✅ **1.00x**)         |
| **`mixed_subtraction`**     | `927.03 ns` (✅ **1.00x**)         |
| **`double`**                | `591.27 ns` (✅ **1.00x**)         |
| **`scalar_multiplication`** | `339.50 us` (✅ **1.00x**)         |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`           |
|:-----------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.92 ns` (✅ **1.00x**)         |
| **`serialize_uncompressed`**             | `192.22 ns` (✅ **1.00x**)         |
| **`deserialize_compressed`**             | `165.25 us` (✅ **1.00x**)         |
| **`deserialize_compressed_unchecked`**   | `39.11 us` (✅ **1.00x**)          |
| **`deserialize_uncompressed`**           | `125.82 us` (✅ **1.00x**)         |
| **`deserialize_uncompressed_unchecked`** | `223.52 ns` (✅ **1.00x**)         |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`           |
|:-------|:--------------------------------- |
|        | `2.35 s` (✅ **1.00x**)            |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

