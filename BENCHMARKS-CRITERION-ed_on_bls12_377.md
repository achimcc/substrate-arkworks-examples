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
|        | `51.91 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`            | `fq::bigint`                   | `g`                       | `fq`                            | `fr`                             |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `381.22 ns` (✅ **1.00x**) | `8.27 ns` (🚀 **46.09x faster**) | `8.14 ns` (🚀 **46.82x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `398.15 ns` (✅ **1.00x**) | `8.59 ns` (🚀 **46.37x faster**) | `8.69 ns` (🚀 **45.82x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `394.84 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `396.81 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                   | `N/A`                          | `293.19 ns` (✅ **1.00x**) | `9.07 ns` (🚀 **32.31x faster**) | `5.28 ns` (🚀 **55.56x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `123.73 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `5.89 ns` (✅ **1.01x faster**)  | `5.95 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `37.39 ns` (✅ **1.00x slower**) | `37.25 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `31.78 ns` (✅ **1.01x slower**) | `31.55 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `6.21 us` (✅ **1.01x faster**)  | `6.26 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `53.16 ns` (✅ **1.00x slower**) | `52.99 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `79.61 ns` (✅ **1.01x faster**) | `80.57 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**) | `6.54 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `7.86 ns` (✅ **1.00x**) | `7.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `3.84 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `64.82 ns` (✅ **1.00x**)  | `27.92 ns` (🚀 **2.32x faster**)    | `27.94 ns` (🚀 **2.32x faster**)     |
| **`serialize_uncompressed`**             | `53.89 ns` (✅ **1.00x**)  | `27.82 ns` (🚀 **1.94x faster**)    | `28.01 ns` (🚀 **1.92x faster**)     |
| **`deserialize_compressed`**             | `163.22 us` (✅ **1.00x**) | `46.64 ns` (🚀 **3499.25x faster**) | `46.46 ns` (🚀 **3512.93x faster**)  |
| **`deserialize_compressed_unchecked`**   | `34.25 us` (✅ **1.00x**)  | `46.63 ns` (🚀 **734.48x faster**)  | `46.48 ns` (🚀 **736.82x faster**)   |
| **`deserialize_uncompressed`**           | `128.39 us` (✅ **1.00x**) | `46.58 ns` (🚀 **2756.39x faster**) | `46.42 ns` (🚀 **2766.09x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `144.95 ns` (✅ **1.00x**) | `46.58 ns` (🚀 **3.11x faster**)    | `46.42 ns` (🚀 **3.12x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.39 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `10.41 us` (✅ **1.00x**) | `27.62 us` (❌ *2.65x slower*)    |
| **`legendre_for_qr`**    | `10.57 us` (✅ **1.00x**) | `9.54 us` (✅ **1.11x faster**)   |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `3.99 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `60.80 ns` (✅ **1.00x**) | `60.69 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `60.74 ns` (✅ **1.00x**) | `60.79 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.11 ns` (✅ **1.01x slower**)   |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.48 ns` (✅ **1.00x faster**)   |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `3.91 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.05 ns` (✅ **1.00x**) | `35.88 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `21.77 ns` (✅ **1.01x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

