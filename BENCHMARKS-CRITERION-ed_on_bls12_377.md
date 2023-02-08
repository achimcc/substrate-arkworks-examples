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
|        | `58.42 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `387.60 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.46x faster**) | `8.63 ns` (🚀 **44.90x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `406.10 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.07x faster**) | `8.80 ns` (🚀 **46.16x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.78 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `404.04 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `291.70 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **49.75x faster**) | `5.84 ns` (🚀 **49.92x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.05 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `42.73 ns` (✅ **1.01x faster**) | `43.18 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.89 ns` (✅ **1.02x slower**) | `35.24 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.01x faster**)  | `6.97 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.65 ns` (✅ **1.01x faster**) | `62.01 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.12 ns` (✅ **1.01x faster**) | `89.84 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.79 ns` (✅ **1.00x**) | `4.81 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.67 ns` (✅ **1.00x**) | `4.63 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.93 ns` (✅ **1.00x**)  | `31.18 ns` (🚀 **2.21x faster**)    | `30.46 ns` (🚀 **2.26x faster**)     |
| **`serialize_uncompressed`**             | `57.52 ns` (✅ **1.00x**)  | `30.86 ns` (🚀 **1.86x faster**)    | `30.77 ns` (🚀 **1.87x faster**)     |
| **`deserialize_compressed`**             | `181.41 us` (✅ **1.00x**) | `49.52 ns` (🚀 **3663.76x faster**) | `52.54 ns` (🚀 **3453.04x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.57 us` (✅ **1.00x**)  | `49.14 ns` (🚀 **784.76x faster**)  | `52.54 ns` (🚀 **734.06x faster**)   |
| **`deserialize_uncompressed`**           | `142.83 us` (✅ **1.00x**) | `49.55 ns` (🚀 **2882.81x faster**) | `52.45 ns` (🚀 **2723.23x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `168.64 ns` (✅ **1.00x**) | `49.02 ns` (🚀 **3.44x faster**)    | `52.46 ns` (🚀 **3.21x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.19 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.89 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.06 ns` (✅ **1.00x**) | `48.18 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.13 ns` (✅ **1.00x**) | `48.17 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.45 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.96 ns` (✅ **1.00x**) | `41.13 ns` (✅ **1.00x slower**)  |
| **`into_bigint`** | `22.68 ns` (✅ **1.00x**) | `22.78 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

