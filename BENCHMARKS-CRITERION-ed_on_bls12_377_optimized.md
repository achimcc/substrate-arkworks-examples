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
|        | `59.13 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.87 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.43x faster**) | `8.14 ns` (🚀 **46.31x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `396.70 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **45.02x faster**) | `8.66 ns` (🚀 **45.79x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `387.72 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `392.37 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `291.72 ns` (✅ **1.00x**) | `9.09 ns` (🚀 **32.08x faster**) | `5.29 ns` (🚀 **55.18x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `131.05 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.26 ns` (✅ **1.00x faster**) | `37.26 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.74 ns` (✅ **1.01x slower**) | `31.57 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.19 us` (✅ **1.02x faster**)  | `6.34 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.04 ns` (✅ **1.00x faster**) | `53.22 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.72 ns` (✅ **1.01x faster**) | `80.70 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `63.91 ns` (✅ **1.00x**)  | `27.95 ns` (🚀 **2.29x faster**)    | `27.97 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `54.27 ns` (✅ **1.00x**)  | `27.83 ns` (🚀 **1.95x faster**)    | `27.94 ns` (🚀 **1.94x faster**)     |
| **`deserialize_compressed`**             | `163.40 us` (✅ **1.00x**) | `46.65 ns` (🚀 **3502.88x faster**) | `46.21 ns` (🚀 **3536.15x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.19 us` (✅ **1.00x**)  | `46.65 ns` (🚀 **733.00x faster**)  | `46.21 ns` (🚀 **739.99x faster**)   |
| **`deserialize_uncompressed`**           | `129.08 us` (✅ **1.00x**) | `46.60 ns` (🚀 **2769.90x faster**) | `44.77 ns` (🚀 **2883.04x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.40 ns` (✅ **1.00x**) | `46.13 ns` (🚀 **3.11x faster**)    | `44.65 ns` (🚀 **3.21x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.37 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.46 us` (✅ **1.00x**) | `27.60 us` (❌ *2.64x slower*)    |
| **`legendre_for_qr`**    | `10.57 us` (✅ **1.00x**) | `9.53 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.91 ns` (✅ **1.00x**)       | `60.88 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.97 ns` (✅ **1.00x**)       | `60.98 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.91 ns` (✅ **1.00x**) | `35.92 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.72 ns` (✅ **1.00x**) | `21.64 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

