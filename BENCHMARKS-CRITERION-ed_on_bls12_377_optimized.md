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
|        | `66.58 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `387.84 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.52x faster**) | `8.63 ns` (🚀 **44.95x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `407.84 ns` (✅ **1.00x**) | `8.82 ns` (🚀 **46.25x faster**) | `8.79 ns` (🚀 **46.38x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `399.76 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `408.09 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `294.57 ns` (✅ **1.00x**) | `5.88 ns` (🚀 **50.11x faster**) | `5.86 ns` (🚀 **50.28x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.17 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.89 ns` (✅ **1.00x slower**) | `42.78 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.38 ns` (✅ **1.01x slower**) | `35.05 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.88 us` (✅ **1.02x faster**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.55 ns` (✅ **1.00x faster**) | `61.55 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.06 ns` (✅ **1.01x faster**) | `89.84 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.74 ns` (✅ **1.00x**)        | `8.74 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.34 ns` (✅ **1.00x**)  | `30.99 ns` (🚀 **2.20x faster**)    | `31.29 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `62.11 ns` (✅ **1.00x**)  | `30.38 ns` (🚀 **2.04x faster**)    | `30.82 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `181.75 us` (✅ **1.00x**) | `49.74 ns` (🚀 **3653.78x faster**) | `52.71 ns` (🚀 **3448.46x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.48 us` (✅ **1.00x**)  | `49.71 ns` (🚀 **774.11x faster**)  | `52.26 ns` (🚀 **736.30x faster**)   |
| **`deserialize_uncompressed`**           | `143.24 us` (✅ **1.00x**) | `49.75 ns` (🚀 **2879.45x faster**) | `52.18 ns` (🚀 **2745.10x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.51 ns` (✅ **1.00x**) | `49.71 ns` (🚀 **3.31x faster**)    | `52.17 ns` (🚀 **3.15x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.20 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.66 ns` (✅ **1.00x**)       | `48.69 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.61 ns` (✅ **1.00x**)       | `48.61 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.42 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.89 ns` (✅ **1.00x**) | `40.77 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `22.49 ns` (✅ **1.06x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

