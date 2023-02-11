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
|        | `196.30 us` (✅ **1.00x**)                  |

### arithmetic_for_bls12_381_optimized

|                             | `g1projectivebls12_381`           |
|:----------------------------|:--------------------------------- |
| **`addition`**              | `1.11 us` (✅ **1.00x**)           |
| **`subtraction`**           | `1.15 us` (✅ **1.00x**)           |
| **`mixed_addition`**        | `816.24 ns` (✅ **1.00x**)         |
| **`mixed_subtraction`**     | `836.89 ns` (✅ **1.00x**)         |
| **`double`**                | `562.86 ns` (✅ **1.00x**)         |
| **`scalar_multiplication`** | `297.59 us` (✅ **1.00x**)         |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`           |
|:-----------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `142.03 ns` (✅ **1.00x**)         |
| **`serialize_uncompressed`**             | `181.17 ns` (✅ **1.00x**)         |
| **`deserialize_compressed`**             | `147.64 us` (✅ **1.00x**)         |
| **`deserialize_compressed_unchecked`**   | `36.23 us` (✅ **1.00x**)          |
| **`deserialize_uncompressed`**           | `111.34 us` (✅ **1.00x**)         |
| **`deserialize_uncompressed_unchecked`** | `232.18 ns` (✅ **1.00x**)         |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`           |
|:-------|:--------------------------------- |
|        | `2.25 s` (✅ **1.00x**)            |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

