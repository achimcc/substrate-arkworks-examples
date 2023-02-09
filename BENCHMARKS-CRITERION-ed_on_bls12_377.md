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
|        | `58.33 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `385.63 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.24x faster**) | `8.64 ns` (🚀 **44.64x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `404.61 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **45.95x faster**) | `8.79 ns` (🚀 **46.01x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `398.58 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `413.71 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `290.86 ns` (✅ **1.00x**) | `5.87 ns` (🚀 **49.53x faster**) | `5.81 ns` (🚀 **50.04x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.72 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.15 ns` (✅ **1.00x faster**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.43 ns` (✅ **1.00x slower**) | `43.30 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.40 ns` (✅ **1.00x faster**) | `35.49 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.01x faster**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `60.98 ns` (✅ **1.02x faster**) | `62.12 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.08 ns` (✅ **1.01x faster**) | `89.86 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.62 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `8.70 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.25 ns` (✅ **1.00x**)  | `30.86 ns` (🚀 **2.24x faster**)    | `31.16 ns` (🚀 **2.22x faster**)     |
| **`serialize_uncompressed`**             | `57.51 ns` (✅ **1.00x**)  | `30.82 ns` (🚀 **1.87x faster**)    | `31.56 ns` (🚀 **1.82x faster**)     |
| **`deserialize_compressed`**             | `181.72 us` (✅ **1.00x**) | `50.55 ns` (🚀 **3594.61x faster**) | `52.33 ns` (🚀 **3472.95x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.48 us` (✅ **1.00x**)  | `50.73 ns` (🚀 **758.46x faster**)  | `52.35 ns` (🚀 **735.03x faster**)   |
| **`deserialize_uncompressed`**           | `143.18 us` (✅ **1.00x**) | `50.60 ns` (🚀 **2829.72x faster**) | `52.20 ns` (🚀 **2743.00x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.81 ns` (✅ **1.00x**) | `50.64 ns` (🚀 **3.29x faster**)    | `52.02 ns` (🚀 **3.21x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.16 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.28 us` (✅ **1.00x**) | `10.88 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.86 ns` (✅ **1.00x**) | `48.97 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.88 ns` (✅ **1.00x**) | `48.90 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)  | `5.42 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.87 ns` (✅ **1.00x**) | `40.53 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.28 ns` (✅ **1.00x**) | `23.75 ns` (✅ **1.02x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

