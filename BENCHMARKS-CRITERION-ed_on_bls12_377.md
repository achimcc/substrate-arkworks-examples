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
|        | `52.69 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `381.94 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **46.09x faster**) | `8.14 ns` (🚀 **46.93x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `399.29 ns` (✅ **1.00x**) | `8.60 ns` (🚀 **46.42x faster**) | `8.62 ns` (🚀 **46.31x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.73 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `396.92 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.81 ns` (✅ **1.00x**) | `9.06 ns` (🚀 **32.53x faster**) | `5.35 ns` (🚀 **55.11x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.29 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.94 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.29 ns` (✅ **1.00x slower**) | `37.24 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.71 ns` (✅ **1.00x slower**) | `31.56 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.61 us` (✅ **1.06x slower**)  | `6.26 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.13 ns` (✅ **1.00x slower**) | `52.93 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.53 ns` (✅ **1.02x faster**) | `80.73 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `6.52 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.06 ns` (✅ **1.00x**)  | `27.82 ns` (🚀 **2.48x faster**)    | `28.02 ns` (🚀 **2.46x faster**)     |
| **`serialize_uncompressed`**             | `53.83 ns` (✅ **1.00x**)  | `27.79 ns` (🚀 **1.94x faster**)    | `27.80 ns` (🚀 **1.94x faster**)     |
| **`deserialize_compressed`**             | `163.25 us` (✅ **1.00x**) | `46.53 ns` (🚀 **3508.66x faster**) | `46.47 ns` (🚀 **3513.23x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.64 us` (✅ **1.00x**)  | `46.54 ns` (🚀 **744.21x faster**)  | `46.47 ns` (🚀 **745.41x faster**)   |
| **`deserialize_uncompressed`**           | `128.52 us` (✅ **1.00x**) | `46.53 ns` (🚀 **2761.85x faster**) | `45.99 ns` (🚀 **2794.25x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `144.67 ns` (✅ **1.00x**) | `46.54 ns` (🚀 **3.11x faster**)    | `46.08 ns` (🚀 **3.14x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.32 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.39 us` (✅ **1.00x**) | `27.59 us` (❌ *2.66x slower*)    |
| **`legendre_for_qr`**    | `10.60 us` (✅ **1.00x**) | `9.52 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.76 ns` (✅ **1.00x**) | `62.21 ns` (✅ **1.02x slower**)  |
| **`from_big-endian_bits`**    | `60.80 ns` (✅ **1.00x**) | `62.24 ns` (✅ **1.02x slower**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.06 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.87 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.92 ns` (✅ **1.00x**) | `35.87 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.71 ns` (✅ **1.00x**) | `21.72 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

