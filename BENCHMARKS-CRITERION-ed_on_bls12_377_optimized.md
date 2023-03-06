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
|        | `72.24 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `452.62 ns` (✅ **1.00x**) | `9.92 ns` (🚀 **45.61x faster**)  | `9.76 ns` (🚀 **46.37x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `475.67 ns` (✅ **1.00x**) | `10.32 ns` (🚀 **46.09x faster**) | `10.32 ns` (🚀 **46.09x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `466.10 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `473.18 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                          | `347.93 ns` (✅ **1.00x**) | `6.36 ns` (🚀 **54.73x faster**)  | `6.34 ns` (🚀 **54.90x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `157.81 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `7.14 ns` (✅ **1.00x slower**)   | `7.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `44.71 ns` (✅ **1.00x faster**)  | `44.72 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `38.17 ns` (✅ **1.01x slower**)  | `37.96 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `8.02 us` (✅ **1.07x slower**)   | `7.53 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `63.81 ns` (✅ **1.00x slower**)  | `63.51 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `98.63 ns` (✅ **1.01x faster**)  | `99.64 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `7.82 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.39 ns` (✅ **1.00x**)        | `9.40 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**)        | `4.67 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.48 ns` (✅ **1.00x**)        | `4.49 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `77.19 ns` (✅ **1.00x**)  | `33.68 ns` (🚀 **2.29x faster**)    | `35.71 ns` (🚀 **2.16x faster**)     |
| **`serialize_uncompressed`**             | `68.37 ns` (✅ **1.00x**)  | `33.54 ns` (🚀 **2.04x faster**)    | `35.57 ns` (🚀 **1.92x faster**)     |
| **`deserialize_compressed`**             | `197.07 us` (✅ **1.00x**) | `55.45 ns` (🚀 **3553.99x faster**) | `54.07 ns` (🚀 **3644.75x faster**)  |
| **`deserialize_compressed_unchecked`**   | `41.71 us` (✅ **1.00x**)  | `55.39 ns` (🚀 **752.98x faster**)  | `53.98 ns` (🚀 **772.59x faster**)   |
| **`deserialize_uncompressed`**           | `154.91 us` (✅ **1.00x**) | `54.20 ns` (🚀 **2857.96x faster**) | `54.82 ns` (🚀 **2825.80x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `174.19 ns` (✅ **1.00x**) | `54.20 ns` (🚀 **3.21x faster**)    | `54.87 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.62 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.54 us` (✅ **1.00x**) | `33.17 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `12.69 us` (✅ **1.00x**) | `11.41 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.77 ns` (✅ **1.00x**)        | `4.77 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `73.23 ns` (✅ **1.00x**)       | `73.16 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `73.06 ns` (✅ **1.00x**)       | `73.11 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.40 ns` (✅ **1.00x**)        | `5.37 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.18 ns` (✅ **1.00x**) | `43.04 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `26.03 ns` (✅ **1.00x**) | `25.99 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

