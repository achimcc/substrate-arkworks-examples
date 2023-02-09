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
|        | `66.81 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `475.87 ns` (✅ **1.00x**) | `11.00 ns` (🚀 **43.24x faster**) | `10.49 ns` (🚀 **45.38x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `499.00 ns` (✅ **1.00x**) | `11.22 ns` (🚀 **44.49x faster**) | `11.05 ns` (🚀 **45.14x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `488.63 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `499.79 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `369.73 ns` (✅ **1.00x**) | `6.62 ns` (🚀 **55.81x faster**)  | `6.67 ns` (🚀 **55.43x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `154.33 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `7.45 ns` (✅ **1.00x slower**)   | `7.43 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `46.68 ns` (✅ **1.01x slower**)  | `46.35 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `39.69 ns` (✅ **1.01x slower**)  | `39.46 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.74 us` (✅ **1.01x faster**)   | `7.83 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `66.64 ns` (✅ **1.00x slower**)  | `66.44 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `99.75 ns` (✅ **1.02x faster**)  | `101.37 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `8.16 ns` (✅ **1.00x**)  | `8.50 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.14 ns` (✅ **1.00x**) | `10.09 ns` (✅ **1.01x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.87 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.68 ns` (✅ **1.00x**)  | `4.68 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `81.67 ns` (✅ **1.00x**)  | `35.72 ns` (🚀 **2.29x faster**)    | `35.07 ns` (🚀 **2.33x faster**)     |
| **`serialize_uncompressed`**             | `69.79 ns` (✅ **1.00x**)  | `34.77 ns` (🚀 **2.01x faster**)    | `35.03 ns` (🚀 **1.99x faster**)     |
| **`deserialize_compressed`**             | `214.53 us` (✅ **1.00x**) | `58.60 ns` (🚀 **3661.23x faster**) | `57.62 ns` (🚀 **3722.89x faster**)  |
| **`deserialize_compressed_unchecked`**   | `42.78 us` (✅ **1.00x**)  | `60.92 ns` (🚀 **702.28x faster**)  | `57.62 ns` (🚀 **742.46x faster**)   |
| **`deserialize_uncompressed`**           | `166.86 us` (✅ **1.00x**) | `58.89 ns` (🚀 **2833.65x faster**) | `55.54 ns` (🚀 **3004.39x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `186.66 ns` (✅ **1.00x**) | `58.65 ns` (🚀 **3.18x faster**)    | `55.85 ns` (🚀 **3.34x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.72 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.50 us` (✅ **1.00x**) | `34.80 us` (❌ *2.58x slower*)    |
| **`legendre_for_qr`**    | `13.31 us` (✅ **1.00x**) | `11.95 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.99 ns` (✅ **1.00x**)  | `4.98 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `76.01 ns` (✅ **1.00x**) | `79.37 ns` (✅ **1.04x slower**)  |
| **`from_big-endian_bits`**    | `78.97 ns` (✅ **1.00x**) | `76.44 ns` (✅ **1.03x faster**)  |
| **`comparison`**              | `5.31 ns` (✅ **1.00x**)  | `5.32 ns` (✅ **1.00x slower**)   |
| **`equality`**                | `5.78 ns` (✅ **1.00x**)  | `5.60 ns` (✅ **1.03x faster**)   |
| **`is_zero`**                 | `4.88 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.03x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.05 ns` (✅ **1.00x**) | `44.93 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `27.08 ns` (✅ **1.00x**) | `26.78 ns` (✅ **1.01x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

