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
|        | `66.48 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `389.72 ns` (✅ **1.00x**) | `8.73 ns` (🚀 **44.66x faster**) | `8.63 ns` (🚀 **45.16x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `408.91 ns` (✅ **1.00x**) | `8.82 ns` (🚀 **46.35x faster**) | `8.79 ns` (🚀 **46.53x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `399.02 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `407.62 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.25 ns` (✅ **1.00x**) | `5.88 ns` (🚀 **50.22x faster**) | `5.87 ns` (🚀 **50.30x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.34 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.00x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.87 ns` (✅ **1.00x slower**) | `42.79 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.40 ns` (✅ **1.01x slower**) | `34.99 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.02x faster**)  | `7.02 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.54 ns` (✅ **1.00x faster**) | `61.56 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.00 ns` (✅ **1.01x faster**) | `89.83 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.60 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.75 ns` (✅ **1.00x**)        | `8.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.32 ns` (✅ **1.00x**)  | `30.96 ns` (🚀 **2.21x faster**)    | `31.04 ns` (🚀 **2.20x faster**)     |
| **`serialize_uncompressed`**             | `62.06 ns` (✅ **1.00x**)  | `30.89 ns` (🚀 **2.01x faster**)    | `30.87 ns` (🚀 **2.01x faster**)     |
| **`deserialize_compressed`**             | `181.99 us` (✅ **1.00x**) | `49.56 ns` (🚀 **3671.88x faster**) | `52.04 ns` (🚀 **3497.21x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.47 us` (✅ **1.00x**)  | `49.55 ns` (🚀 **776.30x faster**)  | `52.10 ns` (🚀 **738.35x faster**)   |
| **`deserialize_uncompressed`**           | `143.54 us` (✅ **1.00x**) | `49.56 ns` (🚀 **2896.24x faster**) | `52.03 ns` (🚀 **2758.68x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.29 ns` (✅ **1.00x**) | `49.66 ns` (🚀 **3.31x faster**)    | `51.97 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.19 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.74 ns` (✅ **1.00x**)       | `48.57 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.62 ns` (✅ **1.00x**)       | `48.65 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.42 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.92 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.87 ns` (✅ **1.00x**) | `40.76 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `22.47 ns` (✅ **1.06x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

