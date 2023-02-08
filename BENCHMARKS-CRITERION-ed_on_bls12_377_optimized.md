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
|        | `59.37 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `378.73 ns` (✅ **1.00x**) | `8.07 ns` (🚀 **46.94x faster**) | `8.15 ns` (🚀 **46.50x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `395.31 ns` (✅ **1.00x**) | `8.60 ns` (🚀 **45.97x faster**) | `8.64 ns` (🚀 **45.73x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `386.23 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `392.80 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `290.37 ns` (✅ **1.00x**) | `9.08 ns` (🚀 **31.99x faster**) | `5.30 ns` (🚀 **54.80x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `131.31 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.31 ns` (✅ **1.00x slower**) | `37.26 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.77 ns` (✅ **1.02x faster**) | `32.39 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.23 us` (✅ **1.00x faster**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.01 ns` (✅ **1.00x faster**) | `53.20 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.83 ns` (✅ **1.01x faster**) | `80.65 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `7.83 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**)        | `3.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.96 ns` (✅ **1.00x**)  | `28.04 ns` (🚀 **2.42x faster**)    | `28.03 ns` (🚀 **2.42x faster**)     |
| **`serialize_uncompressed`**             | `55.31 ns` (✅ **1.00x**)  | `27.93 ns` (🚀 **1.98x faster**)    | `27.78 ns` (🚀 **1.99x faster**)     |
| **`deserialize_compressed`**             | `163.15 us` (✅ **1.00x**) | `46.86 ns` (🚀 **3481.50x faster**) | `44.83 ns` (🚀 **3639.50x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.22 us` (✅ **1.00x**)  | `46.86 ns` (🚀 **730.17x faster**)  | `44.82 ns` (🚀 **763.45x faster**)   |
| **`deserialize_uncompressed`**           | `129.23 us` (✅ **1.00x**) | `46.82 ns` (🚀 **2759.85x faster**) | `46.22 ns` (🚀 **2795.93x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.15 ns` (✅ **1.00x**) | `46.54 ns` (🚀 **3.08x faster**)    | `46.23 ns` (🚀 **3.10x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.40 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.46 us` (✅ **1.00x**) | `27.61 us` (❌ *2.64x slower*)    |
| **`legendre_for_qr`**    | `10.62 us` (✅ **1.00x**) | `9.58 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.70 ns` (✅ **1.00x**)       | `60.62 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.75 ns` (✅ **1.00x**)       | `60.69 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.51 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.04 ns` (✅ **1.00x**) | `35.91 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.67 ns` (✅ **1.00x**) | `21.77 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

