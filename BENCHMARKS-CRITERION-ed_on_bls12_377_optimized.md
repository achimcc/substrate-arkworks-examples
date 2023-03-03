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
|        | `66.92 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.01 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.30x faster**) | `8.64 ns` (🚀 **44.66x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `405.53 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.10x faster**) | `8.79 ns` (🚀 **46.14x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `398.60 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `408.80 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.15 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.37x faster**) | `5.82 ns` (🚀 **50.75x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.53 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.18 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.84 ns` (✅ **1.00x faster**) | `42.85 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.91 ns` (✅ **1.02x slower**) | `35.28 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.02 us` (✅ **1.00x faster**)  | `7.03 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.60 ns` (✅ **1.01x faster**) | `61.98 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.06 ns` (✅ **1.01x faster**) | `89.83 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `7.63 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.27 ns` (✅ **1.00x**)  | `30.40 ns` (🚀 **2.25x faster**)    | `31.34 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `57.52 ns` (✅ **1.00x**)  | `31.88 ns` (🚀 **1.80x faster**)    | `31.17 ns` (🚀 **1.85x faster**)     |
| **`deserialize_compressed`**             | `182.55 us` (✅ **1.00x**) | `53.46 ns` (🚀 **3414.76x faster**) | `52.84 ns` (🚀 **3454.55x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.63 us` (✅ **1.00x**)  | `53.49 ns` (🚀 **722.13x faster**)  | `52.82 ns` (🚀 **731.23x faster**)   |
| **`deserialize_uncompressed`**           | `143.83 us` (✅ **1.00x**) | `53.53 ns` (🚀 **2687.02x faster**) | `52.74 ns` (🚀 **2727.19x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `162.99 ns` (✅ **1.00x**) | `53.43 ns` (🚀 **3.05x faster**)    | `52.75 ns` (🚀 **3.09x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.10 us` (❌ *2.56x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.88 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.15 ns` (✅ **1.00x**)       | `48.16 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `47.96 ns` (✅ **1.00x**)       | `48.14 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.43 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `40.61 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.81 ns` (✅ **1.00x**) | `23.76 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

