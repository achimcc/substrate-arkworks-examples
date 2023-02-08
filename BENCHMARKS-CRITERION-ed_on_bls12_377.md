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
| **`addition`**                        | `N/A`                   | `N/A`                          | `385.83 ns` (✅ **1.00x**) | `8.71 ns` (🚀 **44.28x faster**) | `8.64 ns` (🚀 **44.63x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `406.18 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.12x faster**) | `8.79 ns` (🚀 **46.21x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.62 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `415.19 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `294.73 ns` (✅ **1.00x**) | `5.86 ns` (🚀 **50.33x faster**) | `5.86 ns` (🚀 **50.28x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `137.30 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.18 ns` (✅ **1.00x slower**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.78 ns` (✅ **1.02x slower**) | `43.11 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.92 ns` (✅ **1.01x faster**) | `36.17 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.91 us` (✅ **1.01x faster**)  | `6.97 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.66 ns` (✅ **1.00x faster**) | `61.79 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.13 ns` (✅ **1.01x faster**) | `89.81 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.64 ns` (✅ **1.00x**) | `7.64 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.12 ns` (✅ **1.00x**)  | `30.73 ns` (🚀 **2.25x faster**)    | `30.47 ns` (🚀 **2.27x faster**)     |
| **`serialize_uncompressed`**             | `57.93 ns` (✅ **1.00x**)  | `30.43 ns` (🚀 **1.90x faster**)    | `31.81 ns` (🚀 **1.82x faster**)     |
| **`deserialize_compressed`**             | `182.36 us` (✅ **1.00x**) | `50.62 ns` (🚀 **3602.31x faster**) | `52.41 ns` (🚀 **3479.61x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.52 us` (✅ **1.00x**)  | `50.49 ns` (🚀 **762.96x faster**)  | `52.42 ns` (🚀 **734.89x faster**)   |
| **`deserialize_uncompressed`**           | `143.76 us` (✅ **1.00x**) | `50.36 ns` (🚀 **2854.41x faster**) | `52.32 ns` (🚀 **2747.64x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `169.30 ns` (✅ **1.00x**) | `50.53 ns` (🚀 **3.35x faster**)    | `52.32 ns` (🚀 **3.24x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.88 us` (✅ **1.00x**) | `31.10 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.15 us` (✅ **1.00x**) | `10.84 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `49.13 ns` (✅ **1.00x**) | `48.98 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.96 ns` (✅ **1.00x**) | `48.81 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `4.88 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.41 ns` (✅ **1.00x**)  | `5.41 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.12 ns` (✅ **1.00x**) | `40.89 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.76 ns` (✅ **1.00x**) | `23.74 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

