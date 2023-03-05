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
|        | `51.86 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `382.82 ns` (✅ **1.00x**) | `8.32 ns` (🚀 **46.03x faster**) | `8.13 ns` (🚀 **47.07x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `398.11 ns` (✅ **1.00x**) | `8.69 ns` (🚀 **45.82x faster**) | `8.61 ns` (🚀 **46.25x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.62 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `396.17 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.18 ns` (✅ **1.00x**) | `5.32 ns` (🚀 **55.07x faster**) | `5.28 ns` (🚀 **55.54x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.27 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.31 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.80 ns` (✅ **1.01x slower**) | `31.57 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.23 us` (✅ **1.00x faster**)  | `6.24 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.21 ns` (✅ **1.01x slower**) | `52.66 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.83 ns` (✅ **1.01x faster**) | `80.71 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.85 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.93 ns` (✅ **1.00x**)  | `27.90 ns` (🚀 **2.33x faster**)    | `28.12 ns` (🚀 **2.31x faster**)     |
| **`serialize_uncompressed`**             | `52.47 ns` (✅ **1.00x**)  | `27.81 ns` (🚀 **1.89x faster**)    | `28.16 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `163.13 us` (✅ **1.00x**) | `44.94 ns` (🚀 **3629.86x faster**) | `46.34 ns` (🚀 **3520.29x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.23 us` (✅ **1.00x**)  | `44.94 ns` (🚀 **761.61x faster**)  | `46.34 ns` (🚀 **738.61x faster**)   |
| **`deserialize_uncompressed`**           | `128.49 us` (✅ **1.00x**) | `46.39 ns` (🚀 **2769.90x faster**) | `44.71 ns` (🚀 **2874.09x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.13 ns` (✅ **1.00x**) | `46.39 ns` (🚀 **3.13x faster**)    | `44.71 ns` (🚀 **3.25x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.38 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.46 us` (✅ **1.00x**) | `27.59 us` (❌ *2.64x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.54 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `61.09 ns` (✅ **1.00x**) | `61.14 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `61.06 ns` (✅ **1.00x**) | `61.06 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.06 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.04 ns` (✅ **1.00x**) | `36.04 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.65 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

