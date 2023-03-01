# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_edonbls12_377](#sample_edonbls12_377)
    - [arithmetic_for_edonbls12_377](#arithmetic_for_edonbls12_377)
    - [serialization_for_edonbls12_377](#serialization_for_edonbls12_377)
    - [msm_for_edonbls12_377](#msm_for_edonbls12_377)
    - [squareroot_for_edonbls12_377](#squareroot_for_edonbls12_377)
    - [bitwise_operations_for_edonbls12_377](#bitwise_operations_for_edonbls12_377)
    - [conversions_for_edonbls12_377](#conversions_for_edonbls12_377)

## Benchmark Results

### sample_edonbls12_377

|        | `g_elements`              |
|:-------|:------------------------- |
|        | `52.64 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `382.20 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **46.13x faster**) | `8.13 ns` (🚀 **46.99x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `398.67 ns` (✅ **1.00x**) | `8.61 ns` (🚀 **46.29x faster**) | `8.66 ns` (🚀 **46.02x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.41 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `396.69 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `295.96 ns` (✅ **1.00x**) | `9.08 ns` (🚀 **32.61x faster**) | `5.38 ns` (🚀 **55.05x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.25 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.31 ns` (✅ **1.00x slower**) | `37.24 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.75 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.60 us` (✅ **1.05x slower**)  | `6.26 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.13 ns` (✅ **1.01x slower**) | `52.70 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.67 ns` (✅ **1.01x faster**) | `80.64 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.05 ns` (✅ **1.00x**)  | `27.86 ns` (🚀 **2.48x faster**)    | `28.02 ns` (🚀 **2.46x faster**)     |
| **`serialize_uncompressed`**             | `53.83 ns` (✅ **1.00x**)  | `27.83 ns` (🚀 **1.93x faster**)    | `27.80 ns` (🚀 **1.94x faster**)     |
| **`deserialize_compressed`**             | `163.39 us` (✅ **1.00x**) | `46.52 ns` (🚀 **3512.01x faster**) | `46.36 ns` (🚀 **3524.76x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.64 us` (✅ **1.00x**)  | `46.54 ns` (🚀 **744.34x faster**)  | `46.35 ns` (🚀 **747.31x faster**)   |
| **`deserialize_uncompressed`**           | `128.51 us` (✅ **1.00x**) | `46.50 ns` (🚀 **2763.85x faster**) | `45.47 ns` (🚀 **2826.42x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.69 ns` (✅ **1.00x**) | `46.50 ns` (🚀 **3.13x faster**)    | `45.45 ns` (🚀 **3.21x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.36 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.39 us` (✅ **1.00x**) | `27.58 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.84 us` (✅ **1.00x**) | `9.52 us` (✅ **1.14x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.77 ns` (✅ **1.00x**) | `60.76 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.77 ns` (✅ **1.00x**) | `60.82 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.01x slower**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.10 ns` (✅ **1.00x**) | `35.86 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.70 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

