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
|        | `64.94 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `483.46 ns` (✅ **1.00x**) | `10.88 ns` (🚀 **44.42x faster**) | `11.01 ns` (🚀 **43.92x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `508.01 ns` (✅ **1.00x**) | `11.45 ns` (🚀 **44.35x faster**) | `11.35 ns` (🚀 **44.76x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `496.95 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `503.94 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `409.03 ns` (✅ **1.00x**) | `10.70 ns` (🚀 **38.23x faster**) | `6.41 ns` (🚀 **63.84x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `168.36 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `8.09 ns` (✅ **1.00x slower**)   | `8.08 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `45.33 ns` (✅ **1.02x faster**)  | `46.03 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `39.47 ns` (✅ **1.00x slower**)  | `39.37 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.08 us` (✅ **1.02x slower**)   | `6.93 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `64.89 ns` (✅ **1.09x faster**)  | `70.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `100.29 ns` (✅ **1.00x slower**) | `100.18 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `8.30 ns` (✅ **1.00x**)  | `8.28 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.57 ns` (✅ **1.00x**) | `10.52 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.22 ns` (✅ **1.00x**)  | `4.24 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `80.16 ns` (✅ **1.00x**)  | `37.31 ns` (🚀 **2.15x faster**)    | `36.29 ns` (🚀 **2.21x faster**)     |
| **`serialize_uncompressed`**             | `72.78 ns` (✅ **1.00x**)  | `37.68 ns` (🚀 **1.93x faster**)    | `36.85 ns` (🚀 **1.98x faster**)     |
| **`deserialize_compressed`**             | `214.77 us` (✅ **1.00x**) | `59.85 ns` (🚀 **3588.57x faster**) | `56.88 ns` (🚀 **3775.94x faster**)  |
| **`deserialize_compressed_unchecked`**   | `43.11 us` (✅ **1.00x**)  | `58.57 ns` (🚀 **736.05x faster**)  | `58.50 ns` (🚀 **736.92x faster**)   |
| **`deserialize_uncompressed`**           | `171.07 us` (✅ **1.00x**) | `58.99 ns` (🚀 **2899.83x faster**) | `59.32 ns` (🚀 **2883.76x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `187.99 ns` (✅ **1.00x**) | `58.72 ns` (🚀 **3.20x faster**)    | `61.48 ns` (🚀 **3.06x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.71 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.95 us` (✅ **1.00x**) | `36.48 us` (❌ *2.62x slower*)    |
| **`legendre_for_qr`**    | `13.91 us` (✅ **1.00x**) | `12.58 us` (✅ **1.11x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.60 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.02x slower**)   |
| **`from_little-endian_bits`** | `74.08 ns` (✅ **1.00x**) | `72.76 ns` (✅ **1.02x faster**)  |
| **`from_big-endian_bits`**    | `76.56 ns` (✅ **1.00x**) | `76.50 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.63 ns` (✅ **1.00x**)  | `4.59 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `5.05 ns` (✅ **1.00x**)  | `5.23 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.37 ns` (✅ **1.00x**)  | `4.44 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.05 ns` (✅ **1.00x**) | `45.45 ns` (✅ **1.01x slower**)  |
| **`into_bigint`** | `27.20 ns` (✅ **1.00x**) | `27.19 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

