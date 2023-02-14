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
|        | `64.71 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `484.37 ns` (✅ **1.00x**) | `11.18 ns` (🚀 **43.32x faster**) | `11.03 ns` (🚀 **43.90x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `500.76 ns` (✅ **1.00x**) | `11.66 ns` (🚀 **42.94x faster**) | `11.66 ns` (🚀 **42.95x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `498.41 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `494.41 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `405.72 ns` (✅ **1.00x**) | `10.71 ns` (🚀 **37.88x faster**) | `6.64 ns` (🚀 **61.11x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `164.39 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `8.35 ns` (✅ **1.00x slower**)   | `8.34 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `47.68 ns` (✅ **1.00x slower**)  | `47.64 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `41.36 ns` (✅ **1.03x slower**)  | `39.99 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.21 us` (✅ **1.00x faster**)   | `7.24 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `66.23 ns` (✅ **1.10x faster**)  | `72.73 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `100.78 ns` (✅ **1.00x slower**) | `100.29 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `8.11 ns` (✅ **1.00x**)  | `8.26 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.78 ns` (✅ **1.00x**) | `10.83 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.57 ns` (✅ **1.00x**)  | `4.67 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.41 ns` (✅ **1.00x**)  | `4.39 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `81.64 ns` (✅ **1.00x**)  | `38.57 ns` (🚀 **2.12x faster**)    | `40.94 ns` (🚀 **1.99x faster**)     |
| **`serialize_uncompressed`**             | `69.82 ns` (✅ **1.00x**)  | `38.06 ns` (🚀 **1.83x faster**)    | `41.09 ns` (✅ **1.70x faster**)     |
| **`deserialize_compressed`**             | `219.11 us` (✅ **1.00x**) | `60.38 ns` (🚀 **3628.64x faster**) | `58.48 ns` (🚀 **3747.03x faster**)  |
| **`deserialize_compressed_unchecked`**   | `43.10 us` (✅ **1.00x**)  | `61.38 ns` (🚀 **702.14x faster**)  | `58.81 ns` (🚀 **732.84x faster**)   |
| **`deserialize_uncompressed`**           | `170.61 us` (✅ **1.00x**) | `60.29 ns` (🚀 **2829.88x faster**) | `57.55 ns` (🚀 **2964.36x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `180.93 ns` (✅ **1.00x**) | `60.60 ns` (🚀 **2.99x faster**)    | `57.84 ns` (🚀 **3.13x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.73 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `14.04 us` (✅ **1.00x**) | `35.62 us` (❌ *2.54x slower*)    |
| **`legendre_for_qr`**    | `14.32 us` (✅ **1.00x**) | `12.78 us` (✅ **1.12x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.64 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.02x slower**)   |
| **`from_little-endian_bits`** | `76.23 ns` (✅ **1.00x**) | `77.73 ns` (✅ **1.02x slower**)  |
| **`from_big-endian_bits`**    | `77.67 ns` (✅ **1.00x**) | `77.56 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.65 ns` (✅ **1.00x**)  | `4.78 ns` (✅ **1.03x slower**)   |
| **`equality`**                | `5.10 ns` (✅ **1.00x**)  | `5.14 ns` (✅ **1.01x slower**)   |
| **`is_zero`**                 | `4.44 ns` (✅ **1.00x**)  | `4.47 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `46.31 ns` (✅ **1.00x**) | `46.25 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `27.80 ns` (✅ **1.00x**) | `28.10 ns` (✅ **1.01x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

