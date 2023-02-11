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
|        | `52.75 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                             | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:---------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `380.15 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.91x faster**)  | `8.15 ns` (🚀 **46.67x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `400.28 ns` (✅ **1.00x**) | `13.72 ns` (🚀 **29.18x faster**) | `8.62 ns` (🚀 **46.44x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.95 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `397.73 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `296.57 ns` (✅ **1.00x**) | `9.11 ns` (🚀 **32.55x faster**)  | `5.27 ns` (🚀 **56.25x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.22 us` (✅ **1.00x**) | `N/A`                            | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.03x slower**)   | `5.78 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.19 ns` (✅ **1.00x slower**)  | `37.06 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.79 ns` (✅ **1.01x slower**)  | `31.47 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.62 us` (✅ **1.05x slower**)   | `6.31 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.24 ns` (✅ **1.01x slower**)  | `52.96 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.58 ns` (✅ **1.02x faster**)  | `81.10 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `6.55 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.85 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                            |
| **`division_by_2`**                   | `3.72 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.34 ns` (✅ **1.00x**)  | `27.86 ns` (🚀 **2.49x faster**)    | `28.04 ns` (🚀 **2.47x faster**)     |
| **`serialize_uncompressed`**             | `53.88 ns` (✅ **1.00x**)  | `27.90 ns` (🚀 **1.93x faster**)    | `27.95 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `163.20 us` (✅ **1.00x**) | `44.93 ns` (🚀 **3631.89x faster**) | `44.83 ns` (🚀 **3640.41x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.66 us` (✅ **1.00x**)  | `44.92 ns` (🚀 **771.65x faster**)  | `44.80 ns` (🚀 **773.69x faster**)   |
| **`deserialize_uncompressed`**           | `128.98 us` (✅ **1.00x**) | `44.89 ns` (🚀 **2873.10x faster**) | `44.69 ns` (🚀 **2886.42x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.17 ns` (✅ **1.00x**) | `44.88 ns` (🚀 **3.23x faster**)    | `44.68 ns` (🚀 **3.25x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.37 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.50 us` (✅ **1.00x**) | `27.55 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `10.57 us` (✅ **1.00x**) | `9.53 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `62.93 ns` (✅ **1.00x**) | `63.75 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `63.77 ns` (✅ **1.00x**) | `63.73 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.07 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `3.87 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.90 ns` (✅ **1.00x**) | `35.94 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.75 ns` (✅ **1.00x**) | `21.66 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

