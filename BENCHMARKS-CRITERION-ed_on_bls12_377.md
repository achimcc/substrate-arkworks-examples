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
|        | `58.65 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `386.10 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.33x faster**) | `8.67 ns` (🚀 **44.52x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `404.17 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **45.94x faster**) | `8.80 ns` (🚀 **45.94x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.18 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `414.26 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `296.14 ns` (✅ **1.00x**) | `5.92 ns` (🚀 **49.99x faster**) | `5.82 ns` (🚀 **50.86x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.49 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.15 ns` (✅ **1.00x faster**)  | `6.18 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.74 ns` (✅ **1.02x slower**) | `43.06 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.72 ns` (✅ **1.01x faster**) | `36.15 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.94 us` (✅ **1.01x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.59 ns` (✅ **1.01x faster**) | `62.21 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.17 ns` (✅ **1.01x faster**) | `89.79 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.72 ns` (✅ **1.00x**) | `8.77 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.79 ns` (✅ **1.00x**) | `4.79 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.20 ns` (✅ **1.00x**)  | `31.03 ns` (🚀 **2.23x faster**)    | `31.50 ns` (🚀 **2.20x faster**)     |
| **`serialize_uncompressed`**             | `58.08 ns` (✅ **1.00x**)  | `30.75 ns` (🚀 **1.89x faster**)    | `31.78 ns` (🚀 **1.83x faster**)     |
| **`deserialize_compressed`**             | `181.95 us` (✅ **1.00x**) | `51.16 ns` (🚀 **3556.81x faster**) | `52.31 ns` (🚀 **3478.19x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.54 us` (✅ **1.00x**)  | `51.49 ns` (🚀 **748.41x faster**)  | `52.36 ns` (🚀 **736.08x faster**)   |
| **`deserialize_uncompressed`**           | `143.38 us` (✅ **1.00x**) | `50.65 ns` (🚀 **2830.84x faster**) | `52.08 ns` (🚀 **2752.95x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `168.13 ns` (✅ **1.00x**) | `50.97 ns` (🚀 **3.30x faster**)    | `52.09 ns` (🚀 **3.23x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.87 us` (✅ **1.00x**) | `31.13 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.12 us` (✅ **1.00x**) | `10.86 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.77 ns` (✅ **1.00x**) | `48.58 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.74 ns` (✅ **1.00x**) | `48.48 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.36 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.56 ns` (✅ **1.00x**) | `40.79 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `22.71 ns` (✅ **1.00x**) | `23.74 ns` (✅ **1.05x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

