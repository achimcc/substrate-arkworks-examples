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
|        | `52.66 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `382.31 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **46.18x faster**) | `8.13 ns` (🚀 **47.00x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `398.94 ns` (✅ **1.00x**) | `8.60 ns` (🚀 **46.41x faster**) | `8.63 ns` (🚀 **46.22x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `392.93 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `397.77 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `295.39 ns` (✅ **1.00x**) | `9.07 ns` (🚀 **32.56x faster**) | `5.33 ns` (🚀 **55.41x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.68 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.93 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.29 ns` (✅ **1.00x slower**) | `37.23 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.73 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.60 us` (✅ **1.05x slower**)  | `6.27 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.14 ns` (✅ **1.00x slower**) | `52.93 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `70.20 ns` (✅ **1.15x faster**) | `80.44 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `6.52 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `61.02 ns` (✅ **1.00x**)  | `27.84 ns` (🚀 **2.19x faster**)    | `28.03 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `53.89 ns` (✅ **1.00x**)  | `27.81 ns` (🚀 **1.94x faster**)    | `27.80 ns` (🚀 **1.94x faster**)     |
| **`deserialize_compressed`**             | `144.03 us` (✅ **1.00x**) | `46.59 ns` (🚀 **3091.47x faster**) | `46.44 ns` (🚀 **3101.32x faster**)  |
| **`deserialize_compressed_unchecked`**   | `30.55 us` (✅ **1.00x**)  | `46.58 ns` (🚀 **655.82x faster**)  | `46.43 ns` (🚀 **658.02x faster**)   |
| **`deserialize_uncompressed`**           | `128.89 us` (✅ **1.00x**) | `46.55 ns` (🚀 **2769.00x faster**) | `45.63 ns` (🚀 **2824.47x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `145.57 ns` (✅ **1.00x**) | `140.00 ns` (✅ **1.04x faster**)   | `45.27 ns` (🚀 **3.22x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.39 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `9.18 us` (✅ **1.00x**)  | `27.61 us` (❌ *3.01x slower*)    |
| **`legendre_for_qr`**    | `10.83 us` (✅ **1.00x**) | `9.52 us` (✅ **1.14x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.78 ns` (✅ **1.00x**) | `61.23 ns` (✅ **1.01x slower**)  |
| **`from_big-endian_bits`**    | `53.56 ns` (✅ **1.00x**) | `61.12 ns` (❌ *1.14x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `3.60 ns` (✅ **1.13x faster**)   |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.92 ns` (✅ **1.00x**) | `31.68 ns` (✅ **1.13x faster**)  |
| **`into_bigint`** | `21.76 ns` (✅ **1.00x**) | `21.64 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

