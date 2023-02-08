# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_edonbls12_381_optimized](#sample_edonbls12_381_optimized)
    - [arithmetic_for_edonbls12_381_optimized](#arithmetic_for_edonbls12_381_optimized)
    - [serialization_for_edonbls12_381_optimized](#serialization_for_edonbls12_381_optimized)
    - [msm_for_edonbls12_381_optimized](#msm_for_edonbls12_381_optimized)
    - [squareroot_for_edonbls12_381_optimized](#squareroot_for_edonbls12_381_optimized)
    - [bitwise_operations_for_edonbls12_381_optimized](#bitwise_operations_for_edonbls12_381_optimized)
    - [conversions_for_edonbls12_381_optimized](#conversions_for_edonbls12_381_optimized)

## Benchmark Results

### sample_edonbls12_381_optimized

|        | `goptimized_elements`           |
|:-------|:------------------------------- |
|        | `66.54 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.75 ns` (✅ **1.00x**) | `8.69 ns` (🚀 **44.49x faster**) | `8.62 ns` (🚀 **44.85x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.21 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.21x faster**) | `8.80 ns` (🚀 **46.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `401.38 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `410.75 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `294.93 ns` (✅ **1.00x**) | `5.81 ns` (🚀 **50.75x faster**) | `5.80 ns` (🚀 **50.83x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.72 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.77 ns` (✅ **1.00x faster**) | `42.83 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.34 ns` (✅ **1.02x slower**) | `34.60 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.92 us` (✅ **1.02x faster**)  | `7.03 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.38 ns` (✅ **1.01x faster**) | `61.70 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.04 ns` (✅ **1.01x faster**) | `89.95 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.59 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.97 ns` (✅ **1.00x**)  | `30.99 ns` (🚀 **2.19x faster**)    | `31.45 ns` (🚀 **2.16x faster**)     |
| **`serialize_uncompressed`**             | `58.09 ns` (✅ **1.00x**)  | `31.12 ns` (🚀 **1.87x faster**)    | `30.57 ns` (🚀 **1.90x faster**)     |
| **`deserialize_compressed`**             | `182.12 us` (✅ **1.00x**) | `49.78 ns` (🚀 **3658.76x faster**) | `52.64 ns` (🚀 **3459.49x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.53 us` (✅ **1.00x**)  | `49.76 ns` (🚀 **774.42x faster**)  | `52.65 ns` (🚀 **731.84x faster**)   |
| **`deserialize_uncompressed`**           | `143.54 us` (✅ **1.00x**) | `49.72 ns` (🚀 **2887.29x faster**) | `52.56 ns` (🚀 **2731.19x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `165.74 ns` (✅ **1.00x**) | `49.68 ns` (🚀 **3.34x faster**)    | `52.56 ns` (🚀 **3.15x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.16 us` (❌ *2.56x slower*)    |
| **`legendre_for_qr`**    | `12.30 us` (✅ **1.00x**) | `10.87 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `4.84 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.49 ns` (✅ **1.00x**)       | `48.53 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.54 ns` (✅ **1.00x**)       | `48.56 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.99 ns` (✅ **1.00x**)        | `4.99 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.98 ns` (✅ **1.00x**) | `41.14 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `23.26 ns` (✅ **1.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

