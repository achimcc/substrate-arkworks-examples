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
|        | `59.21 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `377.82 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.62x faster**) | `8.13 ns` (🚀 **46.45x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `392.89 ns` (✅ **1.00x**) | `8.64 ns` (🚀 **45.47x faster**) | `8.63 ns` (🚀 **45.52x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `387.59 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `393.67 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `290.78 ns` (✅ **1.00x**) | `9.08 ns` (🚀 **32.01x faster**) | `5.37 ns` (🚀 **54.13x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.77 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.27 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.79 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.24 us` (✅ **1.02x faster**)  | `6.34 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.00 ns` (✅ **1.00x slower**) | `52.83 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.82 ns` (✅ **1.01x faster**) | `80.62 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `63.91 ns` (✅ **1.00x**)  | `27.95 ns` (🚀 **2.29x faster**)    | `28.08 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `54.22 ns` (✅ **1.00x**)  | `27.83 ns` (🚀 **1.95x faster**)    | `28.04 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `163.33 us` (✅ **1.00x**) | `46.58 ns` (🚀 **3506.72x faster**) | `46.36 ns` (🚀 **3522.89x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.26 us` (✅ **1.00x**)  | `46.56 ns` (🚀 **735.68x faster**)  | `46.35 ns` (🚀 **739.04x faster**)   |
| **`deserialize_uncompressed`**           | `128.93 us` (✅ **1.00x**) | `45.48 ns` (🚀 **2835.24x faster**) | `45.23 ns` (🚀 **2850.35x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `144.02 ns` (✅ **1.00x**) | `45.45 ns` (🚀 **3.17x faster**)    | `45.25 ns` (🚀 **3.18x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.38 us` (✅ **1.00x**) | `27.57 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `10.61 us` (✅ **1.00x**) | `9.52 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.67 ns` (✅ **1.00x**)       | `60.69 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.60 ns` (✅ **1.00x**)       | `60.69 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)        | `4.06 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.99 ns` (✅ **1.00x**) | `36.03 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.64 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

