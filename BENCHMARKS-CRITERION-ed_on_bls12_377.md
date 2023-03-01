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
|        | `56.38 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                              |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `405.88 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.07x faster**) | `8.41 ns` (🚀 **48.24x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `428.76 ns` (✅ **1.00x**) | `8.87 ns` (🚀 **48.34x faster**) | `10.16 ns` (🚀 **42.18x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `411.94 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `422.18 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                          | `319.14 ns` (✅ **1.00x**) | `9.42 ns` (🚀 **33.89x faster**) | `5.64 ns` (🚀 **56.54x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `131.00 us` (✅ **1.00x**) | `N/A`                           | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.26 ns` (✅ **1.01x faster**)  | `6.30 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `38.30 ns` (✅ **1.01x faster**) | `38.85 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `33.01 ns` (✅ **1.01x faster**) | `33.39 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.87 us` (✅ **1.06x slower**)  | `6.46 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `57.05 ns` (✅ **1.04x slower**) | `54.94 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.37 ns` (✅ **1.08x slower**) | `82.58 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.97 ns` (✅ **1.00x**) | `7.55 ns` (✅ **1.08x slower**) | `N/A`                     | `N/A`                           | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.24 ns` (✅ **1.00x**) | `8.16 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                           | `N/A`                             |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**) | `4.10 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                           | `N/A`                             |
| **`division_by_2`**                   | `4.33 ns` (✅ **1.00x**) | `3.92 ns` (✅ **1.10x faster**) | `N/A`                     | `N/A`                           | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `73.22 ns` (✅ **1.00x**)  | `29.56 ns` (🚀 **2.48x faster**)    | `30.71 ns` (🚀 **2.38x faster**)     |
| **`serialize_uncompressed`**             | `56.56 ns` (✅ **1.00x**)  | `29.53 ns` (🚀 **1.92x faster**)    | `28.87 ns` (🚀 **1.96x faster**)     |
| **`deserialize_compressed`**             | `191.36 us` (✅ **1.00x**) | `48.61 ns` (🚀 **3936.22x faster**) | `47.01 ns` (🚀 **4070.63x faster**)  |
| **`deserialize_compressed_unchecked`**   | `36.55 us` (✅ **1.00x**)  | `48.30 ns` (🚀 **756.74x faster**)  | `46.85 ns` (🚀 **780.08x faster**)   |
| **`deserialize_uncompressed`**           | `137.12 us` (✅ **1.00x**) | `48.79 ns` (🚀 **2810.62x faster**) | `49.31 ns` (🚀 **2780.71x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `150.98 ns` (✅ **1.00x**) | `50.01 ns` (🚀 **3.02x faster**)    | `47.84 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.46 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.70 us` (✅ **1.00x**) | `28.71 us` (❌ *2.68x slower*)    |
| **`legendre_for_qr`**    | `12.20 us` (✅ **1.00x**) | `9.92 us` (✅ **1.23x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.15 ns` (✅ **1.00x**)  | `4.17 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `63.57 ns` (✅ **1.00x**) | `63.96 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `65.29 ns` (✅ **1.00x**) | `63.96 ns` (✅ **1.02x faster**)  |
| **`comparison`**              | `4.25 ns` (✅ **1.00x**)  | `4.26 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.73 ns` (✅ **1.00x**)  | `4.62 ns` (✅ **1.02x faster**)   |
| **`is_zero`**                 | `4.04 ns` (✅ **1.00x**)  | `4.07 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `39.85 ns` (✅ **1.03x faster**)  |
| **`into_bigint`** | `22.81 ns` (✅ **1.00x**) | `26.06 ns` (❌ *1.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

