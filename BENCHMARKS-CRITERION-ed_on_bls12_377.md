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
|        | `52.65 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `380.18 ns` (✅ **1.00x**) | `8.30 ns` (🚀 **45.83x faster**) | `8.14 ns` (🚀 **46.71x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `396.34 ns` (✅ **1.00x**) | `8.69 ns` (🚀 **45.59x faster**) | `8.61 ns` (🚀 **46.02x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `391.84 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `398.36 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `292.18 ns` (✅ **1.00x**) | `9.08 ns` (🚀 **32.19x faster**) | `5.29 ns` (🚀 **55.21x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.58 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.95 ns` (✅ **1.00x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.31 ns` (✅ **1.00x slower**) | `37.23 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.81 ns` (✅ **1.01x slower**) | `31.54 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.60 us` (✅ **1.04x slower**)  | `6.35 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.08 ns` (✅ **1.01x slower**) | `52.65 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `90.32 ns` (✅ **1.00x faster**) | `90.52 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**) | `6.52 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `3.89 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.00 ns` (✅ **1.00x**)  | `27.84 ns` (🚀 **2.44x faster**)    | `28.15 ns` (🚀 **2.42x faster**)     |
| **`serialize_uncompressed`**             | `55.95 ns` (✅ **1.00x**)  | `27.81 ns` (🚀 **2.01x faster**)    | `27.99 ns` (🚀 **2.00x faster**)     |
| **`deserialize_compressed`**             | `163.64 us` (✅ **1.00x**) | `46.63 ns` (🚀 **3509.11x faster**) | `46.13 ns` (🚀 **3547.33x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.62 us` (✅ **1.00x**)  | `46.65 ns` (🚀 **742.10x faster**)  | `46.41 ns` (🚀 **745.89x faster**)   |
| **`deserialize_uncompressed`**           | `128.80 us` (✅ **1.00x**) | `46.34 ns` (🚀 **2779.40x faster**) | `46.09 ns` (🚀 **2794.53x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `149.69 ns` (✅ **1.00x**) | `46.36 ns` (🚀 **3.23x faster**)    | `46.09 ns` (🚀 **3.25x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.23 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.42 us` (✅ **1.00x**) | `27.58 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.58 us` (✅ **1.00x**) | `9.53 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `3.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `60.64 ns` (✅ **1.00x**) | `62.13 ns` (✅ **1.02x slower**)  |
| **`from_big-endian_bits`**    | `60.71 ns` (✅ **1.00x**) | `62.13 ns` (✅ **1.02x slower**)  |
| **`comparison`**              | `4.06 ns` (✅ **1.00x**)  | `4.04 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.50 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.05 ns` (✅ **1.00x**) | `35.88 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.72 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

