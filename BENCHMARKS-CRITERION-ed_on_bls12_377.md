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
|        | `51.91 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `380.92 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.93x faster**) | `8.14 ns` (🚀 **46.80x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `396.40 ns` (✅ **1.00x**) | `8.66 ns` (🚀 **45.77x faster**) | `8.64 ns` (🚀 **45.89x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `392.23 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `397.36 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.56 ns` (✅ **1.00x**) | `9.10 ns` (🚀 **32.38x faster**) | `5.28 ns` (🚀 **55.83x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.38 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.94 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.28 ns` (✅ **1.00x slower**) | `37.28 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.80 ns` (✅ **1.02x faster**) | `32.33 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.21 us` (✅ **1.01x faster**)  | `6.26 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.17 ns` (✅ **1.00x slower**) | `52.99 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `80.15 ns` (✅ **1.01x faster**) | `80.67 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `3.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `65.19 ns` (✅ **1.00x**)  | `27.91 ns` (🚀 **2.34x faster**)    | `28.05 ns` (🚀 **2.32x faster**)     |
| **`serialize_uncompressed`**             | `52.44 ns` (✅ **1.00x**)  | `27.81 ns` (🚀 **1.89x faster**)    | `28.16 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `162.64 us` (✅ **1.00x**) | `46.65 ns` (🚀 **3486.15x faster**) | `45.63 ns` (🚀 **3564.57x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.27 us` (✅ **1.00x**)  | `46.65 ns` (🚀 **734.55x faster**)  | `45.67 ns` (🚀 **750.37x faster**)   |
| **`deserialize_uncompressed`**           | `128.24 us` (✅ **1.00x**) | `46.60 ns` (🚀 **2752.10x faster**) | `44.64 ns` (🚀 **2872.86x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.81 ns` (✅ **1.00x**) | `46.59 ns` (🚀 **3.13x faster**)    | `44.63 ns` (🚀 **3.27x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.38 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.42 us` (✅ **1.00x**) | `27.60 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.62 us` (✅ **1.00x**) | `9.51 us` (✅ **1.12x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.99 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `61.00 ns` (✅ **1.00x**) | `60.95 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `61.08 ns` (✅ **1.00x**) | `61.06 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.08 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.95 ns` (✅ **1.00x**) | `35.91 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.77 ns` (✅ **1.00x**) | `21.65 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

