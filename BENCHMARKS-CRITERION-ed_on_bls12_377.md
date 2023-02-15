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
|        | `58.43 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `389.96 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.84x faster**) | `8.64 ns` (🚀 **45.11x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `405.71 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.11x faster**) | `8.79 ns` (🚀 **46.16x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.17 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `413.25 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.74 ns` (✅ **1.00x**) | `5.92 ns` (🚀 **49.79x faster**) | `5.81 ns` (🚀 **50.72x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.74 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.15 ns` (✅ **1.00x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.66 ns` (✅ **1.01x slower**) | `43.13 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.70 ns` (✅ **1.01x faster**) | `36.16 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.92 us` (✅ **1.01x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.61 ns` (✅ **1.01x faster**) | `62.20 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.14 ns` (✅ **1.01x faster**) | `89.75 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `8.71 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.79 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.57 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.28 ns` (✅ **1.00x**)  | `30.61 ns` (🚀 **2.26x faster**)    | `30.32 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `57.68 ns` (✅ **1.00x**)  | `30.77 ns` (🚀 **1.87x faster**)    | `31.82 ns` (🚀 **1.81x faster**)     |
| **`deserialize_compressed`**             | `182.31 us` (✅ **1.00x**) | `51.21 ns` (🚀 **3560.13x faster**) | `52.28 ns` (🚀 **3487.06x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.54 us` (✅ **1.00x**)  | `51.10 ns` (🚀 **754.25x faster**)  | `52.27 ns` (🚀 **737.43x faster**)   |
| **`deserialize_uncompressed`**           | `143.71 us` (✅ **1.00x**) | `51.00 ns` (🚀 **2818.20x faster**) | `52.08 ns` (🚀 **2759.57x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `168.09 ns` (✅ **1.00x**) | `50.96 ns` (🚀 **3.30x faster**)    | `52.09 ns` (🚀 **3.23x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.87 us` (✅ **1.00x**) | `31.13 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.14 us` (✅ **1.00x**) | `10.86 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.87 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.91 ns` (✅ **1.00x**) | `49.04 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.86 ns` (✅ **1.00x**) | `49.04 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.36 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.91 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.58 ns` (✅ **1.00x**) | `40.79 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `22.56 ns` (✅ **1.00x**) | `23.82 ns` (✅ **1.06x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

