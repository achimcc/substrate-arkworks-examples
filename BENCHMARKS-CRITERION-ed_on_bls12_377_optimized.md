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
|        | `73.88 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `goptimized`              | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `487.30 ns` (✅ **1.00x**) | `11.09 ns` (🚀 **43.96x faster**) | `11.11 ns` (🚀 **43.87x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `495.86 ns` (✅ **1.00x**) | `11.67 ns` (🚀 **42.49x faster**) | `11.66 ns` (🚀 **42.52x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `485.74 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `492.19 ns` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `407.41 ns` (✅ **1.00x**) | `6.56 ns` (🚀 **62.06x faster**)  | `6.39 ns` (🚀 **63.71x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `167.56 us` (✅ **1.00x**) | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                     | `8.18 ns` (✅ **1.01x slower**)   | `8.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                     | `47.22 ns` (✅ **1.01x slower**)  | `46.93 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                     | `39.59 ns` (✅ **1.00x faster**)  | `39.63 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                     | `7.07 us` (✅ **1.01x slower**)   | `7.02 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                     | `63.93 ns` (✅ **1.13x faster**)  | `72.49 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                     | `97.89 ns` (✅ **1.01x faster**)  | `98.69 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.08 ns` (✅ **1.00x**)        | `7.82 ns` (✅ **1.03x faster**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `10.47 ns` (✅ **1.00x**)       | `10.43 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.50 ns` (✅ **1.00x**)        | `4.59 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.19 ns` (✅ **1.00x**)        | `4.23 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                            | `N/A`                             |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `80.63 ns` (✅ **1.00x**)  | `37.34 ns` (🚀 **2.16x faster**)    | `37.76 ns` (🚀 **2.14x faster**)     |
| **`serialize_uncompressed`**             | `69.53 ns` (✅ **1.00x**)  | `37.33 ns` (🚀 **1.86x faster**)    | `37.73 ns` (🚀 **1.84x faster**)     |
| **`deserialize_compressed`**             | `215.64 us` (✅ **1.00x**) | `59.17 ns` (🚀 **3644.55x faster**) | `58.80 ns` (🚀 **3667.16x faster**)  |
| **`deserialize_compressed_unchecked`**   | `43.45 us` (✅ **1.00x**)  | `59.64 ns` (🚀 **728.54x faster**)  | `59.52 ns` (🚀 **729.90x faster**)   |
| **`deserialize_uncompressed`**           | `173.36 us` (✅ **1.00x**) | `58.90 ns` (🚀 **2943.30x faster**) | `57.29 ns` (🚀 **3025.91x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `183.00 ns` (✅ **1.00x**) | `59.89 ns` (🚀 **3.06x faster**)    | `56.47 ns` (🚀 **3.24x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.77 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `13.74 us` (✅ **1.00x**) | `34.33 us` (❌ *2.50x slower*)    |
| **`legendre_for_qr`**    | `14.55 us` (✅ **1.00x**) | `12.43 us` (✅ **1.17x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.47 ns` (✅ **1.00x**)        | `4.41 ns` (✅ **1.01x faster**)   |
| **`from_little-endian_bits`** | `71.19 ns` (✅ **1.00x**)       | `75.21 ns` (✅ **1.06x slower**)  |
| **`from_big-endian_bits`**    | `73.64 ns` (✅ **1.00x**)       | `73.80 ns` (✅ **1.00x slower**)  |
| **`comparison`**              | `4.40 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.03x slower**)   |
| **`equality`**                | `5.05 ns` (✅ **1.00x**)        | `4.97 ns` (✅ **1.02x faster**)   |
| **`is_zero`**                 | `4.34 ns` (✅ **1.00x**)        | `4.42 ns` (✅ **1.02x slower**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.37 ns` (✅ **1.00x**) | `46.05 ns` (✅ **1.04x slower**)  |
| **`into_bigint`** | `27.27 ns` (✅ **1.00x**) | `27.53 ns` (✅ **1.01x slower**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

