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
|        | `59.27 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `377.82 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.56x faster**) | `8.14 ns` (🚀 **46.41x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `392.75 ns` (✅ **1.00x**) | `8.59 ns` (🚀 **45.73x faster**) | `8.60 ns` (🚀 **45.66x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `389.70 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `394.26 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `288.45 ns` (✅ **1.00x**) | `5.28 ns` (🚀 **54.66x faster**) | `5.28 ns` (🚀 **54.63x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.75 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.96 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.31 ns` (✅ **1.00x slower**) | `37.27 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.79 ns` (✅ **1.01x slower**) | `31.62 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.18 us` (✅ **1.01x faster**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.02 ns` (✅ **1.01x slower**) | `52.70 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `81.83 ns` (✅ **1.02x faster**) | `83.10 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.12 ns` (✅ **1.00x**)  | `28.07 ns` (🚀 **2.28x faster**)    | `27.92 ns` (🚀 **2.30x faster**)     |
| **`serialize_uncompressed`**             | `56.75 ns` (✅ **1.00x**)  | `28.05 ns` (🚀 **2.02x faster**)    | `27.86 ns` (🚀 **2.04x faster**)     |
| **`deserialize_compressed`**             | `163.59 us` (✅ **1.00x**) | `46.63 ns` (🚀 **3508.05x faster**) | `46.62 ns` (🚀 **3508.88x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.26 us` (✅ **1.00x**)  | `46.56 ns` (🚀 **735.81x faster**)  | `46.62 ns` (🚀 **734.90x faster**)   |
| **`deserialize_uncompressed`**           | `129.16 us` (✅ **1.00x**) | `46.60 ns` (🚀 **2771.86x faster**) | `46.67 ns` (🚀 **2767.77x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `146.40 ns` (✅ **1.00x**) | `46.53 ns` (🚀 **3.15x faster**)    | `46.63 ns` (🚀 **3.14x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.40 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.40 us` (✅ **1.00x**) | `27.64 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `10.57 us` (✅ **1.00x**) | `9.52 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.74 ns` (✅ **1.00x**)       | `60.71 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.88 ns` (✅ **1.00x**)       | `60.76 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.48 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.93 ns` (✅ **1.00x**) | `35.90 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.78 ns` (✅ **1.00x**) | `21.75 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

