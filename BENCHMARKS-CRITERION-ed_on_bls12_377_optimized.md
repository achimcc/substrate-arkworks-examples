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
|        | `60.00 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `378.58 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.70x faster**) | `8.13 ns` (🚀 **46.56x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `395.66 ns` (✅ **1.00x**) | `8.59 ns` (🚀 **46.09x faster**) | `8.59 ns` (🚀 **46.07x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `387.40 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `393.40 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `290.06 ns` (✅ **1.00x**) | `5.30 ns` (🚀 **54.75x faster**) | `5.30 ns` (🚀 **54.78x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `131.43 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.30 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.71 ns` (✅ **1.00x slower**) | `31.63 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.56 us` (✅ **1.05x slower**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.03 ns` (✅ **1.00x faster**) | `53.19 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.55 ns` (✅ **1.01x faster**) | `80.73 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `6.52 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `63.87 ns` (✅ **1.00x**)  | `28.05 ns` (🚀 **2.28x faster**)    | `27.85 ns` (🚀 **2.29x faster**)     |
| **`serialize_uncompressed`**             | `55.08 ns` (✅ **1.00x**)  | `27.93 ns` (🚀 **1.97x faster**)    | `27.74 ns` (🚀 **1.99x faster**)     |
| **`deserialize_compressed`**             | `163.58 us` (✅ **1.00x**) | `46.56 ns` (🚀 **3513.57x faster**) | `45.11 ns` (🚀 **3625.95x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.56 us` (✅ **1.00x**)  | `46.21 ns` (🚀 **747.79x faster**)  | `45.11 ns` (🚀 **766.05x faster**)   |
| **`deserialize_uncompressed`**           | `129.29 us` (✅ **1.00x**) | `46.48 ns` (🚀 **2781.50x faster**) | `45.74 ns` (🚀 **2826.76x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.12 ns` (✅ **1.00x**) | `46.48 ns` (🚀 **3.12x faster**)    | `45.73 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.41 us` (✅ **1.00x**) | `27.58 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.71 us` (✅ **1.00x**) | `9.52 us` (✅ **1.12x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.82 ns` (✅ **1.00x**)       | `61.31 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `60.87 ns` (✅ **1.00x**)       | `61.22 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)        | `4.08 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.48 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.93 ns` (✅ **1.00x**) | `35.98 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.71 ns` (✅ **1.00x**) | `21.66 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

