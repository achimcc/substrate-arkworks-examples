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
|        | `70.30 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `449.10 ns` (✅ **1.00x**) | `9.55 ns` (🚀 **47.00x faster**)  | `9.35 ns` (🚀 **48.01x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `433.72 ns` (✅ **1.00x**) | `10.14 ns` (🚀 **42.77x faster**) | `10.03 ns` (🚀 **43.23x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `459.31 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `466.82 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                          | `332.50 ns` (✅ **1.00x**) | `6.27 ns` (🚀 **53.01x faster**)  | `6.18 ns` (🚀 **53.78x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `154.00 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `7.03 ns` (✅ **1.01x slower**)   | `6.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.60 ns` (✅ **1.00x faster**)  | `43.71 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `37.58 ns` (✅ **1.02x slower**)  | `36.89 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.41 us` (✅ **1.03x slower**)   | `7.18 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `63.56 ns` (✅ **1.04x slower**)  | `61.31 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `96.85 ns` (✅ **1.00x faster**)  | `96.99 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.76 ns` (✅ **1.00x**)        | `7.72 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.24 ns` (✅ **1.00x**)        | `9.38 ns` (✅ **1.02x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.58 ns` (✅ **1.00x**)        | `4.63 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.42 ns` (✅ **1.00x**)        | `4.28 ns` (✅ **1.03x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `76.61 ns` (✅ **1.00x**)  | `33.01 ns` (🚀 **2.32x faster**)    | `33.19 ns` (🚀 **2.31x faster**)     |
| **`serialize_uncompressed`**             | `66.97 ns` (✅ **1.00x**)  | `31.75 ns` (🚀 **2.11x faster**)    | `33.38 ns` (🚀 **2.01x faster**)     |
| **`deserialize_compressed`**             | `194.26 us` (✅ **1.00x**) | `64.66 ns` (🚀 **3004.24x faster**) | `66.89 ns` (🚀 **2904.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.42 us` (✅ **1.00x**)  | `64.63 ns` (🚀 **625.40x faster**)  | `66.79 ns` (🚀 **605.15x faster**)   |
| **`deserialize_uncompressed`**           | `152.17 us` (✅ **1.00x**) | `63.99 ns` (🚀 **2378.17x faster**) | `66.96 ns` (🚀 **2272.67x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `171.35 ns` (✅ **1.00x**) | `65.58 ns` (🚀 **2.61x faster**)    | `65.88 ns` (🚀 **2.60x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.57 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.05 us` (✅ **1.00x**) | `32.00 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `12.32 us` (✅ **1.00x**) | `11.37 us` (✅ **1.08x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.77 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.04x faster**)   |
| **`from_little-endian_bits`** | `72.61 ns` (✅ **1.00x**)       | `69.93 ns` (✅ **1.04x faster**)  |
| **`from_big-endian_bits`**    | `71.54 ns` (✅ **1.00x**)       | `71.78 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `4.83 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)        | `5.24 ns` (✅ **1.03x faster**)   |
| **`is_zero`**                 | `4.58 ns` (✅ **1.00x**)        | `4.67 ns` (✅ **1.02x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.29 ns` (✅ **1.00x**) | `41.84 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `25.03 ns` (✅ **1.00x**) | `25.66 ns` (✅ **1.03x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

