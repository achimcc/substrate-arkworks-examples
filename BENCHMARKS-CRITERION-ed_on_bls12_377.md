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
|        | `59.26 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `388.28 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.60x faster**) | `8.64 ns` (🚀 **44.94x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `407.02 ns` (✅ **1.00x**) | `8.80 ns` (🚀 **46.24x faster**) | `8.79 ns` (🚀 **46.32x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.73 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `404.03 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.86 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.16x faster**) | `5.88 ns` (🚀 **50.02x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `137.10 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.18 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.16 ns` (✅ **1.01x slower**) | `42.89 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.40 ns` (✅ **1.00x faster**) | `35.40 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.17 us` (✅ **1.03x slower**)  | `6.99 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.38 ns` (✅ **1.01x faster**) | `61.80 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.06 ns` (✅ **1.01x faster**) | `89.86 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `8.64 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.72 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.02x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.18 ns` (✅ **1.00x**)  | `31.21 ns` (🚀 **2.18x faster**)    | `31.92 ns` (🚀 **2.14x faster**)     |
| **`serialize_uncompressed`**             | `56.57 ns` (✅ **1.00x**)  | `31.08 ns` (🚀 **1.82x faster**)    | `31.10 ns` (🚀 **1.82x faster**)     |
| **`deserialize_compressed`**             | `182.83 us` (✅ **1.00x**) | `50.41 ns` (🚀 **3626.75x faster**) | `52.50 ns` (🚀 **3482.48x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.95 us` (✅ **1.00x**)  | `49.98 ns` (🚀 **779.23x faster**)  | `52.49 ns` (🚀 **741.99x faster**)   |
| **`deserialize_uncompressed`**           | `143.80 us` (✅ **1.00x**) | `50.06 ns` (🚀 **2872.30x faster**) | `52.29 ns` (🚀 **2750.04x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.95 ns` (✅ **1.00x**) | `49.91 ns` (🚀 **3.35x faster**)    | `52.28 ns` (🚀 **3.19x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.39 us` (❌ *2.59x slower*)    |
| **`legendre_for_qr`**    | `12.31 us` (✅ **1.00x**) | `10.96 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.58 ns` (✅ **1.00x**) | `48.66 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.83 ns` (✅ **1.00x**) | `48.74 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.43 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.52 ns` (✅ **1.00x**) | `40.77 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `23.90 ns` (✅ **1.00x**) | `23.76 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

