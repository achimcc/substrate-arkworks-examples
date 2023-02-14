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
|        | `65.20 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `481.71 ns` (✅ **1.00x**) | `10.90 ns` (🚀 **44.18x faster**) | `10.80 ns` (🚀 **44.59x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `500.44 ns` (✅ **1.00x**) | `11.42 ns` (🚀 **43.80x faster**) | `11.18 ns` (🚀 **44.76x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `496.10 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `501.95 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `399.49 ns` (✅ **1.00x**) | `10.31 ns` (🚀 **38.75x faster**) | `6.52 ns` (🚀 **61.26x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `166.21 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `7.99 ns` (✅ **1.02x faster**)   | `8.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `46.01 ns` (✅ **1.01x slower**)  | `45.52 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `39.51 ns` (✅ **1.03x slower**)  | `38.39 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.07 us` (✅ **1.01x slower**)   | `6.99 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `63.47 ns` (✅ **1.12x faster**)  | `71.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `96.70 ns` (✅ **1.00x faster**)  | `96.86 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.16 ns` (✅ **1.00x**)  | `7.98 ns` (✅ **1.02x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.46 ns` (✅ **1.00x**) | `10.45 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.49 ns` (✅ **1.00x**)  | `4.44 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.21 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `80.35 ns` (✅ **1.00x**)  | `36.82 ns` (🚀 **2.18x faster**)    | `37.02 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `66.79 ns` (✅ **1.00x**)  | `37.01 ns` (🚀 **1.80x faster**)    | `36.94 ns` (🚀 **1.81x faster**)     |
| **`deserialize_compressed`**             | `215.15 us` (✅ **1.00x**) | `57.15 ns` (🚀 **3764.88x faster**) | `57.42 ns` (🚀 **3747.21x faster**)  |
| **`deserialize_compressed_unchecked`**   | `42.56 us` (✅ **1.00x**)  | `58.23 ns` (🚀 **730.98x faster**)  | `56.76 ns` (🚀 **749.83x faster**)   |
| **`deserialize_uncompressed`**           | `168.76 us` (✅ **1.00x**) | `55.93 ns` (🚀 **3017.34x faster**) | `57.25 ns` (🚀 **2947.86x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `180.68 ns` (✅ **1.00x**) | `56.73 ns` (🚀 **3.18x faster**)    | `56.77 ns` (🚀 **3.18x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.71 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.45 us` (✅ **1.00x**) | `34.63 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `13.92 us` (✅ **1.00x**) | `12.21 us` (✅ **1.14x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.50 ns` (✅ **1.00x**)  | `4.52 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `74.73 ns` (✅ **1.00x**) | `76.00 ns` (✅ **1.02x slower**)  |
| **`from_big-endian_bits`**    | `74.97 ns` (✅ **1.00x**) | `74.27 ns` (✅ **1.01x faster**)  |
| **`comparison`**              | `4.55 ns` (✅ **1.00x**)  | `4.57 ns` (✅ **1.01x slower**)   |
| **`equality`**                | `4.89 ns` (✅ **1.00x**)  | `5.00 ns` (✅ **1.02x slower**)   |
| **`is_zero`**                 | `4.32 ns` (✅ **1.00x**)  | `4.33 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.41 ns` (✅ **1.00x**) | `45.10 ns` (✅ **1.04x slower**)  |
| **`into_bigint`** | `26.83 ns` (✅ **1.00x**) | `26.83 ns` (✅ **1.00x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

