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
|        | `155.71 us` (✅ **1.00x**)       |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `470.18 ns` (✅ **1.00x**) | `9.90 ns` (🚀 **47.47x faster**)  | `9.79 ns` (🚀 **48.05x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `484.96 ns` (✅ **1.00x**) | `12.77 ns` (🚀 **37.98x faster**) | `13.09 ns` (🚀 **37.06x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `469.74 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `490.78 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `353.22 ns` (✅ **1.00x**) | `6.16 ns` (🚀 **57.38x faster**)  | `5.98 ns` (🚀 **59.02x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `233.68 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                     | `6.78 ns` (✅ **1.04x faster**)   | `7.02 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                     | `44.50 ns` (✅ **1.04x faster**)  | `46.28 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                     | `36.06 ns` (✅ **1.01x slower**)  | `35.71 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                     | `6.87 us` (✅ **1.02x faster**)   | `7.01 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                     | `61.17 ns` (✅ **1.03x slower**)  | `59.51 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                     | `91.92 ns` (✅ **1.01x faster**)  | `92.50 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.66 ns` (✅ **1.00x**)        | `8.30 ns` (✅ **1.08x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.71 ns` (✅ **1.00x**)        | `10.17 ns` (✅ **1.05x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.32 ns` (✅ **1.00x**)        | `4.35 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.21 ns` (✅ **1.00x**)        | `4.36 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `87.87 ns` (✅ **1.00x**)  | `32.81 ns` (🚀 **2.68x faster**)    | `31.95 ns` (🚀 **2.75x faster**)     |
| **`serialize_uncompressed`**             | `111.60 ns` (✅ **1.00x**) | `32.09 ns` (🚀 **3.48x faster**)    | `32.52 ns` (🚀 **3.43x faster**)     |
| **`deserialize_compressed`**             | `235.93 us` (✅ **1.00x**) | `66.91 ns` (🚀 **3526.06x faster**) | `63.50 ns` (🚀 **3715.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.27 us` (✅ **1.00x**)  | `65.04 ns` (🚀 **603.83x faster**)  | `63.33 ns` (🚀 **620.14x faster**)   |
| **`deserialize_uncompressed`**           | `194.26 us` (✅ **1.00x**) | `66.15 ns` (🚀 **2936.79x faster**) | `66.91 ns` (🚀 **2903.41x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `184.96 ns` (✅ **1.00x**) | `66.97 ns` (🚀 **2.76x faster**)    | `65.79 ns` (🚀 **2.81x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.76 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.40 us` (✅ **1.00x**) | `31.93 us` (❌ *2.58x slower*)    |
| **`legendre_for_qr`**    | `12.76 us` (✅ **1.00x**) | `11.96 us` (✅ **1.07x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.52 ns` (✅ **1.00x**)        | `4.44 ns` (✅ **1.02x faster**)    |
| **`from_little-endian_bits`** | `179.75 ns` (✅ **1.00x**)      | `170.95 ns` (✅ **1.05x faster**)  |
| **`from_big-endian_bits`**    | `172.91 ns` (✅ **1.00x**)      | `174.26 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.40 ns` (✅ **1.00x**)        | `4.60 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.78 ns` (✅ **1.00x**)        | `4.80 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `4.31 ns` (✅ **1.00x**)        | `4.28 ns` (✅ **1.01x faster**)    |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.21 ns` (✅ **1.00x**) | `42.05 ns` (✅ **1.03x faster**)  |
| **`into_bigint`** | `29.23 ns` (✅ **1.00x**) | `28.71 ns` (✅ **1.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

