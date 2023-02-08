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
|        | `52.30 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                             | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:---------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `379.52 ns` (✅ **1.00x**) | `8.30 ns` (🚀 **45.75x faster**)  | `7.88 ns` (🚀 **48.19x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `396.71 ns` (✅ **1.00x**) | `13.70 ns` (🚀 **28.97x faster**) | `8.58 ns` (🚀 **46.23x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `395.26 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `396.23 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `295.43 ns` (✅ **1.00x**) | `9.09 ns` (🚀 **32.51x faster**)  | `5.29 ns` (🚀 **55.80x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.96 us` (✅ **1.00x**) | `N/A`                            | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)   | `5.94 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.32 ns` (✅ **1.00x slower**)  | `37.24 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.75 ns` (✅ **1.02x faster**)  | `32.31 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.29 us` (✅ **1.01x slower**)   | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `52.89 ns` (✅ **1.00x faster**)  | `52.99 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `80.01 ns` (✅ **1.01x faster**)  | `80.91 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `65.20 ns` (✅ **1.00x**)  | `27.87 ns` (🚀 **2.34x faster**)    | `27.94 ns` (🚀 **2.33x faster**)     |
| **`serialize_uncompressed`**             | `52.52 ns` (✅ **1.00x**)  | `27.80 ns` (🚀 **1.89x faster**)    | `27.91 ns` (🚀 **1.88x faster**)     |
| **`deserialize_compressed`**             | `162.77 us` (✅ **1.00x**) | `46.63 ns` (🚀 **3490.45x faster**) | `45.57 ns` (🚀 **3572.21x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.33 us` (✅ **1.00x**)  | `46.64 ns` (🚀 **736.03x faster**)  | `45.56 ns` (🚀 **753.48x faster**)   |
| **`deserialize_uncompressed`**           | `128.33 us` (✅ **1.00x**) | `46.59 ns` (🚀 **2754.75x faster**) | `44.63 ns` (🚀 **2875.50x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.13 ns` (✅ **1.00x**) | `46.59 ns` (🚀 **3.11x faster**)    | `44.63 ns` (🚀 **3.25x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.37 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.43 us` (✅ **1.00x**) | `27.62 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.60 us` (✅ **1.00x**) | `9.64 us` (✅ **1.10x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.86 ns` (✅ **1.00x**) | `61.25 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `60.74 ns` (✅ **1.00x**) | `61.29 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `4.12 ns` (✅ **1.00x**)  | `4.07 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.90 ns` (✅ **1.00x**) | `35.95 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `21.73 ns` (✅ **1.00x**) | `21.73 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

