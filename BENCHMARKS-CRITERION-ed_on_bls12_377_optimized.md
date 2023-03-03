# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_edonbls12_381_optimized](#sample_edonbls12_381_optimized)
    - [arithmetic_for_edonbls12_381_optimized](#arithmetic_for_edonbls12_381_optimized)
    - [serialization_for_edonbls12_381_optimized](#serialization_for_edonbls12_381_optimized)
    - [msm_for_edonbls12_381_optimized](#msm_for_edonbls12_381_optimized)
    - [squareroot_for_edonbls12_381_optimized](#squareroot_for_edonbls12_381_optimized)
    - [bitwise_operations_for_edonbls12_381_optimized](#bitwise_operations_for_edonbls12_381_optimized)
    - [conversions_for_edonbls12_381_optimized](#conversions_for_edonbls12_381_optimized)

## Benchmark Results

### sample_edonbls12_381_optimized

|        | `goptimized_elements`           |
|:-------|:------------------------------- |
|        | `66.94 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `386.23 ns` (✅ **1.00x**) | `8.70 ns` (🚀 **44.37x faster**) | `8.64 ns` (🚀 **44.72x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `405.35 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.04x faster**) | `8.79 ns` (🚀 **46.13x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `398.77 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `408.22 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.62 ns` (✅ **1.00x**) | `5.85 ns` (🚀 **50.54x faster**) | `5.81 ns` (🚀 **50.89x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.49 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.19 ns` (✅ **1.01x slower**)  | `6.15 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.81 ns` (✅ **1.00x faster**) | `42.85 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.89 ns` (✅ **1.02x slower**) | `35.22 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `7.05 us` (✅ **1.00x slower**)  | `7.03 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.58 ns` (✅ **1.01x faster**) | `61.97 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `89.06 ns` (✅ **1.01x faster**) | `89.81 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `7.63 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.67 ns` (✅ **1.00x**)        | `8.68 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.19 ns` (✅ **1.00x**)  | `31.20 ns` (🚀 **2.19x faster**)    | `31.41 ns` (🚀 **2.17x faster**)     |
| **`serialize_uncompressed`**             | `57.47 ns` (✅ **1.00x**)  | `32.25 ns` (✅ **1.78x faster**)    | `32.39 ns` (✅ **1.77x faster**)     |
| **`deserialize_compressed`**             | `182.56 us` (✅ **1.00x**) | `52.03 ns` (🚀 **3508.49x faster**) | `52.83 ns` (🚀 **3455.72x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.61 us` (✅ **1.00x**)  | `51.99 ns` (🚀 **742.62x faster**)  | `52.84 ns` (🚀 **730.73x faster**)   |
| **`deserialize_uncompressed`**           | `143.81 us` (✅ **1.00x**) | `52.09 ns` (🚀 **2760.58x faster**) | `52.78 ns` (🚀 **2724.51x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `164.03 ns` (✅ **1.00x**) | `51.73 ns` (🚀 **3.17x faster**)    | `52.77 ns` (🚀 **3.11x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.35 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.12 us` (✅ **1.00x**) | `31.09 us` (❌ *2.56x slower*)    |
| **`legendre_for_qr`**    | `12.29 us` (✅ **1.00x**) | `10.88 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x faster**)   |
| **`from_little-endian_bits`** | `48.02 ns` (✅ **1.00x**)       | `48.10 ns` (✅ **1.00x slower**)  |
| **`from_big-endian_bits`**    | `48.03 ns` (✅ **1.00x**)       | `48.03 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.89 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.43 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `40.60 ns` (✅ **1.01x faster**)  |
| **`into_bigint`** | `23.78 ns` (✅ **1.00x**) | `23.75 ns` (✅ **1.00x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

