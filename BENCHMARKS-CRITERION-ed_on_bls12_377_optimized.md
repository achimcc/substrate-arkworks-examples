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
|        | `66.55 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `388.90 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.66x faster**) | `8.66 ns` (🚀 **44.92x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.43 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.20x faster**) | `8.80 ns` (🚀 **46.21x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `400.98 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `404.80 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `296.43 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.59x faster**) | `5.81 ns` (🚀 **51.01x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.22 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.16 ns` (✅ **1.00x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.38 ns` (✅ **1.02x slower**) | `42.66 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.53 ns` (✅ **1.00x slower**) | `35.36 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.94 us` (✅ **1.00x faster**)  | `6.96 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.35 ns` (✅ **1.00x faster**) | `61.66 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `91.51 ns` (✅ **1.01x faster**) | `92.27 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.65 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**)        | `8.72 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.71 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.79 ns` (✅ **1.00x**)  | `30.17 ns` (🚀 **2.25x faster**)    | `30.86 ns` (🚀 **2.20x faster**)     |
| **`serialize_uncompressed`**             | `59.81 ns` (✅ **1.00x**)  | `31.09 ns` (🚀 **1.92x faster**)    | `30.38 ns` (🚀 **1.97x faster**)     |
| **`deserialize_compressed`**             | `181.78 us` (✅ **1.00x**) | `49.41 ns` (🚀 **3679.15x faster**) | `52.06 ns` (🚀 **3491.73x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.53 us` (✅ **1.00x**)  | `49.36 ns` (🚀 **780.50x faster**)  | `52.10 ns` (🚀 **739.52x faster**)   |
| **`deserialize_uncompressed`**           | `143.29 us` (✅ **1.00x**) | `49.25 ns` (🚀 **2909.52x faster**) | `52.08 ns` (🚀 **2751.46x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.67 ns` (✅ **1.00x**) | `49.30 ns` (🚀 **3.38x faster**)    | `52.04 ns` (🚀 **3.20x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.16 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.29 us` (✅ **1.00x**) | `10.87 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.86 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.17 ns` (✅ **1.00x**)       | `48.13 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.10 ns` (✅ **1.00x**)       | `48.11 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.98 ns` (✅ **1.00x**) | `40.99 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `23.10 ns` (✅ **1.00x**) | `22.52 ns` (✅ **1.03x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

