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
|        | `52.35 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `382.04 ns` (✅ **1.00x**) | `8.30 ns` (🚀 **46.05x faster**) | `8.17 ns` (🚀 **46.77x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `405.39 ns` (✅ **1.00x**) | `8.60 ns` (🚀 **47.12x faster**) | `8.60 ns` (🚀 **47.11x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `392.36 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `402.18 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.17 ns` (✅ **1.00x**) | `5.28 ns` (🚀 **55.55x faster**) | `8.92 ns` (🚀 **32.87x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `124.22 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.43 ns` (✅ **1.00x slower**) | `37.24 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.60 ns` (✅ **1.02x faster**) | `32.21 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.39 us` (✅ **1.00x faster**)  | `6.40 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.26 ns` (✅ **1.01x slower**) | `52.70 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.60 ns` (✅ **1.02x faster**) | `80.89 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `6.52 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `67.49 ns` (✅ **1.00x**)  | `29.05 ns` (🚀 **2.32x faster**)    | `27.86 ns` (🚀 **2.42x faster**)     |
| **`serialize_uncompressed`**             | `53.72 ns` (✅ **1.00x**)  | `28.98 ns` (🚀 **1.85x faster**)    | `27.84 ns` (🚀 **1.93x faster**)     |
| **`deserialize_compressed`**             | `163.03 us` (✅ **1.00x**) | `45.20 ns` (🚀 **3606.82x faster**) | `46.02 ns` (🚀 **3542.92x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.42 us` (✅ **1.00x**)  | `45.17 ns` (🚀 **761.95x faster**)  | `46.02 ns` (🚀 **747.82x faster**)   |
| **`deserialize_uncompressed`**           | `128.45 us` (✅ **1.00x**) | `45.56 ns` (🚀 **2819.10x faster**) | `45.18 ns` (🚀 **2842.94x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.48 ns` (✅ **1.00x**) | `45.56 ns` (🚀 **3.19x faster**)    | `45.18 ns` (🚀 **3.22x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.39 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.46 us` (✅ **1.00x**) | `27.62 us` (❌ *2.64x slower*)    |
| **`legendre_for_qr`**    | `9.37 us` (✅ **1.00x**)  | `9.55 us` (✅ **1.02x slower**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.07 ns` (✅ **1.00x**) | `60.51 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `60.13 ns` (✅ **1.00x**) | `60.52 ns` (✅ **1.01x slower**)  |
| **`comparison`**              | `3.95 ns` (✅ **1.00x**)  | `3.95 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.94 ns` (✅ **1.00x**) | `35.92 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `22.03 ns` (✅ **1.00x**) | `22.04 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

