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
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.45 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.37x faster**) | `8.63 ns` (🚀 **44.76x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.34 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.22x faster**) | `8.80 ns` (🚀 **46.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `401.41 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `410.79 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.11 ns` (✅ **1.00x**) | `5.82 ns` (🚀 **50.75x faster**) | `5.85 ns` (🚀 **50.45x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.81 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.14 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.72 ns` (✅ **1.00x faster**) | `42.86 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.41 ns` (✅ **1.02x slower**) | `34.64 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.02x faster**)  | `7.03 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.40 ns` (✅ **1.01x faster**) | `61.72 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.09 ns` (✅ **1.01x faster**) | `90.04 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `8.65 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.70 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.59 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.94 ns` (✅ **1.00x**)  | `30.94 ns` (🚀 **2.20x faster**)    | `31.24 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `58.44 ns` (✅ **1.00x**)  | `31.18 ns` (🚀 **1.87x faster**)    | `30.34 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `182.29 us` (✅ **1.00x**) | `49.82 ns` (🚀 **3658.59x faster**) | `52.28 ns` (🚀 **3486.44x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.55 us` (✅ **1.00x**)  | `50.41 ns` (🚀 **764.65x faster**)  | `52.27 ns` (🚀 **737.42x faster**)   |
| **`deserialize_uncompressed`**           | `143.57 us` (✅ **1.00x**) | `50.49 ns` (🚀 **2843.26x faster**) | `52.32 ns` (🚀 **2744.02x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `165.63 ns` (✅ **1.00x**) | `49.72 ns` (🚀 **3.33x faster**)    | `52.27 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.16 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.31 us` (✅ **1.00x**) | `10.88 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.37 ns` (✅ **1.00x**)       | `48.19 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.26 ns` (✅ **1.00x**)       | `48.26 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)        | `4.99 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.36 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.99 ns` (✅ **1.00x**) | `40.77 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.78 ns` (✅ **1.00x**) | `23.31 ns` (✅ **1.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

