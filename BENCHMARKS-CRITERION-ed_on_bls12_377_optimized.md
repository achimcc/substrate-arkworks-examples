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
|        | `66.78 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `388.40 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.56x faster**) | `8.63 ns` (🚀 **44.98x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `407.83 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.29x faster**) | `8.80 ns` (🚀 **46.32x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `398.81 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `407.77 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.09 ns` (✅ **1.00x**) | `5.90 ns` (🚀 **50.02x faster**) | `5.84 ns` (🚀 **50.57x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.41 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.95 ns` (✅ **1.00x slower**) | `42.82 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.44 ns` (✅ **1.01x slower**) | `35.08 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.89 us` (✅ **1.02x faster**)  | `7.03 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.56 ns` (✅ **1.00x faster**) | `61.63 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.03 ns` (✅ **1.01x faster**) | `89.85 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.74 ns` (✅ **1.00x**)        | `8.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.39 ns` (✅ **1.00x**)  | `30.89 ns` (🚀 **2.21x faster**)    | `30.63 ns` (🚀 **2.23x faster**)     |
| **`serialize_uncompressed`**             | `62.00 ns` (✅ **1.00x**)  | `30.39 ns` (🚀 **2.04x faster**)    | `30.81 ns` (🚀 **2.01x faster**)     |
| **`deserialize_compressed`**             | `182.33 us` (✅ **1.00x**) | `49.63 ns` (🚀 **3673.67x faster**) | `52.11 ns` (🚀 **3498.74x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.47 us` (✅ **1.00x**)  | `49.66 ns` (🚀 **774.80x faster**)  | `52.15 ns` (🚀 **737.68x faster**)   |
| **`deserialize_uncompressed`**           | `143.24 us` (✅ **1.00x**) | `49.62 ns` (🚀 **2886.95x faster**) | `51.96 ns` (🚀 **2756.82x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.88 ns` (✅ **1.00x**) | `49.65 ns` (🚀 **3.32x faster**)    | `52.07 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.20 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.97 ns` (✅ **1.00x**)       | `48.90 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.96 ns` (✅ **1.00x**)       | `48.83 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.43 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.91 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `40.76 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `22.47 ns` (✅ **1.06x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

