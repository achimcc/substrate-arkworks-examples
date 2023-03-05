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
|        | `59.19 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `377.53 ns` (✅ **1.00x**) | `8.30 ns` (🚀 **45.47x faster**) | `8.13 ns` (🚀 **46.42x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `392.80 ns` (✅ **1.00x**) | `8.62 ns` (🚀 **45.59x faster**) | `8.63 ns` (🚀 **45.54x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `387.43 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `393.74 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `290.96 ns` (✅ **1.00x**) | `9.09 ns` (🚀 **32.02x faster**) | `5.33 ns` (🚀 **54.62x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.56 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.96 ns` (✅ **1.00x slower**)  | `5.94 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.28 ns` (✅ **1.00x slower**) | `37.26 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.79 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.23 us` (✅ **1.02x faster**)  | `6.34 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.02 ns` (✅ **1.00x slower**) | `52.84 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.95 ns` (✅ **1.01x faster**) | `80.66 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.56 ns` (✅ **1.00x**)        | `7.84 ns` (✅ **1.04x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.10 ns` (✅ **1.00x**)  | `27.95 ns` (🚀 **2.29x faster**)    | `28.07 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `54.18 ns` (✅ **1.00x**)  | `27.82 ns` (🚀 **1.95x faster**)    | `28.05 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `163.70 us` (✅ **1.00x**) | `46.60 ns` (🚀 **3512.88x faster**) | `46.38 ns` (🚀 **3529.31x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.27 us` (✅ **1.00x**)  | `46.60 ns` (🚀 **735.34x faster**)  | `46.37 ns` (🚀 **738.90x faster**)   |
| **`deserialize_uncompressed`**           | `128.88 us` (✅ **1.00x**) | `46.54 ns` (🚀 **2769.54x faster**) | `46.33 ns` (🚀 **2781.78x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.17 ns` (✅ **1.00x**) | `46.20 ns` (🚀 **3.10x faster**)    | `46.33 ns` (🚀 **3.09x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.37 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.38 us` (✅ **1.00x**) | `27.57 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `10.62 us` (✅ **1.00x**) | `9.52 us` (✅ **1.12x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.74 ns` (✅ **1.00x**)       | `60.62 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.65 ns` (✅ **1.00x**)       | `60.65 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.06 ns` (✅ **1.00x**)        | `4.07 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.90 ns` (✅ **1.00x**) | `35.97 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.77 ns` (✅ **1.00x**) | `21.77 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

