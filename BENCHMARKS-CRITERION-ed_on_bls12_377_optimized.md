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
|        | `60.32 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `379.35 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.77x faster**) | `8.14 ns` (🚀 **46.63x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `395.42 ns` (✅ **1.00x**) | `8.60 ns` (🚀 **45.99x faster**) | `8.72 ns` (🚀 **45.35x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `385.19 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `393.40 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `291.36 ns` (✅ **1.00x**) | `9.11 ns` (🚀 **31.99x faster**) | `5.35 ns` (🚀 **54.50x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.68 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.29 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.79 ns` (✅ **1.01x slower**) | `31.62 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.69 us` (✅ **1.07x slower**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.06 ns` (✅ **1.00x slower**) | `52.83 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `80.02 ns` (✅ **1.01x faster**) | `80.69 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `6.52 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.85 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.93 ns` (✅ **1.00x**)  | `28.06 ns` (🚀 **2.42x faster**)    | `28.07 ns` (🚀 **2.42x faster**)     |
| **`serialize_uncompressed`**             | `54.21 ns` (✅ **1.00x**)  | `27.94 ns` (🚀 **1.94x faster**)    | `28.02 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `164.17 us` (✅ **1.00x**) | `46.73 ns` (🚀 **3512.97x faster**) | `46.27 ns` (🚀 **3547.98x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.74 us` (✅ **1.00x**)  | `46.74 ns` (🚀 **743.28x faster**)  | `46.50 ns` (🚀 **747.08x faster**)   |
| **`deserialize_uncompressed`**           | `129.21 us` (✅ **1.00x**) | `46.70 ns` (🚀 **2766.77x faster**) | `46.47 ns` (🚀 **2780.26x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.71 ns` (✅ **1.00x**) | `46.71 ns` (🚀 **3.08x faster**)    | `46.47 ns` (🚀 **3.09x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.36 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.41 us` (✅ **1.00x**) | `27.57 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.52 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.99 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `62.56 ns` (✅ **1.00x**)       | `62.51 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `61.90 ns` (✅ **1.00x**)       | `62.58 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.08 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.51 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.94 ns` (✅ **1.00x**) | `36.16 ns` (✅ **1.01x slower**)  |
| **`into_bigint`** | `21.75 ns` (✅ **1.00x**) | `21.65 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

