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
|        | `58.34 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `383.87 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.00x faster**) | `8.63 ns` (🚀 **44.47x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `403.30 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **45.79x faster**) | `8.82 ns` (🚀 **45.71x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.99 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `414.73 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `291.92 ns` (✅ **1.00x**) | `5.87 ns` (🚀 **49.73x faster**) | `5.80 ns` (🚀 **50.29x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.63 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.15 ns` (✅ **1.00x faster**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.54 ns` (✅ **1.01x slower**) | `43.24 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.40 ns` (✅ **1.00x faster**) | `35.49 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.01x faster**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.03 ns` (✅ **1.02x faster**) | `62.05 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.08 ns` (✅ **1.01x faster**) | `89.87 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `7.63 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.70 ns` (✅ **1.00x**) | `8.70 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.44 ns` (✅ **1.00x**)  | `30.96 ns` (🚀 **2.24x faster**)    | `31.16 ns` (🚀 **2.23x faster**)     |
| **`serialize_uncompressed`**             | `57.51 ns` (✅ **1.00x**)  | `30.98 ns` (🚀 **1.86x faster**)    | `31.70 ns` (🚀 **1.81x faster**)     |
| **`deserialize_compressed`**             | `181.42 us` (✅ **1.00x**) | `50.68 ns` (🚀 **3579.66x faster**) | `52.24 ns` (🚀 **3473.14x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.49 us` (✅ **1.00x**)  | `50.62 ns` (🚀 **760.31x faster**)  | `52.19 ns` (🚀 **737.51x faster**)   |
| **`deserialize_uncompressed`**           | `142.91 us` (✅ **1.00x**) | `50.09 ns` (🚀 **2852.99x faster**) | `52.01 ns` (🚀 **2747.54x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.34 ns` (✅ **1.00x**) | `50.46 ns` (🚀 **3.30x faster**)    | `52.02 ns` (🚀 **3.20x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.17 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.91 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.91 ns` (✅ **1.00x**) | `48.87 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.85 ns` (✅ **1.00x**) | `48.72 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)  | `5.42 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.87 ns` (✅ **1.00x**) | `40.60 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.17 ns` (✅ **1.00x**) | `23.74 ns` (✅ **1.02x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

