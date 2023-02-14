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
|        | `59.16 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.57 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.46x faster**) | `8.13 ns` (🚀 **46.31x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `396.92 ns` (✅ **1.00x**) | `8.59 ns` (🚀 **46.20x faster**) | `8.69 ns` (🚀 **45.69x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `387.69 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `392.57 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `291.83 ns` (✅ **1.00x**) | `9.02 ns` (🚀 **32.34x faster**) | `5.29 ns` (🚀 **55.15x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `131.13 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.31 ns` (✅ **1.00x slower**) | `37.26 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.75 ns` (✅ **1.01x slower**) | `31.57 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.19 us` (✅ **1.03x faster**)  | `6.34 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.05 ns` (✅ **1.00x faster**) | `53.22 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.78 ns` (✅ **1.01x faster**) | `80.66 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.85 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `63.71 ns` (✅ **1.00x**)  | `27.95 ns` (🚀 **2.28x faster**)    | `27.85 ns` (🚀 **2.29x faster**)     |
| **`serialize_uncompressed`**             | `54.36 ns` (✅ **1.00x**)  | `27.86 ns` (🚀 **1.95x faster**)    | `27.85 ns` (🚀 **1.95x faster**)     |
| **`deserialize_compressed`**             | `163.89 us` (✅ **1.00x**) | `46.64 ns` (🚀 **3513.85x faster**) | `45.91 ns` (🚀 **3569.51x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.23 us` (✅ **1.00x**)  | `46.63 ns` (🚀 **733.90x faster**)  | `45.91 ns` (🚀 **745.47x faster**)   |
| **`deserialize_uncompressed`**           | `129.06 us` (✅ **1.00x**) | `45.83 ns` (🚀 **2815.94x faster**) | `46.22 ns` (🚀 **2792.28x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.75 ns` (✅ **1.00x**) | `45.83 ns` (🚀 **3.14x faster**)    | `45.93 ns` (🚀 **3.13x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.39 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.39 us` (✅ **1.00x**) | `27.59 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.58 us` (✅ **1.00x**) | `9.53 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `61.07 ns` (✅ **1.00x**)       | `61.03 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `61.04 ns` (✅ **1.00x**)       | `61.01 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)        | `4.08 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.48 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `3.91 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.92 ns` (✅ **1.00x**) | `35.92 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.78 ns` (✅ **1.00x**) | `21.65 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

