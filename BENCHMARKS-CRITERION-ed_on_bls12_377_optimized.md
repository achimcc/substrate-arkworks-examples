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
|        | `59.25 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.48 ns` (✅ **1.00x**) | `8.30 ns` (🚀 **45.36x faster**) | `8.14 ns` (🚀 **46.24x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `392.92 ns` (✅ **1.00x**) | `8.61 ns` (🚀 **45.64x faster**) | `8.61 ns` (🚀 **45.63x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `387.44 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `393.43 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `291.59 ns` (✅ **1.00x**) | `5.31 ns` (🚀 **54.86x faster**) | `5.29 ns` (🚀 **55.09x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `130.91 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.30 ns` (✅ **1.00x slower**) | `37.24 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.80 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.23 us` (✅ **1.01x faster**)  | `6.26 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.00 ns` (✅ **1.00x faster**) | `53.20 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `80.07 ns` (✅ **1.01x faster**) | `81.10 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.86 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.84 ns` (✅ **1.00x**)  | `27.91 ns` (🚀 **2.47x faster**)    | `28.24 ns` (🚀 **2.44x faster**)     |
| **`serialize_uncompressed`**             | `54.07 ns` (✅ **1.00x**)  | `27.82 ns` (🚀 **1.94x faster**)    | `28.19 ns` (🚀 **1.92x faster**)     |
| **`deserialize_compressed`**             | `163.46 us` (✅ **1.00x**) | `45.64 ns` (🚀 **3581.13x faster**) | `44.79 ns` (🚀 **3649.44x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.25 us` (✅ **1.00x**)  | `45.67 ns` (🚀 **749.93x faster**)  | `44.79 ns` (🚀 **764.71x faster**)   |
| **`deserialize_uncompressed`**           | `128.97 us` (✅ **1.00x**) | `44.87 ns` (🚀 **2874.34x faster**) | `45.27 ns` (🚀 **2848.96x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.52 ns` (✅ **1.00x**) | `44.89 ns` (🚀 **3.20x faster**)    | `45.31 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.36 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.39 us` (✅ **1.00x**) | `27.62 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.55 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.83 ns` (✅ **1.00x**)       | `60.78 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.86 ns` (✅ **1.00x**)       | `60.70 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.05 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.51 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.99 ns` (✅ **1.00x**) | `35.88 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `21.75 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

