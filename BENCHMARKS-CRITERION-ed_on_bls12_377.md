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
|        | `51.89 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `381.21 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **46.05x faster**) | `8.14 ns` (🚀 **46.81x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `396.11 ns` (✅ **1.00x**) | `8.66 ns` (🚀 **45.75x faster**) | `8.62 ns` (🚀 **45.97x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `393.53 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `395.94 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.39 ns` (✅ **1.00x**) | `9.11 ns` (🚀 **32.31x faster**) | `5.28 ns` (🚀 **55.77x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.59 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.33 ns` (✅ **1.01x slower**) | `37.06 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.85 ns` (✅ **1.01x faster**) | `32.33 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.21 us` (✅ **1.01x faster**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.18 ns` (✅ **1.00x slower**) | `52.98 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `80.01 ns` (✅ **1.01x faster**) | `80.67 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.85 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `3.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `65.37 ns` (✅ **1.00x**)  | `27.94 ns` (🚀 **2.34x faster**)    | `28.06 ns` (🚀 **2.33x faster**)     |
| **`serialize_uncompressed`**             | `52.34 ns` (✅ **1.00x**)  | `27.83 ns` (🚀 **1.88x faster**)    | `28.18 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `163.08 us` (✅ **1.00x**) | `46.57 ns` (🚀 **3501.89x faster**) | `45.86 ns` (🚀 **3556.04x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.27 us` (✅ **1.00x**)  | `46.56 ns` (🚀 **736.01x faster**)  | `45.89 ns` (🚀 **746.82x faster**)   |
| **`deserialize_uncompressed`**           | `128.19 us` (✅ **1.00x**) | `46.54 ns` (🚀 **2754.61x faster**) | `44.63 ns` (🚀 **2872.63x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.24 ns` (✅ **1.00x**) | `46.54 ns` (🚀 **3.12x faster**)    | `44.59 ns` (🚀 **3.26x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.36 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.43 us` (✅ **1.00x**) | `27.59 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.60 us` (✅ **1.00x**) | `9.53 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.97 ns` (✅ **1.00x**) | `62.23 ns` (✅ **1.02x slower**)  |
| **`from_big-endian_bits`**    | `60.90 ns` (✅ **1.00x**) | `62.32 ns` (✅ **1.02x slower**)  |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.09 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.10 ns` (✅ **1.00x**) | `35.98 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `21.68 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

