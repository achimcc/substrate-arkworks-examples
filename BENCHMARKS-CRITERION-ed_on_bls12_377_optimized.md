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
|        | `66.86 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `385.94 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.34x faster**) | `8.64 ns` (🚀 **44.65x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `407.22 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.30x faster**) | `8.80 ns` (🚀 **46.26x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `399.53 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `408.63 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `294.50 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.27x faster**) | `5.81 ns` (🚀 **50.67x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.35 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.81 ns` (✅ **1.00x faster**) | `42.82 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.93 ns` (✅ **1.02x slower**) | `35.27 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.08 us` (✅ **1.01x slower**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.62 ns` (✅ **1.01x faster**) | `61.96 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.02 ns` (✅ **1.01x faster**) | `89.82 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `7.63 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `8.67 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.22 ns` (✅ **1.00x**)  | `33.63 ns` (🚀 **2.03x faster**)    | `31.35 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `57.48 ns` (✅ **1.00x**)  | `33.38 ns` (✅ **1.72x faster**)    | `31.63 ns` (🚀 **1.82x faster**)     |
| **`deserialize_compressed`**             | `182.54 us` (✅ **1.00x**) | `51.88 ns` (🚀 **3518.56x faster**) | `52.84 ns` (🚀 **3454.83x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.60 us` (✅ **1.00x**)  | `51.79 ns` (🚀 **745.34x faster**)  | `52.83 ns` (🚀 **730.70x faster**)   |
| **`deserialize_uncompressed`**           | `143.88 us` (✅ **1.00x**) | `51.65 ns` (🚀 **2785.98x faster**) | `52.66 ns` (🚀 **2732.35x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.20 ns` (✅ **1.00x**) | `52.09 ns` (🚀 **3.15x faster**)    | `52.64 ns` (🚀 **3.12x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.34 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.10 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.88 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `47.97 ns` (✅ **1.00x**)       | `48.43 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `47.98 ns` (✅ **1.00x**)       | `48.42 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.45 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.90 ns` (✅ **1.00x**) | `40.65 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `23.76 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

