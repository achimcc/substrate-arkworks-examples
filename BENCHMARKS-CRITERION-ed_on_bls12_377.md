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
|        | `62.00 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `450.13 ns` (✅ **1.00x**) | `9.91 ns` (🚀 **45.42x faster**)  | `9.62 ns` (🚀 **46.79x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `474.68 ns` (✅ **1.00x**) | `16.43 ns` (🚀 **28.90x faster**) | `10.22 ns` (🚀 **46.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `469.35 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `478.05 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                          | `352.15 ns` (✅ **1.00x**) | `11.48 ns` (🚀 **30.67x faster**) | `6.71 ns` (🚀 **52.48x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `146.94 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `7.13 ns` (✅ **1.01x slower**)   | `7.08 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `45.52 ns` (✅ **1.02x slower**)  | `44.61 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `38.20 ns` (✅ **1.00x faster**)  | `38.20 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.55 us` (✅ **1.02x slower**)   | `7.41 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `63.77 ns` (✅ **1.02x slower**)  | `62.73 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `95.94 ns` (✅ **1.00x slower**)  | `95.92 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**) | `8.12 ns` (✅ **1.04x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.33 ns` (✅ **1.00x**) | `9.34 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.61 ns` (✅ **1.00x**) | `4.66 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.52 ns` (✅ **1.00x**) | `4.70 ns` (✅ **1.04x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `77.68 ns` (✅ **1.00x**)  | `33.10 ns` (🚀 **2.35x faster**)    | `33.47 ns` (🚀 **2.32x faster**)     |
| **`serialize_uncompressed`**             | `62.66 ns` (✅ **1.00x**)  | `33.04 ns` (🚀 **1.90x faster**)    | `33.38 ns` (🚀 **1.88x faster**)     |
| **`deserialize_compressed`**             | `192.32 us` (✅ **1.00x**) | `53.40 ns` (🚀 **3601.82x faster**) | `53.42 ns` (🚀 **3600.41x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.70 us` (✅ **1.00x**)  | `53.17 ns` (🚀 **765.46x faster**)  | `54.12 ns` (🚀 **752.08x faster**)   |
| **`deserialize_uncompressed`**           | `151.58 us` (✅ **1.00x**) | `53.02 ns` (🚀 **2859.18x faster**) | `56.15 ns` (🚀 **2699.62x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `170.81 ns` (✅ **1.00x**) | `53.46 ns` (🚀 **3.20x faster**)    | `52.87 ns` (🚀 **3.23x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.62 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.48 us` (✅ **1.00x**) | `33.35 us` (❌ *2.67x slower*)    |
| **`legendre_for_qr`**    | `12.57 us` (✅ **1.00x**) | `11.53 us` (✅ **1.09x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.86 ns` (✅ **1.00x**)  | `4.73 ns` (✅ **1.03x faster**)   |
| **`from_little-endian_bits`** | `72.38 ns` (✅ **1.00x**) | `72.23 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `72.31 ns` (✅ **1.00x**) | `71.94 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `5.03 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.03x faster**)   |
| **`equality`**                | `5.32 ns` (✅ **1.00x**)  | `5.38 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `4.63 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.56 ns` (✅ **1.00x**) | `43.03 ns` (✅ **1.01x slower**)  |
| **`into_bigint`** | `25.99 ns` (✅ **1.00x**) | `25.91 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

