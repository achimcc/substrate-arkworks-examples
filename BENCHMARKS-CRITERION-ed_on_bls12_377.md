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
|        | `58.82 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `384.71 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.13x faster**) | `8.64 ns` (🚀 **44.51x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `403.35 ns` (✅ **1.00x**) | `8.83 ns` (🚀 **45.66x faster**) | `8.80 ns` (🚀 **45.84x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `402.29 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `416.15 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `297.33 ns` (✅ **1.00x**) | `5.87 ns` (🚀 **50.68x faster**) | `5.85 ns` (🚀 **50.79x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `138.17 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.15 ns` (✅ **1.00x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.19 ns` (✅ **1.00x slower**) | `43.06 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.79 ns` (✅ **1.02x slower**) | `35.19 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.06 us` (✅ **1.01x slower**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.21 ns` (✅ **1.01x faster**) | `61.64 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `91.60 ns` (✅ **1.01x faster**) | `92.21 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.62 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.65 ns` (✅ **1.00x**)  | `31.15 ns` (🚀 **2.17x faster**)    | `31.65 ns` (🚀 **2.14x faster**)     |
| **`serialize_uncompressed`**             | `60.42 ns` (✅ **1.00x**)  | `31.15 ns` (🚀 **1.94x faster**)    | `30.53 ns` (🚀 **1.98x faster**)     |
| **`deserialize_compressed`**             | `183.99 us` (✅ **1.00x**) | `50.73 ns` (🚀 **3626.91x faster**) | `52.29 ns` (🚀 **3518.64x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.75 us` (✅ **1.00x**)  | `50.41 ns` (🚀 **768.68x faster**)  | `52.28 ns` (🚀 **741.14x faster**)   |
| **`deserialize_uncompressed`**           | `145.18 us` (✅ **1.00x**) | `50.78 ns` (🚀 **2859.13x faster**) | `52.06 ns` (🚀 **2788.66x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `174.54 ns` (✅ **1.00x**) | `50.75 ns` (🚀 **3.44x faster**)    | `52.05 ns` (🚀 **3.35x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.16 us` (✅ **1.00x**) | `31.31 us` (❌ *2.58x slower*)    |
| **`legendre_for_qr`**    | `12.26 us` (✅ **1.00x**) | `10.94 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.12 ns` (✅ **1.00x**) | `47.93 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.17 ns` (✅ **1.00x**) | `47.95 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)  | `5.01 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.43 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.92 ns` (✅ **1.00x**) | `40.59 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `22.74 ns` (✅ **1.00x**) | `23.97 ns` (✅ **1.05x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

