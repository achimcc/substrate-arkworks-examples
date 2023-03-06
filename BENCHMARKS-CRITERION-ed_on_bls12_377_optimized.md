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
|        | `70.53 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `448.22 ns` (✅ **1.00x**) | `9.87 ns` (🚀 **45.42x faster**)  | `9.64 ns` (🚀 **46.50x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `466.37 ns` (✅ **1.00x**) | `10.28 ns` (🚀 **45.36x faster**) | `10.32 ns` (🚀 **45.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `467.91 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `472.94 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                          | `347.32 ns` (✅ **1.00x**) | `6.34 ns` (🚀 **54.77x faster**)  | `6.33 ns` (🚀 **54.89x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `156.83 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `7.07 ns` (✅ **1.01x faster**)   | `7.11 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `44.25 ns` (✅ **1.01x faster**)  | `44.62 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `38.10 ns` (✅ **1.01x slower**)  | `37.72 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.34 us` (✅ **1.01x faster**)   | `7.43 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `63.29 ns` (✅ **1.01x slower**)  | `62.81 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `97.93 ns` (✅ **1.01x faster**)  | `98.87 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.79 ns` (✅ **1.00x**)        | `7.79 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.27 ns` (✅ **1.00x**)        | `9.34 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.64 ns` (✅ **1.00x**)        | `4.64 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.46 ns` (✅ **1.00x**)        | `4.46 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `76.66 ns` (✅ **1.00x**)  | `33.40 ns` (🚀 **2.30x faster**)    | `33.44 ns` (🚀 **2.29x faster**)     |
| **`serialize_uncompressed`**             | `67.98 ns` (✅ **1.00x**)  | `33.47 ns` (🚀 **2.03x faster**)    | `33.33 ns` (🚀 **2.04x faster**)     |
| **`deserialize_compressed`**             | `195.92 us` (✅ **1.00x**) | `55.61 ns` (🚀 **3523.18x faster**) | `53.86 ns` (🚀 **3637.48x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.95 us` (✅ **1.00x**)  | `55.59 ns` (🚀 **736.69x faster**)  | `53.79 ns` (🚀 **761.36x faster**)   |
| **`deserialize_uncompressed`**           | `153.98 us` (✅ **1.00x**) | `53.79 ns` (🚀 **2862.83x faster**) | `55.29 ns` (🚀 **2784.88x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `174.38 ns` (✅ **1.00x**) | `53.49 ns` (🚀 **3.26x faster**)    | `55.25 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.60 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.46 us` (✅ **1.00x**) | `32.79 us` (❌ *2.63x slower*)    |
| **`legendre_for_qr`**    | `12.65 us` (✅ **1.00x**) | `11.41 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.75 ns` (✅ **1.00x**)        | `4.74 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `72.42 ns` (✅ **1.00x**)       | `72.50 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `72.77 ns` (✅ **1.00x**)       | `72.86 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.38 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.65 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.85 ns` (✅ **1.00x**) | `42.80 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `25.91 ns` (✅ **1.00x**) | `25.91 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

