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
|        | `66.53 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `388.54 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.57x faster**) | `8.64 ns` (🚀 **44.99x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `407.73 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.27x faster**) | `8.80 ns` (🚀 **46.33x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `399.98 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `409.29 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.04 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.32x faster**) | `5.87 ns` (🚀 **50.30x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.19 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.85 ns` (✅ **1.00x slower**) | `42.80 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.40 ns` (✅ **1.01x slower**) | `34.99 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.89 us` (✅ **1.02x faster**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.57 ns` (✅ **1.00x faster**) | `61.63 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.02 ns` (✅ **1.01x faster**) | `89.86 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.64 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.74 ns` (✅ **1.00x**)        | `8.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.24 ns` (✅ **1.00x**)  | `33.06 ns` (🚀 **2.06x faster**)    | `30.76 ns` (🚀 **2.22x faster**)     |
| **`serialize_uncompressed`**             | `62.00 ns` (✅ **1.00x**)  | `33.13 ns` (🚀 **1.87x faster**)    | `30.95 ns` (🚀 **2.00x faster**)     |
| **`deserialize_compressed`**             | `181.77 us` (✅ **1.00x**) | `49.65 ns` (🚀 **3660.88x faster**) | `52.03 ns` (🚀 **3493.54x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.46 us` (✅ **1.00x**)  | `49.65 ns` (🚀 **774.61x faster**)  | `51.96 ns` (🚀 **740.06x faster**)   |
| **`deserialize_uncompressed`**           | `144.23 us` (✅ **1.00x**) | `49.58 ns` (🚀 **2909.24x faster**) | `51.95 ns` (🚀 **2776.39x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `165.08 ns` (✅ **1.00x**) | `49.53 ns` (🚀 **3.33x faster**)    | `51.95 ns` (🚀 **3.18x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.20 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.77 ns` (✅ **1.00x**)       | `48.63 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.63 ns` (✅ **1.00x**)       | `48.59 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.42 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `40.79 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `22.49 ns` (✅ **1.06x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

