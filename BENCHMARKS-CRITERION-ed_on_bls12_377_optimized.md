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
|        | `66.47 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `388.58 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.63x faster**) | `8.63 ns` (🚀 **45.01x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `408.33 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.41x faster**) | `8.79 ns` (🚀 **46.45x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `400.18 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `408.99 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `294.22 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.22x faster**) | `5.87 ns` (🚀 **50.09x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.18 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.85 ns` (✅ **1.00x slower**) | `42.78 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.42 ns` (✅ **1.01x slower**) | `35.03 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.88 us` (✅ **1.02x faster**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.58 ns` (✅ **1.00x slower**) | `61.55 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `88.97 ns` (✅ **1.01x faster**) | `89.82 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.75 ns` (✅ **1.00x**)        | `8.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.41 ns` (✅ **1.00x**)  | `31.00 ns` (🚀 **2.21x faster**)    | `31.30 ns` (🚀 **2.19x faster**)     |
| **`serialize_uncompressed`**             | `62.25 ns` (✅ **1.00x**)  | `30.40 ns` (🚀 **2.05x faster**)    | `31.04 ns` (🚀 **2.01x faster**)     |
| **`deserialize_compressed`**             | `181.87 us` (✅ **1.00x**) | `51.04 ns` (🚀 **3563.59x faster**) | `52.07 ns` (🚀 **3492.56x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.47 us` (✅ **1.00x**)  | `50.98 ns` (🚀 **754.59x faster**)  | `52.18 ns` (🚀 **737.16x faster**)   |
| **`deserialize_uncompressed`**           | `143.41 us` (✅ **1.00x**) | `50.81 ns` (🚀 **2822.26x faster**) | `52.08 ns` (🚀 **2753.62x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.64 ns` (✅ **1.00x**) | `50.76 ns` (🚀 **3.24x faster**)    | `52.08 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.20 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.70 ns` (✅ **1.00x**)       | `48.67 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.70 ns` (✅ **1.00x**)       | `48.72 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)        | `5.42 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.86 ns` (✅ **1.00x**) | `40.77 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.73 ns` (✅ **1.00x**) | `22.52 ns` (✅ **1.05x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

