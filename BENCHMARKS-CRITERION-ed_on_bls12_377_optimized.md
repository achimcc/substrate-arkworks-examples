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
|        | `66.44 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `384.64 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.21x faster**) | `8.65 ns` (🚀 **44.48x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `404.51 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **45.94x faster**) | `8.79 ns` (🚀 **46.03x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `401.02 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `411.36 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `299.77 ns` (✅ **1.00x**) | `5.83 ns` (🚀 **51.46x faster**) | `5.89 ns` (🚀 **50.86x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.87 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.15 ns` (✅ **1.00x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.27 ns` (✅ **1.00x faster**) | `43.38 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.83 ns` (✅ **1.03x slower**) | `34.96 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.01x faster**)  | `6.98 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.52 ns` (✅ **1.00x faster**) | `61.75 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `88.96 ns` (✅ **1.01x faster**) | `89.93 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.63 ns` (✅ **1.00x**)        | `8.64 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.71 ns` (✅ **1.00x**)  | `30.52 ns` (🚀 **2.25x faster**)    | `30.72 ns` (🚀 **2.24x faster**)     |
| **`serialize_uncompressed`**             | `61.51 ns` (✅ **1.00x**)  | `30.88 ns` (🚀 **1.99x faster**)    | `30.38 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `182.55 us` (✅ **1.00x**) | `51.03 ns` (🚀 **3577.55x faster**) | `51.85 ns` (🚀 **3520.61x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.41 us` (✅ **1.00x**)  | `51.01 ns` (🚀 **752.96x faster**)  | `51.86 ns` (🚀 **740.64x faster**)   |
| **`deserialize_uncompressed`**           | `144.02 us` (✅ **1.00x**) | `50.92 ns` (🚀 **2828.50x faster**) | `51.93 ns` (🚀 **2773.24x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.13 ns` (✅ **1.00x**) | `50.87 ns` (🚀 **3.23x faster**)    | `51.94 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.32 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.08 us` (❌ *2.56x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.85 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.10 ns` (✅ **1.00x**)       | `48.18 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.11 ns` (✅ **1.00x**)       | `48.13 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)        | `5.45 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.89 ns` (✅ **1.00x**) | `40.78 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `22.96 ns` (✅ **1.00x**) | `23.75 ns` (✅ **1.03x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

