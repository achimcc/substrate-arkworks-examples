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
|        | `63.20 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `459.05 ns` (✅ **1.00x**) | `9.94 ns` (🚀 **46.18x faster**)  | `9.78 ns` (🚀 **46.92x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `478.51 ns` (✅ **1.00x**) | `10.34 ns` (🚀 **46.26x faster**) | `10.39 ns` (🚀 **46.04x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `476.10 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `476.82 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                          | `354.73 ns` (✅ **1.00x**) | `10.92 ns` (🚀 **32.48x faster**) | `6.46 ns` (🚀 **54.92x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `148.06 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `7.14 ns` (✅ **1.00x slower**)   | `7.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `44.77 ns` (✅ **1.00x slower**)  | `44.74 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `38.10 ns` (✅ **1.00x slower**)  | `37.95 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.94 us` (✅ **1.06x slower**)   | `7.52 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `63.80 ns` (✅ **1.00x slower**)  | `63.58 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `95.54 ns` (✅ **1.01x faster**)  | `96.72 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.42 ns` (✅ **1.00x**) | `9.41 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**) | `4.72 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.67 ns` (✅ **1.00x**) | `4.50 ns` (✅ **1.04x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `82.91 ns` (✅ **1.00x**)  | `33.63 ns` (🚀 **2.47x faster**)    | `33.80 ns` (🚀 **2.45x faster**)     |
| **`serialize_uncompressed`**             | `64.68 ns` (✅ **1.00x**)  | `33.60 ns` (🚀 **1.93x faster**)    | `33.39 ns` (🚀 **1.94x faster**)     |
| **`deserialize_compressed`**             | `195.95 us` (✅ **1.00x**) | `55.87 ns` (🚀 **3507.38x faster**) | `55.32 ns` (🚀 **3541.94x faster**)  |
| **`deserialize_compressed_unchecked`**   | `41.57 us` (✅ **1.00x**)  | `55.85 ns` (🚀 **744.40x faster**)  | `55.30 ns` (🚀 **751.79x faster**)   |
| **`deserialize_uncompressed`**           | `154.81 us` (✅ **1.00x**) | `55.84 ns` (🚀 **2772.46x faster**) | `53.68 ns` (🚀 **2884.18x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `175.84 ns` (✅ **1.00x**) | `55.52 ns` (🚀 **3.17x faster**)    | `53.68 ns` (🚀 **3.28x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.63 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.49 us` (✅ **1.00x**) | `33.12 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `13.01 us` (✅ **1.00x**) | `11.44 us` (✅ **1.14x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.78 ns` (✅ **1.00x**)  | `4.78 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `73.32 ns` (✅ **1.00x**) | `73.47 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `73.54 ns` (✅ **1.00x**) | `73.48 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.90 ns` (✅ **1.00x**)  | `4.88 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)  | `5.45 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `4.69 ns` (✅ **1.00x**)  | `4.70 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.30 ns` (✅ **1.00x**) | `43.14 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `26.01 ns` (✅ **1.00x**) | `26.15 ns` (✅ **1.01x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

