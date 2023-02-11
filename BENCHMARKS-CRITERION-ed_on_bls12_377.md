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
|        | `58.76 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `387.35 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.51x faster**) | `8.64 ns` (🚀 **44.81x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `404.86 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **45.99x faster**) | `8.80 ns` (🚀 **46.02x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.19 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `413.63 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `297.53 ns` (✅ **1.00x**) | `5.88 ns` (🚀 **50.60x faster**) | `5.80 ns` (🚀 **51.30x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `139.05 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.16 ns` (✅ **1.00x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.45 ns` (✅ **1.02x slower**) | `42.53 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.56 ns` (✅ **1.02x faster**) | `36.23 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.04 us` (✅ **1.01x slower**)  | `6.99 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.37 ns` (✅ **1.01x faster**) | `62.03 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.02 ns` (✅ **1.01x faster**) | `89.71 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `8.67 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.08 ns` (✅ **1.00x**)  | `31.20 ns` (🚀 **2.18x faster**)    | `32.05 ns` (🚀 **2.12x faster**)     |
| **`serialize_uncompressed`**             | `57.55 ns` (✅ **1.00x**)  | `31.97 ns` (🚀 **1.80x faster**)    | `30.92 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `183.03 us` (✅ **1.00x**) | `50.03 ns` (🚀 **3658.18x faster**) | `52.28 ns` (🚀 **3500.80x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.70 us` (✅ **1.00x**)  | `50.20 ns` (🚀 **771.02x faster**)  | `52.21 ns` (🚀 **741.26x faster**)   |
| **`deserialize_uncompressed`**           | `144.26 us` (✅ **1.00x**) | `50.18 ns` (🚀 **2874.74x faster**) | `52.03 ns` (🚀 **2772.73x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `167.72 ns` (✅ **1.00x**) | `50.22 ns` (🚀 **3.34x faster**)    | `52.13 ns` (🚀 **3.22x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.85 us` (✅ **1.00x**) | `31.29 us` (❌ *2.64x slower*)    |
| **`legendre_for_qr`**    | `12.12 us` (✅ **1.00x**) | `10.93 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.70 ns` (✅ **1.00x**) | `48.79 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.72 ns` (✅ **1.00x**) | `48.72 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `5.00 ns` (✅ **1.00x**)  | `5.00 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)  | `5.38 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.76 ns` (✅ **1.00x**) | `40.76 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `22.93 ns` (✅ **1.04x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

