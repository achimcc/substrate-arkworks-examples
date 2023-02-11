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
|        | `60.32 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.57 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.47x faster**) | `8.14 ns` (🚀 **46.24x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `395.75 ns` (✅ **1.00x**) | `8.62 ns` (🚀 **45.89x faster**) | `8.64 ns` (🚀 **45.82x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `389.02 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `394.12 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `290.75 ns` (✅ **1.00x**) | `9.11 ns` (🚀 **31.90x faster**) | `5.29 ns` (🚀 **54.96x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.92 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.96 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.32 ns` (✅ **1.01x slower**) | `37.05 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.78 ns` (✅ **1.01x slower**) | `31.57 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.62 us` (✅ **1.04x slower**)  | `6.37 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.09 ns` (✅ **1.00x faster**) | `53.24 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.60 ns` (✅ **1.01x faster**) | `80.60 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**)        | `7.85 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**)        | `3.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.11 ns` (✅ **1.00x**)  | `27.89 ns` (🚀 **2.30x faster**)    | `28.08 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `55.31 ns` (✅ **1.00x**)  | `27.83 ns` (🚀 **1.99x faster**)    | `28.07 ns` (🚀 **1.97x faster**)     |
| **`deserialize_compressed`**             | `163.70 us` (✅ **1.00x**) | `46.60 ns` (🚀 **3512.52x faster**) | `46.45 ns` (🚀 **3523.98x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.75 us` (✅ **1.00x**)  | `46.60 ns` (🚀 **745.68x faster**)  | `46.49 ns` (🚀 **747.54x faster**)   |
| **`deserialize_uncompressed`**           | `128.76 us` (✅ **1.00x**) | `46.61 ns` (🚀 **2762.11x faster**) | `46.42 ns` (🚀 **2773.91x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.84 ns` (✅ **1.00x**) | `46.58 ns` (🚀 **3.09x faster**)    | `46.13 ns` (🚀 **3.12x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.37 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.44 us` (✅ **1.00x**) | `27.64 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.57 us` (✅ **1.00x**) | `9.56 us` (✅ **1.10x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.95 ns` (✅ **1.00x**)       | `60.97 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.93 ns` (✅ **1.00x**)       | `60.90 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.10 ns` (✅ **1.01x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.04 ns` (✅ **1.00x**) | `35.93 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.73 ns` (✅ **1.00x**) | `21.66 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

