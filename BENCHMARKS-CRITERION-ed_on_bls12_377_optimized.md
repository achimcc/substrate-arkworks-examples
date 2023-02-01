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
|        | `144.99 us` (✅ **1.00x**)       |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `385.44 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.27x faster**) | `8.68 ns` (🚀 **44.41x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `405.61 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.02x faster**) | `8.80 ns` (🚀 **46.07x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `402.13 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `402.70 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `300.57 ns` (✅ **1.00x**) | `5.85 ns` (🚀 **51.34x faster**) | `5.92 ns` (🚀 **50.77x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `226.29 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.16 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.66 ns` (✅ **1.02x slower**) | `42.64 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.79 ns` (✅ **1.01x faster**) | `36.14 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.32 us` (✅ **1.01x faster**)  | `7.41 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `62.52 ns` (✅ **1.05x slower**) | `59.48 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.15 ns` (✅ **1.01x faster**) | `89.93 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.62 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**)        | `8.67 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.66 ns` (✅ **1.00x**)  | `31.45 ns` (🚀 **2.15x faster**)    | `31.18 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `58.98 ns` (✅ **1.00x**)  | `32.28 ns` (🚀 **1.83x faster**)    | `30.89 ns` (🚀 **1.91x faster**)     |
| **`deserialize_compressed`**             | `226.62 us` (✅ **1.00x**) | `50.01 ns` (🚀 **4531.81x faster**) | `51.52 ns` (🚀 **4399.05x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.77 us` (✅ **1.00x**)  | `50.54 ns` (🚀 **767.01x faster**)  | `51.60 ns` (🚀 **751.24x faster**)   |
| **`deserialize_uncompressed`**           | `188.12 us` (✅ **1.00x**) | `49.65 ns` (🚀 **3789.01x faster**) | `51.23 ns` (🚀 **3671.76x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `167.32 ns` (✅ **1.00x**) | `49.77 ns` (🚀 **3.36x faster**)    | `51.37 ns` (🚀 **3.26x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.89 us` (✅ **1.00x**) | `31.03 us` (❌ *2.61x slower*)    |
| **`legendre_for_qr`**    | `12.12 us` (✅ **1.00x**) | `10.88 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.90 ns` (✅ **1.00x**)       | `48.95 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `49.08 ns` (✅ **1.00x**)       | `48.96 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)        | `4.91 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.85 ns` (✅ **1.00x**) | `40.69 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.43 ns` (✅ **1.00x**) | `22.74 ns` (✅ **1.03x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

