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
|        | `58.44 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `385.30 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.31x faster**) | `8.65 ns` (🚀 **44.56x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `404.20 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.01x faster**) | `8.79 ns` (🚀 **45.99x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.33 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `413.86 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.96 ns` (✅ **1.00x**) | `5.92 ns` (🚀 **49.81x faster**) | `5.80 ns` (🚀 **50.84x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.89 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.14 ns` (✅ **1.01x faster**)  | `6.17 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.65 ns` (✅ **1.01x slower**) | `43.11 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.71 ns` (✅ **1.01x faster**) | `36.16 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.01x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.61 ns` (✅ **1.01x faster**) | `62.16 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.14 ns` (✅ **1.01x faster**) | `89.77 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.72 ns` (✅ **1.00x**) | `8.72 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.79 ns` (✅ **1.00x**) | `4.78 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.24 ns` (✅ **1.00x**)  | `30.98 ns` (🚀 **2.23x faster**)    | `30.36 ns` (🚀 **2.28x faster**)     |
| **`serialize_uncompressed`**             | `57.80 ns` (✅ **1.00x**)  | `30.61 ns` (🚀 **1.89x faster**)    | `32.14 ns` (✅ **1.80x faster**)     |
| **`deserialize_compressed`**             | `182.24 us` (✅ **1.00x**) | `50.99 ns` (🚀 **3574.32x faster**) | `52.26 ns` (🚀 **3487.13x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.50 us` (✅ **1.00x**)  | `50.47 ns` (🚀 **762.83x faster**)  | `52.15 ns` (🚀 **738.31x faster**)   |
| **`deserialize_uncompressed`**           | `143.71 us` (✅ **1.00x**) | `50.51 ns` (🚀 **2845.41x faster**) | `52.04 ns` (🚀 **2761.68x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `167.13 ns` (✅ **1.00x**) | `50.96 ns` (🚀 **3.28x faster**)    | `52.05 ns` (🚀 **3.21x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.87 us` (✅ **1.00x**) | `31.13 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.13 us` (✅ **1.00x**) | `10.87 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.59 ns` (✅ **1.00x**) | `48.75 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.77 ns` (✅ **1.00x**) | `48.72 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.37 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.92 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.57 ns` (✅ **1.00x**) | `40.78 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `22.58 ns` (✅ **1.00x**) | `23.89 ns` (✅ **1.06x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

