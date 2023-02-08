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
|        | `66.53 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.79 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.40x faster**) | `8.63 ns` (🚀 **44.84x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.65 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.21x faster**) | `8.80 ns` (🚀 **46.21x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `401.69 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `410.48 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `294.89 ns` (✅ **1.00x**) | `5.81 ns` (🚀 **50.74x faster**) | `5.82 ns` (🚀 **50.63x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.67 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.82 ns` (✅ **1.00x slower**) | `42.79 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.40 ns` (✅ **1.02x slower**) | `34.61 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.89 us` (✅ **1.02x faster**)  | `7.03 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.40 ns` (✅ **1.01x faster**) | `61.72 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.05 ns` (✅ **1.01x faster**) | `89.97 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `8.64 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.58 ns` (✅ **1.00x**)        | `4.59 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.98 ns` (✅ **1.00x**)  | `30.99 ns` (🚀 **2.19x faster**)    | `31.08 ns` (🚀 **2.19x faster**)     |
| **`serialize_uncompressed`**             | `58.49 ns` (✅ **1.00x**)  | `31.21 ns` (🚀 **1.87x faster**)    | `30.48 ns` (🚀 **1.92x faster**)     |
| **`deserialize_compressed`**             | `182.26 us` (✅ **1.00x**) | `50.91 ns` (🚀 **3579.86x faster**) | `52.30 ns` (🚀 **3485.10x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.53 us` (✅ **1.00x**)  | `50.92 ns` (🚀 **756.63x faster**)  | `52.29 ns` (🚀 **736.79x faster**)   |
| **`deserialize_uncompressed`**           | `143.69 us` (✅ **1.00x**) | `50.81 ns` (🚀 **2828.00x faster**) | `52.33 ns` (🚀 **2745.94x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `165.60 ns` (✅ **1.00x**) | `50.83 ns` (🚀 **3.26x faster**)    | `52.33 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.18 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.30 us` (✅ **1.00x**) | `10.87 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `49.30 ns` (✅ **1.00x**)       | `49.70 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `49.42 ns` (✅ **1.00x**)       | `49.51 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.99 ns` (✅ **1.00x**)        | `5.00 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.36 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.14 ns` (✅ **1.00x**) | `40.79 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `23.32 ns` (✅ **1.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

