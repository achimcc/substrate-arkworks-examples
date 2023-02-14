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
|        | `66.86 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `389.09 ns` (✅ **1.00x**) | `8.73 ns` (🚀 **44.56x faster**) | `8.67 ns` (🚀 **44.87x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `407.43 ns` (✅ **1.00x**) | `9.97 ns` (🚀 **40.87x faster**) | `9.64 ns` (🚀 **42.27x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `398.76 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `414.30 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.06 ns` (✅ **1.00x**) | `5.85 ns` (🚀 **50.43x faster**) | `5.86 ns` (🚀 **50.33x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.39 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.12 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.95 ns` (✅ **1.00x slower**) | `42.81 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.45 ns` (✅ **1.01x slower**) | `35.05 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.02x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.58 ns` (✅ **1.00x faster**) | `61.62 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `90.65 ns` (✅ **1.01x slower**) | `89.96 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.74 ns` (✅ **1.00x**)        | `8.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.39 ns` (✅ **1.00x**)  | `30.91 ns` (🚀 **2.21x faster**)    | `31.25 ns` (🚀 **2.19x faster**)     |
| **`serialize_uncompressed`**             | `62.11 ns` (✅ **1.00x**)  | `30.35 ns` (🚀 **2.05x faster**)    | `30.78 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `181.78 us` (✅ **1.00x**) | `49.60 ns` (🚀 **3665.11x faster**) | `52.09 ns` (🚀 **3489.60x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.66 us` (✅ **1.00x**)  | `49.61 ns` (🚀 **779.40x faster**)  | `51.95 ns` (🚀 **744.30x faster**)   |
| **`deserialize_uncompressed`**           | `143.32 us` (✅ **1.00x**) | `49.57 ns` (🚀 **2891.17x faster**) | `51.99 ns` (🚀 **2756.90x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.56 ns` (✅ **1.00x**) | `49.60 ns` (🚀 **3.32x faster**)    | `51.98 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.27 us` (❌ *2.58x slower*)    |
| **`legendre_for_qr`**    | `12.36 us` (✅ **1.00x**) | `11.14 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.99 ns` (✅ **1.00x**)       | `48.66 ns` (✅ **1.01x faster**)  |
| **`from_big-endian_bits`**    | `48.57 ns` (✅ **1.00x**)       | `48.58 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.95 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.42 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.88 ns` (✅ **1.00x**) | `40.76 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.73 ns` (✅ **1.00x**) | `22.55 ns` (✅ **1.05x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

