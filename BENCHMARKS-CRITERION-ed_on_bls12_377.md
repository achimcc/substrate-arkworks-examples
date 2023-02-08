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
|        | `58.40 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `385.18 ns` (✅ **1.00x**) | `8.73 ns` (🚀 **44.14x faster**) | `8.65 ns` (🚀 **44.55x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `406.00 ns` (✅ **1.00x**) | `8.79 ns` (🚀 **46.20x faster**) | `8.79 ns` (🚀 **46.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.27 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `414.27 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.41 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.08x faster**) | `5.86 ns` (🚀 **50.08x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `137.69 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.17 ns` (✅ **1.00x slower**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `44.13 ns` (✅ **1.02x slower**) | `43.09 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.94 ns` (✅ **1.01x faster**) | `36.15 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.90 us` (✅ **1.01x faster**)  | `6.96 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.62 ns` (✅ **1.00x faster**) | `61.78 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.10 ns` (✅ **1.01x faster**) | `89.80 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.64 ns` (✅ **1.00x**) | `7.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `8.64 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.20 ns` (✅ **1.00x**)  | `30.60 ns` (🚀 **2.26x faster**)    | `30.85 ns` (🚀 **2.24x faster**)     |
| **`serialize_uncompressed`**             | `58.13 ns` (✅ **1.00x**)  | `30.52 ns` (🚀 **1.90x faster**)    | `31.19 ns` (🚀 **1.86x faster**)     |
| **`deserialize_compressed`**             | `182.70 us` (✅ **1.00x**) | `50.65 ns` (🚀 **3606.83x faster**) | `52.70 ns` (🚀 **3466.98x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.49 us` (✅ **1.00x**)  | `50.70 ns` (🚀 **759.18x faster**)  | `52.70 ns` (🚀 **730.48x faster**)   |
| **`deserialize_uncompressed`**           | `144.15 us` (✅ **1.00x**) | `50.60 ns` (🚀 **2848.58x faster**) | `52.40 ns` (🚀 **2751.05x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `168.38 ns` (✅ **1.00x**) | `50.60 ns` (🚀 **3.33x faster**)    | `52.36 ns` (🚀 **3.22x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.30 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.90 us` (✅ **1.00x**) | `31.10 us` (❌ *2.61x slower*)    |
| **`legendre_for_qr`**    | `12.14 us` (✅ **1.00x**) | `10.84 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `49.02 ns` (✅ **1.00x**) | `49.07 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `49.01 ns` (✅ **1.00x**) | `49.01 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)  | `5.41 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.99 ns` (✅ **1.00x**) | `40.88 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.75 ns` (✅ **1.00x**) | `23.74 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

