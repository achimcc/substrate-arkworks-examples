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
|        | `58.35 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `387.07 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.39x faster**) | `8.65 ns` (🚀 **44.77x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `407.55 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.36x faster**) | `8.80 ns` (🚀 **46.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.33 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `413.80 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.31 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.09x faster**) | `5.87 ns` (🚀 **50.00x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `137.44 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.86 ns` (✅ **1.02x slower**) | `43.08 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.93 ns` (✅ **1.01x faster**) | `36.16 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.89 us` (✅ **1.01x faster**)  | `6.96 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.64 ns` (✅ **1.00x faster**) | `61.78 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.13 ns` (✅ **1.01x faster**) | `89.87 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.64 ns` (✅ **1.00x**) | `7.64 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.22 ns` (✅ **1.00x**)  | `30.75 ns` (🚀 **2.25x faster**)    | `30.76 ns` (🚀 **2.25x faster**)     |
| **`serialize_uncompressed`**             | `58.01 ns` (✅ **1.00x**)  | `30.53 ns` (🚀 **1.90x faster**)    | `31.21 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `182.65 us` (✅ **1.00x**) | `50.51 ns` (🚀 **3616.39x faster**) | `52.46 ns` (🚀 **3481.48x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.53 us` (✅ **1.00x**)  | `50.52 ns` (🚀 **762.64x faster**)  | `52.44 ns` (🚀 **734.74x faster**)   |
| **`deserialize_uncompressed`**           | `144.09 us` (✅ **1.00x**) | `50.50 ns` (🚀 **2853.30x faster**) | `52.38 ns` (🚀 **2750.87x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `175.14 ns` (✅ **1.00x**) | `50.58 ns` (🚀 **3.46x faster**)    | `52.36 ns` (🚀 **3.34x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.86 us` (✅ **1.00x**) | `31.11 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.12 us` (✅ **1.00x**) | `10.86 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.93 ns` (✅ **1.00x**) | `48.93 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.93 ns` (✅ **1.00x**) | `48.93 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `4.88 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)  | `5.41 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.08 ns` (✅ **1.00x**) | `40.89 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.77 ns` (✅ **1.00x**) | `23.73 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

