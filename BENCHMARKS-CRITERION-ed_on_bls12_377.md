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
|        | `59.07 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `386.75 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.33x faster**) | `8.68 ns` (🚀 **44.53x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `405.00 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **45.96x faster**) | `8.81 ns` (🚀 **45.95x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `401.73 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `402.19 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `292.70 ns` (✅ **1.00x**) | `5.90 ns` (🚀 **49.59x faster**) | `5.85 ns` (🚀 **50.02x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `139.38 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `6.16 ns` (✅ **1.00x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `43.97 ns` (✅ **1.02x slower**) | `43.01 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `35.73 ns` (✅ **1.02x slower**) | `34.91 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `7.17 us` (✅ **1.01x faster**)  | `7.26 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `62.47 ns` (✅ **1.05x slower**) | `59.64 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `89.20 ns` (✅ **1.01x faster**) | `89.91 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `7.62 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**) | `8.65 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.51 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `75.98 ns` (✅ **1.00x**)  | `32.15 ns` (🚀 **2.36x faster**)    | `31.46 ns` (🚀 **2.42x faster**)     |
| **`serialize_uncompressed`**             | `93.38 ns` (✅ **1.00x**)  | `32.38 ns` (🚀 **2.88x faster**)    | `31.67 ns` (🚀 **2.95x faster**)     |
| **`deserialize_compressed`**             | `184.46 us` (✅ **1.00x**) | `50.02 ns` (🚀 **3688.03x faster**) | `51.43 ns` (🚀 **3587.00x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.86 us` (✅ **1.00x**)  | `50.10 ns` (🚀 **775.62x faster**)  | `51.40 ns` (🚀 **755.95x faster**)   |
| **`deserialize_uncompressed`**           | `145.60 us` (✅ **1.00x**) | `50.02 ns` (🚀 **2910.67x faster**) | `51.37 ns` (🚀 **2834.35x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `168.13 ns` (✅ **1.00x**) | `50.21 ns` (🚀 **3.35x faster**)    | `51.39 ns` (🚀 **3.27x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.15 us` (✅ **1.00x**) | `31.28 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.29 us` (✅ **1.00x**) | `10.96 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `49.10 ns` (✅ **1.00x**) | `48.76 ns` (✅ **1.01x faster**)  |
| **`from_big-endian_bits`**    | `49.12 ns` (✅ **1.00x**) | `48.87 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)  | `4.91 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)  | `5.39 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.83 ns` (✅ **1.00x**) | `40.79 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `22.76 ns` (✅ **1.00x**) | `22.48 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

