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
|        | `51.87 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `380.82 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.94x faster**) | `8.15 ns` (🚀 **46.75x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `395.56 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **45.44x faster**) | `8.60 ns` (🚀 **46.02x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.95 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `395.82 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.10 ns` (✅ **1.00x**) | `9.10 ns` (🚀 **32.32x faster**) | `5.28 ns` (🚀 **55.67x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.28 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.28 ns` (✅ **1.00x slower**) | `37.23 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.76 ns` (✅ **1.02x faster**) | `32.31 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.22 us` (✅ **1.00x faster**)  | `6.25 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.14 ns` (✅ **1.00x slower**) | `52.98 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.65 ns` (✅ **1.01x faster**) | `80.58 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `6.54 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `3.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `65.38 ns` (✅ **1.00x**)  | `27.93 ns` (🚀 **2.34x faster**)    | `27.82 ns` (🚀 **2.35x faster**)     |
| **`serialize_uncompressed`**             | `52.47 ns` (✅ **1.00x**)  | `27.82 ns` (🚀 **1.89x faster**)    | `27.95 ns` (🚀 **1.88x faster**)     |
| **`deserialize_compressed`**             | `162.62 us` (✅ **1.00x**) | `46.62 ns` (🚀 **3488.48x faster**) | `45.58 ns` (🚀 **3567.66x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.26 us` (✅ **1.00x**)  | `46.61 ns` (🚀 **735.05x faster**)  | `45.58 ns` (🚀 **751.66x faster**)   |
| **`deserialize_uncompressed`**           | `128.72 us` (✅ **1.00x**) | `46.57 ns` (🚀 **2763.84x faster**) | `44.63 ns` (🚀 **2884.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `146.63 ns` (✅ **1.00x**) | `46.57 ns` (🚀 **3.15x faster**)    | `44.64 ns` (🚀 **3.28x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.36 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.42 us` (✅ **1.00x**) | `27.62 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.51 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `61.03 ns` (✅ **1.00x**) | `61.07 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.92 ns` (✅ **1.00x**) | `61.08 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.06 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.01x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.93 ns` (✅ **1.00x**) | `35.87 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.74 ns` (✅ **1.00x**) | `21.76 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

