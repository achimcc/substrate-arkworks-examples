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
|        | `58.67 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `387.97 ns` (✅ **1.00x**) | `8.73 ns` (🚀 **44.43x faster**) | `8.63 ns` (🚀 **44.95x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `407.57 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.26x faster**) | `8.78 ns` (🚀 **46.43x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `399.25 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `401.58 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `289.08 ns` (✅ **1.00x**) | `5.87 ns` (🚀 **49.28x faster**) | `5.86 ns` (🚀 **49.29x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.38 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.00x faster**)  | `6.14 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.13 ns` (✅ **1.01x slower**) | `42.74 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.46 ns` (✅ **1.02x faster**) | `36.28 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.03 us` (✅ **1.01x slower**)  | `6.99 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.45 ns` (✅ **1.01x faster**) | `61.90 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.03 ns` (✅ **1.01x faster**) | `89.94 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**) | `8.68 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.78 ns` (✅ **1.00x**) | `4.77 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.37 ns` (✅ **1.00x**)  | `30.95 ns` (🚀 **2.24x faster**)    | `30.90 ns` (🚀 **2.24x faster**)     |
| **`serialize_uncompressed`**             | `57.32 ns` (✅ **1.00x**)  | `31.37 ns` (🚀 **1.83x faster**)    | `31.00 ns` (🚀 **1.85x faster**)     |
| **`deserialize_compressed`**             | `181.20 us` (✅ **1.00x**) | `50.04 ns` (🚀 **3620.85x faster**) | `52.38 ns` (🚀 **3459.32x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.66 us` (✅ **1.00x**)  | `50.15 ns` (🚀 **770.79x faster**)  | `52.46 ns` (🚀 **736.83x faster**)   |
| **`deserialize_uncompressed`**           | `142.50 us` (✅ **1.00x**) | `49.99 ns` (🚀 **2850.61x faster**) | `52.12 ns` (🚀 **2734.05x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `166.55 ns` (✅ **1.00x**) | `50.11 ns` (🚀 **3.32x faster**)    | `52.12 ns` (🚀 **3.20x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.92 us` (✅ **1.00x**) | `31.09 us` (❌ *2.61x slower*)    |
| **`legendre_for_qr`**    | `12.12 us` (✅ **1.00x**) | `10.85 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.26 ns` (✅ **1.00x**) | `48.16 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.17 ns` (✅ **1.00x**) | `48.15 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.90 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.43 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.17 ns` (✅ **1.00x**) | `40.79 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.89 ns` (✅ **1.00x**) | `22.95 ns` (✅ **1.04x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

