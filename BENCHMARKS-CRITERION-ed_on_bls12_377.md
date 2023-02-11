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
|        | `58.84 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `389.80 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.76x faster**) | `8.65 ns` (🚀 **45.08x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `405.28 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.04x faster**) | `8.79 ns` (🚀 **46.11x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `397.99 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `416.23 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `296.37 ns` (✅ **1.00x**) | `5.88 ns` (🚀 **50.42x faster**) | `5.80 ns` (🚀 **51.06x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `139.45 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.16 ns` (✅ **1.00x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.56 ns` (✅ **1.02x slower**) | `42.54 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.73 ns` (✅ **1.01x faster**) | `36.23 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.03 us` (✅ **1.01x slower**)  | `6.99 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.38 ns` (✅ **1.01x faster**) | `62.04 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.01 ns` (✅ **1.01x faster**) | `89.72 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `8.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.13 ns` (✅ **1.00x**)  | `32.02 ns` (🚀 **2.13x faster**)    | `32.40 ns` (🚀 **2.10x faster**)     |
| **`serialize_uncompressed`**             | `57.67 ns` (✅ **1.00x**)  | `31.68 ns` (🚀 **1.82x faster**)    | `30.89 ns` (🚀 **1.87x faster**)     |
| **`deserialize_compressed`**             | `183.42 us` (✅ **1.00x**) | `50.57 ns` (🚀 **3626.86x faster**) | `52.21 ns` (🚀 **3513.20x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.70 us` (✅ **1.00x**)  | `50.25 ns` (🚀 **770.27x faster**)  | `52.19 ns` (🚀 **741.56x faster**)   |
| **`deserialize_uncompressed`**           | `144.63 us` (✅ **1.00x**) | `50.20 ns` (🚀 **2881.01x faster**) | `52.14 ns` (🚀 **2774.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.48 ns` (✅ **1.00x**) | `50.15 ns` (🚀 **3.32x faster**)    | `52.14 ns` (🚀 **3.19x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.29 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.85 us` (✅ **1.00x**) | `31.28 us` (❌ *2.64x slower*)    |
| **`legendre_for_qr`**    | `12.13 us` (✅ **1.00x**) | `10.92 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.87 ns` (✅ **1.00x**) | `48.74 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.78 ns` (✅ **1.00x**) | `48.57 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)  | `5.00 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.76 ns` (✅ **1.00x**) | `40.75 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.87 ns` (✅ **1.00x**) | `22.93 ns` (✅ **1.04x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

