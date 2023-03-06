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
|        | `65.91 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `483.68 ns` (✅ **1.00x**) | `11.25 ns` (🚀 **43.00x faster**) | `10.93 ns` (🚀 **44.26x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `499.58 ns` (✅ **1.00x**) | `11.56 ns` (🚀 **43.23x faster**) | `11.44 ns` (🚀 **43.66x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `497.32 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `511.63 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `412.19 ns` (✅ **1.00x**) | `10.81 ns` (🚀 **38.13x faster**) | `6.44 ns` (🚀 **64.05x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `167.18 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `8.12 ns` (✅ **1.00x slower**)   | `8.12 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `46.27 ns` (✅ **1.00x slower**)  | `46.15 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `40.35 ns` (✅ **1.02x slower**)  | `39.52 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.14 us` (✅ **1.02x slower**)   | `7.03 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `65.26 ns` (✅ **1.12x faster**)  | `73.19 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `99.56 ns` (✅ **1.03x faster**)  | `102.84 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.99 ns` (✅ **1.00x**)  | `8.04 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.57 ns` (✅ **1.00x**) | `10.71 ns` (✅ **1.01x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.60 ns` (✅ **1.00x**)  | `4.62 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.30 ns` (✅ **1.00x**)  | `4.37 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `80.15 ns` (✅ **1.00x**)  | `37.54 ns` (🚀 **2.14x faster**)    | `36.83 ns` (🚀 **2.18x faster**)     |
| **`serialize_uncompressed`**             | `73.94 ns` (✅ **1.00x**)  | `37.21 ns` (🚀 **1.99x faster**)    | `37.25 ns` (🚀 **1.98x faster**)     |
| **`deserialize_compressed`**             | `216.13 us` (✅ **1.00x**) | `58.31 ns` (🚀 **3706.38x faster**) | `57.63 ns` (🚀 **3750.00x faster**)  |
| **`deserialize_compressed_unchecked`**   | `42.45 us` (✅ **1.00x**)  | `58.70 ns` (🚀 **723.18x faster**)  | `58.42 ns` (🚀 **726.64x faster**)   |
| **`deserialize_uncompressed`**           | `170.97 us` (✅ **1.00x**) | `58.05 ns` (🚀 **2945.13x faster**) | `59.88 ns` (🚀 **2854.98x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `187.95 ns` (✅ **1.00x**) | `58.30 ns` (🚀 **3.22x faster**)    | `59.44 ns` (🚀 **3.16x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.66 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.49 us` (✅ **1.00x**) | `35.45 us` (❌ *2.63x slower*)    |
| **`legendre_for_qr`**    | `13.73 us` (✅ **1.00x**) | `12.63 us` (✅ **1.09x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.57 ns` (✅ **1.00x**)  | `4.53 ns` (✅ **1.01x faster**)   |
| **`from_little-endian_bits`** | `74.17 ns` (✅ **1.00x**) | `74.31 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `74.92 ns` (✅ **1.00x**) | `74.20 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `4.66 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.14 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `4.48 ns` (✅ **1.00x**)  | `4.35 ns` (✅ **1.03x faster**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.55 ns` (✅ **1.00x**) | `43.96 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `27.59 ns` (✅ **1.00x**) | `27.03 ns` (✅ **1.02x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

