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
|        | `66.41 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.04 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.27x faster**) | `8.62 ns` (🚀 **44.77x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.27 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.16x faster**) | `8.80 ns` (🚀 **46.14x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `400.38 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `409.81 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `294.52 ns` (✅ **1.00x**) | `5.88 ns` (🚀 **50.07x faster**) | `5.84 ns` (🚀 **50.41x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.46 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.71 ns` (✅ **1.01x slower**) | `43.22 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.50 ns` (✅ **1.02x faster**) | `36.20 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.02x faster**)  | `7.02 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.54 ns` (✅ **1.00x faster**) | `61.85 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.17 ns` (✅ **1.01x faster**) | `89.77 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.62 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**)        | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**)        | `4.78 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.68 ns` (✅ **1.00x**)  | `31.27 ns` (🚀 **2.16x faster**)    | `32.06 ns` (🚀 **2.11x faster**)     |
| **`serialize_uncompressed`**             | `58.38 ns` (✅ **1.00x**)  | `31.92 ns` (🚀 **1.83x faster**)    | `31.00 ns` (🚀 **1.88x faster**)     |
| **`deserialize_compressed`**             | `182.90 us` (✅ **1.00x**) | `50.43 ns` (🚀 **3627.14x faster**) | `52.13 ns` (🚀 **3508.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.46 us` (✅ **1.00x**)  | `50.39 ns` (🚀 **763.28x faster**)  | `52.11 ns` (🚀 **738.15x faster**)   |
| **`deserialize_uncompressed`**           | `144.40 us` (✅ **1.00x**) | `50.25 ns` (🚀 **2873.75x faster**) | `51.87 ns` (🚀 **2784.04x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `165.50 ns` (✅ **1.00x**) | `50.44 ns` (🚀 **3.28x faster**)    | `51.90 ns` (🚀 **3.19x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.29 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.93 us` (✅ **1.00x**) | `31.16 us` (❌ *2.61x slower*)    |
| **`legendre_for_qr`**    | `12.20 us` (✅ **1.00x**) | `10.89 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.88 ns` (✅ **1.00x**)       | `48.78 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.84 ns` (✅ **1.00x**)       | `48.87 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)        | `5.39 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.84 ns` (✅ **1.00x**) | `40.58 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `22.79 ns` (✅ **1.00x**) | `23.23 ns` (✅ **1.02x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

