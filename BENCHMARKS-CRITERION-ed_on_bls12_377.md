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
|        | `58.86 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `384.55 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.14x faster**) | `8.64 ns` (🚀 **44.51x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `402.97 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **45.72x faster**) | `8.79 ns` (🚀 **45.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.26 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `415.23 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `297.37 ns` (✅ **1.00x**) | `5.87 ns` (🚀 **50.62x faster**) | `5.84 ns` (🚀 **50.92x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `138.52 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.14 ns` (✅ **1.00x faster**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.94 ns` (✅ **1.02x slower**) | `43.02 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.78 ns` (✅ **1.02x slower**) | `35.21 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.06 us` (✅ **1.01x slower**)  | `6.99 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.14 ns` (✅ **1.01x faster**) | `61.62 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `91.53 ns` (✅ **1.01x faster**) | `92.18 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `8.64 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.05 ns` (✅ **1.00x**)  | `31.20 ns` (🚀 **2.18x faster**)    | `31.26 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `61.69 ns` (✅ **1.00x**)  | `31.84 ns` (🚀 **1.94x faster**)    | `30.47 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `184.97 us` (✅ **1.00x**) | `50.24 ns` (🚀 **3681.91x faster**) | `52.28 ns` (🚀 **3537.90x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.73 us` (✅ **1.00x**)  | `50.52 ns` (🚀 **766.55x faster**)  | `52.29 ns` (🚀 **740.61x faster**)   |
| **`deserialize_uncompressed`**           | `144.91 us` (✅ **1.00x**) | `50.62 ns` (🚀 **2862.77x faster**) | `52.05 ns` (🚀 **2784.05x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `174.78 ns` (✅ **1.00x**) | `50.33 ns` (🚀 **3.47x faster**)    | `52.05 ns` (🚀 **3.36x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.26 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.94 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.06 ns` (✅ **1.00x**) | `48.20 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.12 ns` (✅ **1.00x**) | `48.15 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `5.00 ns` (✅ **1.00x**)  | `5.01 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.45 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.92 ns` (✅ **1.00x**) | `40.57 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `22.82 ns` (✅ **1.00x**) | `23.96 ns` (✅ **1.05x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

