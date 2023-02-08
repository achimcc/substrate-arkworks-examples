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
|        | `73.51 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `462.28 ns` (✅ **1.00x**) | `10.74 ns` (🚀 **43.06x faster**) | `10.45 ns` (🚀 **44.24x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `502.51 ns` (✅ **1.00x**) | `11.14 ns` (🚀 **45.11x faster**) | `11.28 ns` (🚀 **44.54x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `494.29 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `480.32 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `395.02 ns` (✅ **1.00x**) | `6.50 ns` (🚀 **60.77x faster**)  | `6.24 ns` (🚀 **63.29x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `170.45 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                     | `8.11 ns` (✅ **1.03x slower**)   | `7.84 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                     | `45.85 ns` (✅ **1.00x faster**)  | `45.93 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                     | `38.43 ns` (✅ **1.05x faster**)  | `40.17 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                     | `6.93 us` (✅ **1.02x slower**)   | `6.83 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                     | `62.77 ns` (✅ **1.16x faster**)  | `72.53 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                     | `99.51 ns` (✅ **1.05x slower**)  | `94.63 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.09 ns` (✅ **1.00x**)        | `7.68 ns` (✅ **1.05x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.35 ns` (✅ **1.00x**)       | `10.69 ns` (✅ **1.03x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.50 ns` (✅ **1.00x**)        | `4.61 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.37 ns` (✅ **1.00x**)        | `4.17 ns` (✅ **1.05x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `78.15 ns` (✅ **1.00x**)  | `37.32 ns` (🚀 **2.09x faster**)    | `37.88 ns` (🚀 **2.06x faster**)     |
| **`serialize_uncompressed`**             | `67.47 ns` (✅ **1.00x**)  | `37.26 ns` (🚀 **1.81x faster**)    | `36.78 ns` (🚀 **1.83x faster**)     |
| **`deserialize_compressed`**             | `214.73 us` (✅ **1.00x**) | `58.08 ns` (🚀 **3697.29x faster**) | `57.08 ns` (🚀 **3761.91x faster**)  |
| **`deserialize_compressed_unchecked`**   | `43.26 us` (✅ **1.00x**)  | `58.72 ns` (🚀 **736.70x faster**)  | `58.13 ns` (🚀 **744.14x faster**)   |
| **`deserialize_uncompressed`**           | `166.99 us` (✅ **1.00x**) | `58.83 ns` (🚀 **2838.65x faster**) | `59.50 ns` (🚀 **2806.38x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `178.35 ns` (✅ **1.00x**) | `58.25 ns` (🚀 **3.06x faster**)    | `60.98 ns` (🚀 **2.92x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.69 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.07 us` (✅ **1.00x**) | `35.09 us` (❌ *2.68x slower*)    |
| **`legendre_for_qr`**    | `13.51 us` (✅ **1.00x**) | `12.35 us` (✅ **1.09x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.49 ns` (✅ **1.00x**)        | `4.48 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `73.19 ns` (✅ **1.00x**)       | `72.31 ns` (✅ **1.01x faster**)  |
| **`from_big-endian_bits`**    | `74.23 ns` (✅ **1.00x**)       | `72.27 ns` (✅ **1.03x faster**)  |
| **`comparison`**              | `4.69 ns` (✅ **1.00x**)        | `4.49 ns` (✅ **1.04x faster**)   |
| **`equality`**                | `5.06 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.04x faster**)   |
| **`is_zero`**                 | `4.30 ns` (✅ **1.00x**)        | `4.24 ns` (✅ **1.01x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.18 ns` (✅ **1.00x**) | `43.43 ns` (✅ **1.01x slower**)  |
| **`into_bigint`** | `27.13 ns` (✅ **1.00x**) | `26.88 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

