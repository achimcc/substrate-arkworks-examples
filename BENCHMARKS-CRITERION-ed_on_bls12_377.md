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
|        | `58.38 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `386.08 ns` (✅ **1.00x**) | `8.69 ns` (🚀 **44.41x faster**) | `8.64 ns` (🚀 **44.67x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `404.12 ns` (✅ **1.00x**) | `8.78 ns` (🚀 **46.00x faster**) | `8.79 ns` (🚀 **45.95x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `400.57 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `414.50 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `295.37 ns` (✅ **1.00x**) | `5.93 ns` (🚀 **49.84x faster**) | `5.80 ns` (🚀 **50.91x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `136.41 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.14 ns` (✅ **1.01x faster**)  | `6.18 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.84 ns` (✅ **1.02x slower**) | `43.10 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.51 ns` (✅ **1.02x faster**) | `36.13 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.89 us` (✅ **1.02x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `61.57 ns` (✅ **1.01x faster**) | `62.19 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.10 ns` (✅ **1.01x faster**) | `89.74 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.60 ns` (✅ **1.00x**) | `7.60 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `8.71 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.78 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `69.15 ns` (✅ **1.00x**)  | `33.12 ns` (🚀 **2.09x faster**)    | `30.40 ns` (🚀 **2.27x faster**)     |
| **`serialize_uncompressed`**             | `57.71 ns` (✅ **1.00x**)  | `32.83 ns` (✅ **1.76x faster**)    | `32.18 ns` (✅ **1.79x faster**)     |
| **`deserialize_compressed`**             | `181.84 us` (✅ **1.00x**) | `50.97 ns` (🚀 **3567.53x faster**) | `52.15 ns` (🚀 **3486.97x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.52 us` (✅ **1.00x**)  | `51.01 ns` (🚀 **755.00x faster**)  | `52.15 ns` (🚀 **738.55x faster**)   |
| **`deserialize_uncompressed`**           | `143.31 us` (✅ **1.00x**) | `50.92 ns` (🚀 **2814.40x faster**) | `52.07 ns` (🚀 **2752.31x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `168.36 ns` (✅ **1.00x**) | `51.07 ns` (🚀 **3.30x faster**)    | `52.06 ns` (🚀 **3.23x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `11.86 us` (✅ **1.00x**) | `31.12 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `12.15 us` (✅ **1.00x**) | `10.86 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.84 ns` (✅ **1.00x**) | `48.78 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.82 ns` (✅ **1.00x**) | `48.79 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.36 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.57 ns` (✅ **1.00x**) | `40.76 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `22.53 ns` (✅ **1.00x**) | `23.74 ns` (✅ **1.05x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

