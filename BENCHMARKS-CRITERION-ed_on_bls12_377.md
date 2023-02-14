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
|        | `63.08 us` (✅ **1.00x**)  |

### arithmetic_for_edonbls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g`                       | `fq`                             | `fr`                              |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `455.12 ns` (✅ **1.00x**) | `10.50 ns` (🚀 **43.35x faster**) | `10.27 ns` (🚀 **44.32x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `483.72 ns` (✅ **1.00x**) | `10.99 ns` (🚀 **44.01x faster**) | `10.96 ns` (🚀 **44.12x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `489.04 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `497.22 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                    | `N/A`                           | `366.26 ns` (✅ **1.00x**) | `10.27 ns` (🚀 **35.66x faster**) | `6.18 ns` (🚀 **59.27x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `153.80 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `7.74 ns` (✅ **1.02x slower**)   | `7.56 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `47.15 ns` (✅ **1.01x faster**)  | `47.69 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `38.33 ns` (✅ **1.00x faster**)  | `38.34 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `7.33 us` (✅ **1.07x slower**)   | `6.87 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `63.31 ns` (✅ **1.14x faster**)  | `72.32 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `100.13 ns` (✅ **1.01x slower**) | `98.89 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.21 ns` (✅ **1.00x**)  | `8.24 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.47 ns` (✅ **1.00x**) | `10.29 ns` (✅ **1.02x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.48 ns` (✅ **1.00x**)  | `4.52 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.52 ns` (✅ **1.00x**)  | `4.47 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_377

|                                          | `g`                       | `fr`                               | `fq`                                |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `81.52 ns` (✅ **1.00x**)  | `35.83 ns` (🚀 **2.28x faster**)    | `35.66 ns` (🚀 **2.29x faster**)     |
| **`serialize_uncompressed`**             | `66.31 ns` (✅ **1.00x**)  | `37.68 ns` (✅ **1.76x faster**)    | `35.23 ns` (🚀 **1.88x faster**)     |
| **`deserialize_compressed`**             | `201.52 us` (✅ **1.00x**) | `58.85 ns` (🚀 **3424.13x faster**) | `57.74 ns` (🚀 **3490.11x faster**)  |
| **`deserialize_compressed_unchecked`**   | `41.69 us` (✅ **1.00x**)  | `58.13 ns` (🚀 **717.29x faster**)  | `58.81 ns` (🚀 **708.96x faster**)   |
| **`deserialize_uncompressed`**           | `156.74 us` (✅ **1.00x**) | `58.08 ns` (🚀 **2698.64x faster**) | `56.83 ns` (🚀 **2757.93x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `183.50 ns` (✅ **1.00x**) | `58.34 ns` (🚀 **3.15x faster**)    | `57.96 ns` (🚀 **3.17x faster**)     |

### msm_for_edonbls12_377

|        | `g`                     |
|:-------|:----------------------- |
|        | `1.75 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_377

|                          | `fr`                     | `fq`                             |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.44 us` (✅ **1.00x**) | `34.37 us` (❌ *2.56x slower*)    |
| **`legendre_for_qr`**    | `13.91 us` (✅ **1.00x**) | `12.04 us` (✅ **1.16x faster**)  |

### bitwise_operations_for_edonbls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.74 ns` (✅ **1.00x**)  | `4.75 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `77.88 ns` (✅ **1.00x**) | `84.67 ns` (✅ **1.09x slower**)  |
| **`from_big-endian_bits`**    | `77.29 ns` (✅ **1.00x**) | `84.09 ns` (✅ **1.09x slower**)  |
| **`comparison`**              | `4.90 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.01x faster**)   |
| **`equality`**                | `5.23 ns` (✅ **1.00x**)  | `5.10 ns` (✅ **1.03x faster**)   |
| **`is_zero`**                 | `4.56 ns` (✅ **1.00x**)  | `4.61 ns` (✅ **1.01x slower**)   |

### conversions_for_edonbls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.38 ns` (✅ **1.00x**) | `43.43 ns` (✅ **1.02x faster**)  |
| **`into_bigint`** | `28.16 ns` (✅ **1.00x**) | `26.87 ns` (✅ **1.05x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

