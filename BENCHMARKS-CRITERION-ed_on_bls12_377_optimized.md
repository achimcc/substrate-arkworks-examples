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
|        | `65.88 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `433.63 ns` (✅ **1.00x**) | `9.51 ns` (🚀 **45.61x faster**) | `9.54 ns` (🚀 **45.46x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `443.78 ns` (✅ **1.00x**) | `9.81 ns` (🚀 **45.21x faster**) | `9.73 ns` (🚀 **45.61x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `440.55 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `434.54 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `329.76 ns` (✅ **1.00x**) | `5.74 ns` (🚀 **57.42x faster**) | `6.06 ns` (🚀 **54.38x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `145.81 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.59 ns` (✅ **1.03x faster**)  | `6.77 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.22 ns` (✅ **1.01x faster**) | `42.68 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.76 ns` (✅ **1.02x faster**) | `36.32 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.22 us` (✅ **1.04x faster**)  | `7.51 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `57.99 ns` (✅ **1.06x faster**) | `61.25 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `94.59 ns` (✅ **1.01x faster**) | `95.20 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.21 ns` (✅ **1.00x**)        | `7.75 ns` (✅ **1.07x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.60 ns` (✅ **1.00x**)        | `9.21 ns` (✅ **1.07x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.53 ns` (✅ **1.00x**)        | `4.28 ns` (✅ **1.06x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.16 ns` (✅ **1.00x**)        | `4.27 ns` (✅ **1.03x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `73.68 ns` (✅ **1.00x**)  | `31.99 ns` (🚀 **2.30x faster**)    | `32.42 ns` (🚀 **2.27x faster**)     |
| **`serialize_uncompressed`**             | `64.48 ns` (✅ **1.00x**)  | `31.58 ns` (🚀 **2.04x faster**)    | `31.92 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `189.02 us` (✅ **1.00x**) | `56.45 ns` (🚀 **3348.55x faster**) | `51.29 ns` (🚀 **3684.94x faster**)  |
| **`deserialize_compressed_unchecked`**   | `41.36 us` (✅ **1.00x**)  | `57.34 ns` (🚀 **721.19x faster**)  | `54.26 ns` (🚀 **762.15x faster**)   |
| **`deserialize_uncompressed`**           | `148.64 us` (✅ **1.00x**) | `51.92 ns` (🚀 **2863.08x faster**) | `52.81 ns` (🚀 **2814.65x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `170.54 ns` (✅ **1.00x**) | `51.40 ns` (🚀 **3.32x faster**)    | `51.01 ns` (🚀 **3.34x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.58 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.90 us` (✅ **1.00x**) | `32.64 us` (❌ *2.74x slower*)    |
| **`legendre_for_qr`**    | `11.72 us` (✅ **1.00x**) | `11.26 us` (✅ **1.04x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.24 ns` (✅ **1.00x**)        | `4.40 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `67.37 ns` (✅ **1.00x**)       | `69.11 ns` (✅ **1.03x slower**)  |
| **`from_big-endian_bits`**    | `66.67 ns` (✅ **1.00x**)       | `68.58 ns` (✅ **1.03x slower**)  |
| **`comparison`**              | `4.42 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.10x slower**)   |
| **`equality`**                | `5.06 ns` (✅ **1.00x**)        | `5.15 ns` (✅ **1.02x slower**)   |
| **`is_zero`**                 | `4.57 ns` (✅ **1.00x**)        | `4.57 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `39.07 ns` (✅ **1.00x**) | `39.50 ns` (✅ **1.01x slower**)  |
| **`into_bigint`** | `24.20 ns` (✅ **1.00x**) | `24.18 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

