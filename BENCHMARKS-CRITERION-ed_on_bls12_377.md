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
|        | `52.70 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `380.03 ns` (✅ **1.00x**) | `8.29 ns` (🚀 **45.85x faster**) | `8.13 ns` (🚀 **46.73x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `396.37 ns` (✅ **1.00x**) | `8.74 ns` (🚀 **45.36x faster**) | `8.60 ns` (🚀 **46.08x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.65 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `398.15 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `292.16 ns` (✅ **1.00x**) | `9.08 ns` (🚀 **32.17x faster**) | `5.27 ns` (🚀 **55.43x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.59 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.94 ns` (✅ **1.00x slower**)  | `5.94 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.29 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.74 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.60 us` (✅ **1.04x slower**)  | `6.35 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.06 ns` (✅ **1.01x slower**) | `52.65 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `82.16 ns` (✅ **1.01x faster**) | `83.04 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**) | `6.54 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.00 ns` (✅ **1.00x**)  | `27.84 ns` (🚀 **2.44x faster**)    | `27.90 ns` (🚀 **2.44x faster**)     |
| **`serialize_uncompressed`**             | `55.84 ns` (✅ **1.00x**)  | `27.81 ns` (🚀 **2.01x faster**)    | `27.81 ns` (🚀 **2.01x faster**)     |
| **`deserialize_compressed`**             | `162.98 us` (✅ **1.00x**) | `44.87 ns` (🚀 **3632.15x faster**) | `44.68 ns` (🚀 **3647.62x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.61 us` (✅ **1.00x**)  | `44.87 ns` (🚀 **771.32x faster**)  | `44.67 ns` (🚀 **774.74x faster**)   |
| **`deserialize_uncompressed`**           | `128.17 us` (✅ **1.00x**) | `44.89 ns` (🚀 **2855.39x faster**) | `44.65 ns` (🚀 **2870.76x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `152.55 ns` (✅ **1.00x**) | `44.85 ns` (🚀 **3.40x faster**)    | `44.64 ns` (🚀 **3.42x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.37 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.42 us` (✅ **1.00x**) | `27.59 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.53 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.78 ns` (✅ **1.00x**) | `60.79 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.68 ns` (✅ **1.00x**) | `60.77 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.06 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.98 ns` (✅ **1.00x**) | `35.95 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.64 ns` (✅ **1.00x**) | `21.64 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

