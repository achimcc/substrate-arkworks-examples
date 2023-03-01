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
| **`addition`**                        | `N/A`                          | `N/A`                          | `387.22 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.45x faster**) | `8.70 ns` (🚀 **44.51x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `406.72 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.23x faster**) | `8.81 ns` (🚀 **46.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `398.38 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `410.55 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.24 ns` (✅ **1.00x**) | `5.85 ns` (🚀 **50.45x faster**) | `5.81 ns` (🚀 **50.83x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.60 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.18 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.90 ns` (✅ **1.00x slower**) | `42.84 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.92 ns` (✅ **1.02x slower**) | `35.05 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.11 us` (✅ **1.01x slower**)  | `7.02 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.57 ns` (✅ **1.01x faster**) | `61.98 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.15 ns` (✅ **1.01x faster**) | `89.77 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `7.63 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**)        | `8.65 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.12 ns` (✅ **1.00x**)  | `30.75 ns` (🚀 **2.22x faster**)    | `31.46 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `58.60 ns` (✅ **1.00x**)  | `32.57 ns` (✅ **1.80x faster**)    | `31.69 ns` (🚀 **1.85x faster**)     |
| **`deserialize_compressed`**             | `182.56 us` (✅ **1.00x**) | `52.21 ns` (🚀 **3496.92x faster**) | `52.86 ns` (🚀 **3453.62x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.63 us` (✅ **1.00x**)  | `51.74 ns` (🚀 **746.75x faster**)  | `52.87 ns` (🚀 **730.71x faster**)   |
| **`deserialize_uncompressed`**           | `143.81 us` (✅ **1.00x**) | `51.70 ns` (🚀 **2781.67x faster**) | `52.69 ns` (🚀 **2729.20x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `163.93 ns` (✅ **1.00x**) | `51.82 ns` (🚀 **3.16x faster**)    | `52.70 ns` (🚀 **3.11x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.09 us` (❌ *2.56x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.88 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.04 ns` (✅ **1.00x**)       | `48.02 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `47.99 ns` (✅ **1.00x**)       | `48.04 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.43 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.66 ns` (✅ **1.00x**) | `40.58 ns` (✅ **1.13x faster**)  |
| **`into_bigint`** | `23.79 ns` (✅ **1.00x**) | `23.82 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

