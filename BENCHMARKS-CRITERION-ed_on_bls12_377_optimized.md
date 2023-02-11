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
|        | `66.51 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `384.73 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.23x faster**) | `8.65 ns` (🚀 **44.49x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `404.69 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.04x faster**) | `8.79 ns` (🚀 **46.05x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `400.49 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `411.48 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `299.76 ns` (✅ **1.00x**) | `5.83 ns` (🚀 **51.46x faster**) | `5.86 ns` (🚀 **51.15x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.87 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.00x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `43.41 ns` (✅ **1.00x slower**) | `43.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.66 ns` (✅ **1.02x slower**) | `34.98 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.01x faster**)  | `6.99 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.51 ns` (✅ **1.01x faster**) | `61.86 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `88.93 ns` (✅ **1.01x faster**) | `89.90 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.63 ns` (✅ **1.00x**)        | `8.63 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.70 ns` (✅ **1.00x**)  | `30.60 ns` (🚀 **2.25x faster**)    | `30.73 ns` (🚀 **2.24x faster**)     |
| **`serialize_uncompressed`**             | `61.54 ns` (✅ **1.00x**)  | `30.82 ns` (🚀 **2.00x faster**)    | `30.51 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `182.54 us` (✅ **1.00x**) | `51.09 ns` (🚀 **3572.66x faster**) | `51.85 ns` (🚀 **3520.44x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.41 us` (✅ **1.00x**)  | `51.13 ns` (🚀 **751.12x faster**)  | `51.86 ns` (🚀 **740.58x faster**)   |
| **`deserialize_uncompressed`**           | `144.00 us` (✅ **1.00x**) | `51.04 ns` (🚀 **2821.33x faster**) | `51.87 ns` (🚀 **2776.27x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `163.96 ns` (✅ **1.00x**) | `51.08 ns` (🚀 **3.21x faster**)    | `51.85 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.32 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.11 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.86 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.17 ns` (✅ **1.00x**)       | `48.10 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.10 ns` (✅ **1.00x**)       | `48.18 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)        | `5.45 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `40.74 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `22.96 ns` (✅ **1.00x**) | `23.74 ns` (✅ **1.03x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

