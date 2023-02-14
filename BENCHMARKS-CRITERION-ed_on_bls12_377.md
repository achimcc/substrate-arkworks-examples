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
|        | `51.94 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `380.96 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **45.98x faster**) | `8.14 ns` (🚀 **46.77x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `394.44 ns` (✅ **1.00x**) | `8.69 ns` (🚀 **45.41x faster**) | `8.63 ns` (🚀 **45.71x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `392.33 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `396.92 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.67 ns` (✅ **1.00x**) | `9.11 ns` (🚀 **32.23x faster**) | `5.29 ns` (🚀 **55.51x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.70 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.96 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.30 ns` (✅ **1.00x slower**) | `37.26 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.83 ns` (✅ **1.02x faster**) | `32.35 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.21 us` (✅ **1.01x faster**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.15 ns` (✅ **1.00x slower**) | `52.99 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `80.04 ns` (✅ **1.01x faster**) | `80.62 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `3.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `65.43 ns` (✅ **1.00x**)  | `27.93 ns` (🚀 **2.34x faster**)    | `28.03 ns` (🚀 **2.33x faster**)     |
| **`serialize_uncompressed`**             | `52.43 ns` (✅ **1.00x**)  | `27.83 ns` (🚀 **1.88x faster**)    | `28.14 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `163.59 us` (✅ **1.00x**) | `46.64 ns` (🚀 **3507.74x faster**) | `45.34 ns` (🚀 **3607.94x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.26 us` (✅ **1.00x**)  | `46.65 ns` (🚀 **734.35x faster**)  | `45.37 ns` (🚀 **755.11x faster**)   |
| **`deserialize_uncompressed`**           | `129.13 us` (✅ **1.00x**) | `46.59 ns` (🚀 **2772.00x faster**) | `44.66 ns` (🚀 **2891.33x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `144.67 ns` (✅ **1.00x**) | `46.59 ns` (🚀 **3.10x faster**)    | `44.46 ns` (🚀 **3.25x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.38 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.42 us` (✅ **1.00x**) | `27.63 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.60 us` (✅ **1.00x**) | `9.57 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `3.99 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.95 ns` (✅ **1.00x**) | `61.03 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.94 ns` (✅ **1.00x**) | `60.93 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.07 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `3.91 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.09 ns` (✅ **1.00x**) | `35.95 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `21.66 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

