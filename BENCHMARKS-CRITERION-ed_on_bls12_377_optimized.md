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
|        | `66.49 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `389.92 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.70x faster**) | `8.64 ns` (🚀 **45.13x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `409.15 ns` (✅ **1.00x**) | `8.83 ns` (🚀 **46.36x faster**) | `8.80 ns` (🚀 **46.50x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `400.88 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `407.44 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.54 ns` (✅ **1.00x**) | `5.87 ns` (🚀 **50.31x faster**) | `5.86 ns` (🚀 **50.45x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `147.72 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.97 ns` (✅ **1.00x slower**) | `42.80 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.38 ns` (✅ **1.01x slower**) | `35.03 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.93 us` (✅ **1.01x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.57 ns` (✅ **1.00x faster**) | `61.59 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.00 ns` (✅ **1.01x faster**) | `89.85 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.75 ns` (✅ **1.00x**)        | `8.75 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.13 ns` (✅ **1.00x**)  | `30.97 ns` (🚀 **2.20x faster**)    | `30.97 ns` (🚀 **2.20x faster**)     |
| **`serialize_uncompressed`**             | `62.00 ns` (✅ **1.00x**)  | `30.32 ns` (🚀 **2.04x faster**)    | `30.85 ns` (🚀 **2.01x faster**)     |
| **`deserialize_compressed`**             | `181.87 us` (✅ **1.00x**) | `49.73 ns` (🚀 **3657.12x faster**) | `52.22 ns` (🚀 **3482.93x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.47 us` (✅ **1.00x**)  | `49.83 ns` (🚀 **771.94x faster**)  | `52.17 ns` (🚀 **737.32x faster**)   |
| **`deserialize_uncompressed`**           | `143.46 us` (✅ **1.00x**) | `49.75 ns` (🚀 **2883.51x faster**) | `52.14 ns` (🚀 **2751.37x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.44 ns` (✅ **1.00x**) | `49.69 ns` (🚀 **3.31x faster**)    | `52.13 ns` (🚀 **3.15x faster**)     |

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
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.82 ns` (✅ **1.00x**)       | `49.18 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `48.80 ns` (✅ **1.00x**)       | `49.19 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.41 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.08 ns` (✅ **1.00x**) | `40.76 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `22.48 ns` (✅ **1.06x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

