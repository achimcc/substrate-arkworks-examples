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
| **`addition`**                        | `N/A`                   | `N/A`                          | `389.62 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.69x faster**) | `8.65 ns` (🚀 **45.04x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `406.84 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.16x faster**) | `8.80 ns` (🚀 **46.25x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.61 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `401.37 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `291.46 ns` (✅ **1.00x**) | `5.82 ns` (🚀 **50.09x faster**) | `5.85 ns` (🚀 **49.85x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `137.51 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x faster**)  | `6.18 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.35 ns` (✅ **1.02x slower**) | `42.58 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.86 ns` (✅ **1.03x slower**) | `34.94 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.89 us` (✅ **1.02x faster**)  | `7.00 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.56 ns` (✅ **1.00x faster**) | `61.69 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.09 ns` (✅ **1.01x faster**) | `89.85 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `7.62 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.57 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.69 ns` (✅ **1.00x**)  | `29.36 ns` (🚀 **2.34x faster**)    | `30.80 ns` (🚀 **2.23x faster**)     |
| **`serialize_uncompressed`**             | `58.02 ns` (✅ **1.00x**)  | `30.75 ns` (🚀 **1.89x faster**)    | `30.12 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `183.18 us` (✅ **1.00x**) | `49.45 ns` (🚀 **3704.58x faster**) | `52.05 ns` (🚀 **3519.22x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.50 us` (✅ **1.00x**)  | `49.14 ns` (🚀 **783.48x faster**)  | `52.06 ns` (🚀 **739.58x faster**)   |
| **`deserialize_uncompressed`**           | `144.63 us` (✅ **1.00x**) | `49.16 ns` (🚀 **2941.79x faster**) | `52.06 ns` (🚀 **2778.35x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.41 ns` (✅ **1.00x**) | `49.01 ns` (🚀 **3.40x faster**)    | `52.01 ns` (🚀 **3.20x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.16 us` (✅ **1.00x**) | `31.21 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.29 us` (✅ **1.00x**) | `10.86 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.94 ns` (✅ **1.00x**) | `48.95 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.89 ns` (✅ **1.00x**) | `48.98 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.43 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.19 ns` (✅ **1.00x**) | `40.60 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `22.82 ns` (✅ **1.04x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

