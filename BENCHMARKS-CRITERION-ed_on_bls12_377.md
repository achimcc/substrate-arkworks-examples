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
|        | `58.39 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `385.75 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.29x faster**) | `8.66 ns` (🚀 **44.56x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `406.58 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.28x faster**) | `8.80 ns` (🚀 **46.21x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.34 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `415.18 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.14 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.15x faster**) | `5.87 ns` (🚀 **50.12x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `137.24 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `44.08 ns` (✅ **1.02x slower**) | `43.09 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.94 ns` (✅ **1.00x faster**) | `36.12 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.01x faster**)  | `6.95 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.61 ns` (✅ **1.00x faster**) | `61.83 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.16 ns` (✅ **1.01x faster**) | `89.84 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.65 ns` (✅ **1.00x**) | `7.64 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.16 ns` (✅ **1.00x**)  | `30.82 ns` (🚀 **2.24x faster**)    | `30.70 ns` (🚀 **2.25x faster**)     |
| **`serialize_uncompressed`**             | `58.09 ns` (✅ **1.00x**)  | `30.49 ns` (🚀 **1.91x faster**)    | `31.27 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `182.34 us` (✅ **1.00x**) | `52.80 ns` (🚀 **3453.24x faster**) | `53.81 ns` (🚀 **3388.40x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.52 us` (✅ **1.00x**)  | `52.78 ns` (🚀 **729.92x faster**)  | `53.81 ns` (🚀 **715.88x faster**)   |
| **`deserialize_uncompressed`**           | `143.77 us` (✅ **1.00x**) | `52.74 ns` (🚀 **2725.92x faster**) | `53.74 ns` (🚀 **2675.39x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `169.32 ns` (✅ **1.00x**) | `52.75 ns` (🚀 **3.21x faster**)    | `53.73 ns` (🚀 **3.15x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.90 us` (✅ **1.00x**) | `31.13 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.12 us` (✅ **1.00x**) | `10.85 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.98 ns` (✅ **1.00x**) | `49.23 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `48.89 ns` (✅ **1.00x**) | `49.10 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `4.87 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)  | `5.41 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.00 ns` (✅ **1.00x**) | `40.88 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.77 ns` (✅ **1.00x**) | `23.75 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

