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
|        | `66.56 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `389.12 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.65x faster**) | `8.65 ns` (🚀 **44.98x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `412.09 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.78x faster**) | `8.78 ns` (🚀 **46.93x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `401.17 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `421.06 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `297.58 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.74x faster**) | `5.89 ns` (🚀 **50.51x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.45 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.16 ns` (✅ **1.00x slower**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.10 ns` (✅ **1.00x slower**) | `43.00 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.91 ns` (✅ **1.02x slower**) | `35.35 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.02x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.37 ns` (✅ **1.00x faster**) | `61.65 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.04 ns` (✅ **1.01x faster**) | `89.83 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**)        | `8.64 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.57 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.71 ns` (✅ **1.00x**)  | `30.59 ns` (🚀 **2.21x faster**)    | `31.23 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `58.23 ns` (✅ **1.00x**)  | `31.44 ns` (🚀 **1.85x faster**)    | `30.74 ns` (🚀 **1.89x faster**)     |
| **`deserialize_compressed`**             | `183.39 us` (✅ **1.00x**) | `49.91 ns` (🚀 **3674.26x faster**) | `52.11 ns` (🚀 **3519.55x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.49 us` (✅ **1.00x**)  | `50.03 ns` (🚀 **769.31x faster**)  | `52.10 ns` (🚀 **738.73x faster**)   |
| **`deserialize_uncompressed`**           | `144.79 us` (✅ **1.00x**) | `50.07 ns` (🚀 **2891.79x faster**) | `51.92 ns` (🚀 **2788.65x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.65 ns` (✅ **1.00x**) | `49.88 ns` (🚀 **3.30x faster**)    | `51.99 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.29 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.14 us` (✅ **1.00x**) | `31.17 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.29 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.27 ns` (✅ **1.00x**)       | `48.25 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.13 ns` (✅ **1.00x**)       | `48.33 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)        | `5.45 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.92 ns` (✅ **1.00x**) | `40.69 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `22.84 ns` (✅ **1.00x**) | `23.30 ns` (✅ **1.02x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

