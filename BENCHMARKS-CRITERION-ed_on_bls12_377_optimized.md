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
|        | `70.72 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `446.70 ns` (✅ **1.00x**) | `9.85 ns` (🚀 **45.34x faster**)  | `9.65 ns` (🚀 **46.30x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `467.14 ns` (✅ **1.00x**) | `10.20 ns` (🚀 **45.79x faster**) | `10.73 ns` (🚀 **43.54x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `463.74 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `471.89 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                          | `344.36 ns` (✅ **1.00x**) | `6.31 ns` (🚀 **54.59x faster**)  | `6.56 ns` (🚀 **52.53x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `156.49 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `7.06 ns` (✅ **1.01x faster**)   | `7.11 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `44.76 ns` (✅ **1.02x slower**)  | `44.03 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `37.84 ns` (✅ **1.02x faster**)  | `38.63 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.35 us` (✅ **1.05x faster**)   | `7.75 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `62.81 ns` (✅ **1.00x faster**)  | `63.06 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `94.10 ns` (✅ **1.03x faster**)  | `96.64 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.75 ns` (✅ **1.00x**)        | `7.81 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.29 ns` (✅ **1.00x**)        | `9.38 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.60 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.44 ns` (✅ **1.00x**)        | `4.47 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `82.30 ns` (✅ **1.00x**)  | `33.49 ns` (🚀 **2.46x faster**)    | `32.94 ns` (🚀 **2.50x faster**)     |
| **`serialize_uncompressed`**             | `64.93 ns` (✅ **1.00x**)  | `34.44 ns` (🚀 **1.89x faster**)    | `33.29 ns` (🚀 **1.95x faster**)     |
| **`deserialize_compressed`**             | `195.47 us` (✅ **1.00x**) | `55.11 ns` (🚀 **3546.66x faster**) | `56.97 ns` (🚀 **3431.17x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.72 us` (✅ **1.00x**)  | `55.39 ns` (🚀 **735.18x faster**)  | `53.41 ns` (🚀 **762.44x faster**)   |
| **`deserialize_uncompressed`**           | `156.63 us` (✅ **1.00x**) | `55.36 ns` (🚀 **2829.49x faster**) | `53.55 ns` (🚀 **2925.06x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `170.88 ns` (✅ **1.00x**) | `55.07 ns` (🚀 **3.10x faster**)    | `53.58 ns` (🚀 **3.19x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.62 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.28 us` (✅ **1.00x**) | `34.81 us` (❌ *2.83x slower*)    |
| **`legendre_for_qr`**    | `12.52 us` (✅ **1.00x**) | `12.00 us` (✅ **1.04x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.74 ns` (✅ **1.00x**)        | `4.74 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `74.22 ns` (✅ **1.00x**)       | `72.34 ns` (✅ **1.03x faster**)  |
| **`from_big-endian_bits`**    | `71.56 ns` (✅ **1.00x**)       | `72.47 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.82 ns` (✅ **1.00x**)        | `4.83 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.59 ns` (✅ **1.00x**)        | `5.34 ns` (✅ **1.05x faster**)   |
| **`is_zero`**                 | `4.84 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.76 ns` (✅ **1.00x**) | `43.10 ns` (✅ **1.01x slower**)  |
| **`into_bigint`** | `25.87 ns` (✅ **1.00x**) | `25.89 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

