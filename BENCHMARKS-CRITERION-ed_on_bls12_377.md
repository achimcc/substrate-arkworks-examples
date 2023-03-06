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
|        | `52.61 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `381.68 ns` (✅ **1.00x**) | `8.28 ns` (🚀 **46.11x faster**) | `7.19 ns` (🚀 **53.09x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `397.99 ns` (✅ **1.00x**) | `8.61 ns` (🚀 **46.24x faster**) | `8.59 ns` (🚀 **46.32x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `392.34 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `398.76 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `292.96 ns` (✅ **1.00x**) | `4.66 ns` (🚀 **62.83x faster**) | `5.33 ns` (🚀 **54.99x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `109.26 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x slower**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.30 ns` (✅ **1.00x slower**) | `37.27 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.81 ns` (✅ **1.01x slower**) | `31.56 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.59 us` (✅ **1.06x slower**)  | `6.24 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.02 ns` (✅ **1.01x slower**) | `52.69 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `82.30 ns` (✅ **1.01x faster**) | `83.07 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `5.76 ns` (✅ **1.13x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.43 ns` (✅ **1.00x**) | `3.89 ns` (❌ *1.13x slower*)   | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.59 ns` (✅ **1.00x**)  | `27.95 ns` (🚀 **2.45x faster**)    | `28.21 ns` (🚀 **2.43x faster**)     |
| **`serialize_uncompressed`**             | `55.96 ns` (✅ **1.00x**)  | `24.56 ns` (🚀 **2.28x faster**)    | `28.03 ns` (🚀 **2.00x faster**)     |
| **`deserialize_compressed`**             | `143.66 us` (✅ **1.00x**) | `46.77 ns` (🚀 **3071.85x faster**) | `40.98 ns` (🚀 **3505.52x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.60 us` (✅ **1.00x**)  | `46.77 ns` (🚀 **739.92x faster**)  | `40.98 ns` (🚀 **844.41x faster**)   |
| **`deserialize_uncompressed`**           | `128.11 us` (✅ **1.00x**) | `41.27 ns` (🚀 **3104.50x faster**) | `45.00 ns` (🚀 **2846.69x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `150.20 ns` (✅ **1.00x**) | `41.14 ns` (🚀 **3.65x faster**)    | `45.01 ns` (🚀 **3.34x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.39 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.44 us` (✅ **1.00x**) | `24.35 us` (❌ *2.33x slower*)    |
| **`legendre_for_qr`**    | `10.59 us` (✅ **1.00x**) | `9.56 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.51 ns` (✅ **1.00x**)  | `3.98 ns` (❌ *1.13x slower*)     |
| **`from_little-endian_bits`** | `60.68 ns` (✅ **1.00x**) | `60.69 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `60.67 ns` (✅ **1.00x**) | `60.04 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.08 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `3.97 ns` (✅ **1.13x faster**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `3.91 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.03 ns` (✅ **1.00x**) | `35.90 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.79 ns` (✅ **1.01x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

