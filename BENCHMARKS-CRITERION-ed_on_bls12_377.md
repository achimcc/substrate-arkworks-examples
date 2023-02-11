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
|        | `51.92 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `379.46 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.76x faster**) | `8.14 ns` (🚀 **46.62x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `397.91 ns` (✅ **1.00x**) | `8.64 ns` (🚀 **46.03x faster**) | `8.61 ns` (🚀 **46.21x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `392.78 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `398.53 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `292.25 ns` (✅ **1.00x**) | `9.11 ns` (🚀 **32.07x faster**) | `5.29 ns` (🚀 **55.20x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.15 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.96 ns` (✅ **1.00x slower**)  | `5.94 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.28 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.82 ns` (✅ **1.01x slower**) | `31.53 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.17 us` (✅ **1.02x faster**)  | `6.27 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.13 ns` (✅ **1.00x slower**) | `52.95 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.76 ns` (✅ **1.01x faster**) | `80.39 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `6.53 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `65.06 ns` (✅ **1.00x**)  | `27.84 ns` (🚀 **2.34x faster**)    | `28.06 ns` (🚀 **2.32x faster**)     |
| **`serialize_uncompressed`**             | `53.87 ns` (✅ **1.00x**)  | `27.73 ns` (🚀 **1.94x faster**)    | `27.97 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `162.65 us` (✅ **1.00x**) | `45.12 ns` (🚀 **3604.86x faster**) | `45.01 ns` (🚀 **3613.71x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.25 us` (✅ **1.00x**)  | `45.12 ns` (🚀 **759.08x faster**)  | `45.01 ns` (🚀 **760.98x faster**)   |
| **`deserialize_uncompressed`**           | `128.70 us` (✅ **1.00x**) | `44.91 ns` (🚀 **2865.59x faster**) | `44.39 ns` (🚀 **2899.31x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `150.70 ns` (✅ **1.00x**) | `44.90 ns` (🚀 **3.36x faster**)    | `44.70 ns` (🚀 **3.37x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.41 us` (✅ **1.00x**) | `27.60 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.63 us` (✅ **1.00x**) | `9.52 us` (✅ **1.12x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.95 ns` (✅ **1.00x**) | `60.96 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.96 ns` (✅ **1.00x**) | `60.89 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.07 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.05 ns` (✅ **1.00x**) | `36.01 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.71 ns` (✅ **1.00x**) | `21.65 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

