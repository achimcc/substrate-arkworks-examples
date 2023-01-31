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
|        | `129.15 us` (✅ **1.00x**)       |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `376.48 ns` (✅ **1.00x**) | `8.27 ns` (🚀 **45.54x faster**) | `8.15 ns` (🚀 **46.20x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `394.10 ns` (✅ **1.00x**) | `8.62 ns` (🚀 **45.70x faster**) | `13.53 ns` (🚀 **29.12x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `386.13 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `391.92 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                          | `288.82 ns` (✅ **1.00x**) | `5.28 ns` (🚀 **54.68x faster**) | `5.28 ns` (🚀 **54.73x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `199.61 us` (✅ **1.00x**) | `N/A`                           | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `37.30 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `31.59 ns` (✅ **1.02x faster**) | `32.27 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.52 us` (✅ **1.03x slower**)  | `6.33 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `53.03 ns` (✅ **1.01x slower**) | `52.66 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `79.40 ns` (✅ **1.01x faster**) | `80.30 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**)        | `7.83 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.23 ns` (✅ **1.00x**)  | `28.02 ns` (🚀 **2.43x faster**)    | `27.86 ns` (🚀 **2.45x faster**)     |
| **`serialize_uncompressed`**             | `56.72 ns` (✅ **1.00x**)  | `27.91 ns` (🚀 **2.03x faster**)    | `27.74 ns` (🚀 **2.05x faster**)     |
| **`deserialize_compressed`**             | `201.89 us` (✅ **1.00x**) | `45.42 ns` (🚀 **4444.77x faster**) | `46.85 ns` (🚀 **4309.25x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.58 us` (✅ **1.00x**)  | `45.37 ns` (🚀 **762.11x faster**)  | `46.86 ns` (🚀 **737.92x faster**)   |
| **`deserialize_uncompressed`**           | `166.61 us` (✅ **1.00x**) | `46.99 ns` (🚀 **3545.83x faster**) | `45.54 ns` (🚀 **3658.25x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `146.19 ns` (✅ **1.00x**) | `41.40 ns` (🚀 **3.53x faster**)    | `45.57 ns` (🚀 **3.21x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.23 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.46 us` (✅ **1.00x**) | `24.38 us` (❌ *2.33x slower*)    |
| **`legendre_for_qr`**    | `10.63 us` (✅ **1.00x**) | `9.58 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `59.96 ns` (✅ **1.00x**)       | `59.88 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `59.79 ns` (✅ **1.00x**)       | `59.92 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `3.98 ns` (✅ **1.00x**)        | `3.96 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.48 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.93 ns` (✅ **1.00x**) | `35.91 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `22.03 ns` (✅ **1.00x**) | `22.03 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

