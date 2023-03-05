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
|        | `62.11 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `460.75 ns` (✅ **1.00x**) | `9.76 ns` (🚀 **47.22x faster**)  | `9.64 ns` (🚀 **47.80x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `477.87 ns` (✅ **1.00x**) | `10.23 ns` (🚀 **46.72x faster**) | `10.25 ns` (🚀 **46.61x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `470.55 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `476.45 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                          | `352.95 ns` (✅ **1.00x**) | `6.25 ns` (🚀 **56.49x faster**)  | `6.24 ns` (🚀 **56.54x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `148.49 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `7.06 ns` (✅ **1.00x faster**)   | `7.09 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `44.09 ns` (✅ **1.00x faster**)  | `44.29 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `37.63 ns` (✅ **1.01x slower**)  | `37.34 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.35 us` (✅ **1.01x faster**)   | `7.40 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `63.06 ns` (✅ **1.01x slower**)  | `62.59 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `93.71 ns` (✅ **1.02x faster**)  | `95.37 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.78 ns` (✅ **1.00x**) | `7.68 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.26 ns` (✅ **1.00x**) | `9.34 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.61 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.42 ns` (✅ **1.00x**) | `4.44 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `77.10 ns` (✅ **1.00x**)  | `33.02 ns` (🚀 **2.33x faster**)    | `33.29 ns` (🚀 **2.32x faster**)     |
| **`serialize_uncompressed`**             | `62.65 ns` (✅ **1.00x**)  | `33.26 ns` (🚀 **1.88x faster**)    | `33.27 ns` (🚀 **1.88x faster**)     |
| **`deserialize_compressed`**             | `194.37 us` (✅ **1.00x**) | `53.77 ns` (🚀 **3614.53x faster**) | `54.57 ns` (🚀 **3561.77x faster**)  |
| **`deserialize_compressed_unchecked`**   | `40.60 us` (✅ **1.00x**)  | `53.31 ns` (🚀 **761.48x faster**)  | `54.30 ns` (🚀 **747.60x faster**)   |
| **`deserialize_uncompressed`**           | `153.52 us` (✅ **1.00x**) | `54.40 ns` (🚀 **2822.31x faster**) | `52.75 ns` (🚀 **2910.26x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `172.03 ns` (✅ **1.00x**) | `54.35 ns` (🚀 **3.17x faster**)    | `52.95 ns` (🚀 **3.25x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.61 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.41 us` (✅ **1.00x**) | `32.60 us` (❌ *2.63x slower*)    |
| **`legendre_for_qr`**    | `12.54 us` (✅ **1.00x**) | `11.18 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.76 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.01x faster**)   |
| **`from_little-endian_bits`** | `72.26 ns` (✅ **1.00x**) | `71.55 ns` (✅ **1.01x faster**)  |
| **`from_big-endian_bits`**    | `72.41 ns` (✅ **1.00x**) | `71.48 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `4.84 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.35 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.59 ns` (✅ **1.00x**)  | `4.60 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.15 ns` (✅ **1.00x**) | `42.99 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `26.03 ns` (✅ **1.00x**) | `25.79 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

