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
|        | `66.52 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.82 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.41x faster**) | `8.63 ns` (🚀 **44.80x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.63 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.25x faster**) | `8.79 ns` (🚀 **46.24x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `401.77 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `416.20 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.04 ns` (✅ **1.00x**) | `5.81 ns` (🚀 **50.79x faster**) | `5.85 ns` (🚀 **50.46x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.73 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.14 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.70 ns` (✅ **1.00x faster**) | `42.86 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.34 ns` (✅ **1.02x slower**) | `34.65 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.88 us` (✅ **1.02x faster**)  | `7.02 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.38 ns` (✅ **1.01x faster**) | `61.73 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.12 ns` (✅ **1.01x faster**) | `89.97 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `8.65 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.59 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.10 ns` (✅ **1.00x**)  | `30.93 ns` (🚀 **2.20x faster**)    | `31.33 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `58.30 ns` (✅ **1.00x**)  | `31.43 ns` (🚀 **1.85x faster**)    | `30.52 ns` (🚀 **1.91x faster**)     |
| **`deserialize_compressed`**             | `182.14 us` (✅ **1.00x**) | `50.46 ns` (🚀 **3609.85x faster**) | `52.39 ns` (🚀 **3476.27x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.56 us` (✅ **1.00x**)  | `49.75 ns` (🚀 **774.98x faster**)  | `52.39 ns` (🚀 **735.97x faster**)   |
| **`deserialize_uncompressed`**           | `143.60 us` (✅ **1.00x**) | `49.68 ns` (🚀 **2890.51x faster**) | `52.26 ns` (🚀 **2748.02x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `165.34 ns` (✅ **1.00x**) | `49.83 ns` (🚀 **3.32x faster**)    | `52.26 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.16 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.30 us` (✅ **1.00x**) | `10.87 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.21 ns` (✅ **1.00x**)       | `48.25 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.14 ns` (✅ **1.00x**)       | `48.17 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.99 ns` (✅ **1.00x**)        | `5.00 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.98 ns` (✅ **1.00x**) | `40.73 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.77 ns` (✅ **1.00x**) | `23.31 ns` (✅ **1.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

