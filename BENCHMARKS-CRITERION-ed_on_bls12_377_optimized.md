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
|        | `59.07 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.69 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.47x faster**) | `8.14 ns` (🚀 **46.26x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `398.07 ns` (✅ **1.00x**) | `8.78 ns` (🚀 **45.32x faster**) | `8.79 ns` (🚀 **45.28x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `389.10 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `392.22 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `292.75 ns` (✅ **1.00x**) | `9.09 ns` (🚀 **32.21x faster**) | `5.29 ns` (🚀 **55.31x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `131.10 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.96 ns` (✅ **1.00x faster**)  | `5.96 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.30 ns` (✅ **1.00x slower**) | `37.27 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.78 ns` (✅ **1.00x slower**) | `31.63 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.19 us` (✅ **1.02x faster**)  | `6.34 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.06 ns` (✅ **1.00x faster**) | `53.19 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.75 ns` (✅ **1.01x faster**) | `80.72 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**)        | `7.85 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.00 ns` (✅ **1.00x**)  | `28.07 ns` (🚀 **2.28x faster**)    | `28.08 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `54.27 ns` (✅ **1.00x**)  | `27.96 ns` (🚀 **1.94x faster**)    | `28.04 ns` (🚀 **1.94x faster**)     |
| **`deserialize_compressed`**             | `164.14 us` (✅ **1.00x**) | `46.58 ns` (🚀 **3523.68x faster**) | `44.69 ns` (🚀 **3672.68x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.21 us` (✅ **1.00x**)  | `46.57 ns` (🚀 **734.57x faster**)  | `44.46 ns` (🚀 **769.41x faster**)   |
| **`deserialize_uncompressed`**           | `129.83 us` (✅ **1.00x**) | `46.52 ns` (🚀 **2790.65x faster**) | `45.53 ns` (🚀 **2851.60x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `143.63 ns` (✅ **1.00x**) | `46.53 ns` (🚀 **3.09x faster**)    | `45.56 ns` (🚀 **3.15x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.36 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.40 us` (✅ **1.00x**) | `27.60 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.52 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `62.15 ns` (✅ **1.00x**)       | `62.16 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `62.16 ns` (✅ **1.00x**)       | `62.14 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.07 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.50 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `3.91 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.05 ns` (✅ **1.00x**) | `35.90 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.73 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

