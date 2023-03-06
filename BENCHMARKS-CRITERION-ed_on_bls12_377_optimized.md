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
|        | `59.18 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.60 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.46x faster**) | `8.14 ns` (🚀 **46.28x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `392.74 ns` (✅ **1.00x**) | `8.62 ns` (🚀 **45.59x faster**) | `8.58 ns` (🚀 **45.75x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `389.39 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `393.89 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `289.08 ns` (✅ **1.00x**) | `5.29 ns` (🚀 **54.65x faster**) | `5.29 ns` (🚀 **54.64x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.75 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.94 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.22 ns` (✅ **1.00x faster**) | `37.22 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.76 ns` (✅ **1.01x slower**) | `31.56 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.19 us` (✅ **1.01x faster**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.05 ns` (✅ **1.01x slower**) | `52.65 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `81.99 ns` (✅ **1.01x faster**) | `82.97 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.19 ns` (✅ **1.00x**)  | `27.82 ns` (🚀 **2.31x faster**)    | `27.88 ns` (🚀 **2.30x faster**)     |
| **`serialize_uncompressed`**             | `56.69 ns` (✅ **1.00x**)  | `27.81 ns` (🚀 **2.04x faster**)    | `27.81 ns` (🚀 **2.04x faster**)     |
| **`deserialize_compressed`**             | `163.45 us` (✅ **1.00x**) | `46.58 ns` (🚀 **3508.90x faster**) | `45.20 ns` (🚀 **3616.55x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.22 us` (✅ **1.00x**)  | `46.57 ns` (🚀 **734.87x faster**)  | `45.18 ns` (🚀 **757.44x faster**)   |
| **`deserialize_uncompressed`**           | `128.97 us` (✅ **1.00x**) | `44.86 ns` (🚀 **2874.63x faster**) | `46.44 ns` (🚀 **2777.01x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.89 ns` (✅ **1.00x**) | `44.88 ns` (🚀 **3.25x faster**)    | `46.45 ns` (🚀 **3.14x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.40 us` (✅ **1.00x**) | `27.61 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `10.56 us` (✅ **1.00x**) | `9.52 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.98 ns` (✅ **1.00x**)       | `60.83 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.64 ns` (✅ **1.00x**)       | `60.82 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.06 ns` (✅ **1.00x**)        | `4.08 ns` (✅ **1.01x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.50 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.99 ns` (✅ **1.00x**) | `35.89 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.64 ns` (✅ **1.00x**) | `21.73 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

