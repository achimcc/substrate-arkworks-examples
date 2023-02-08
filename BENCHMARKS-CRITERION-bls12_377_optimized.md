# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377_optimized](#sample_bls12_377_optimized)
    - [arithmetic_for_bls12_377_optimized](#arithmetic_for_bls12_377_optimized)
    - [serialization_for_bls12_377_optimized](#serialization_for_bls12_377_optimized)
    - [msm_for_bls12_377_optimized](#msm_for_bls12_377_optimized)
    - [squareroot_for_bls12_377_optimized](#squareroot_for_bls12_377_optimized)
    - [bitwise_operations_for_bls12_377_optimized](#bitwise_operations_for_bls12_377_optimized)
    - [conversions_for_bls12_377_optimized](#conversions_for_bls12_377_optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `211.65 us` (✅ **1.00x**)                 | `2.06 ms` (❌ *9.71x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.54 us` (❌ *3.65x slower*)     | `23.32 ns` (🚀 **53.43x faster**) | `195.28 ns` (🚀 **6.38x faster**)  | `12.54 ns` (🚀 **99.35x faster**)  | `8.71 ns` (🚀 **143.11x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.60 us` (❌ *3.55x slower*)     | `23.32 ns` (🚀 **55.55x faster**) | `162.16 ns` (🚀 **7.99x faster**)  | `12.76 ns` (🚀 **101.50x faster**) | `8.83 ns` (🚀 **146.80x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `909.14 ns` (✅ **1.00x**)        | `3.29 us` (❌ *3.62x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `937.24 ns` (✅ **1.00x**)        | `3.33 us` (❌ *3.55x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `597.06 ns` (✅ **1.00x**)        | `2.23 us` (❌ *3.74x slower*)     | `12.27 ns` (🚀 **48.68x faster**) | `67.70 ns` (🚀 **8.82x faster**)   | `7.15 ns` (🚀 **83.53x faster**)   | `5.85 ns` (🚀 **101.98x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.47 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.48x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.17 ns` (❌ *3.75x slower*)    | `103.48 ns` (❌ *16.76x slower*)   | `18.90 ns` (❌ *3.06x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `268.61 ns` (❌ *5.94x slower*)   | `7.14 us` (❌ *157.81x slower*)    | `76.49 ns` (❌ *1.69x slower*)     | `45.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `243.88 ns` (❌ *6.78x slower*)   | `5.03 us` (❌ *139.79x slower*)    | `66.49 ns` (❌ *1.85x slower*)     | `35.96 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.03 us` (❌ *2.14x slower*)    | `27.44 us` (❌ *3.91x slower*)     | `14.65 us` (❌ *2.09x slower*)     | `7.02 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `581.14 ns` (❌ *9.47x slower*)   | `14.64 us` (❌ *238.59x slower*)   | `117.96 ns` (❌ *1.92x slower*)    | `61.36 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `568.45 ns` (❌ *6.21x slower*)   | `14.56 us` (❌ *159.15x slower*)   | `164.15 ns` (❌ *1.79x slower*)    | `91.47 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.44 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.59 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.01x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.43 ns` (✅ **1.00x**)        | `222.11 ns` (❌ *1.38x slower*)   | `32.40 ns` (🚀 **4.95x faster**)    | `55.53 ns` (🚀 **2.89x faster**)    | `108.85 ns` (✅ **1.47x faster**)    | `693.37 ns` (❌ *4.32x slower*)    |
| **`serialize_uncompressed`**             | `208.99 ns` (✅ **1.00x**)        | `343.22 ns` (❌ *1.64x slower*)   | `31.33 ns` (🚀 **6.67x faster**)    | `56.36 ns` (🚀 **3.71x faster**)    | `108.68 ns` (🚀 **1.92x faster**)    | `705.99 ns` (❌ *3.38x slower*)    |
| **`deserialize_compressed`**             | `314.91 us` (✅ **1.00x**)        | `1.07 ms` (❌ *3.38x slower*)     | `51.90 ns` (🚀 **6067.20x faster**) | `92.04 ns` (🚀 **3421.37x faster**) | `211.04 ns` (🚀 **1492.16x faster**) | `1.26 us` (🚀 **249.47x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.29 us` (✅ **1.00x**)         | `183.58 us` (❌ *2.69x slower*)   | `51.86 ns` (🚀 **1316.63x faster**) | `92.18 ns` (🚀 **740.76x faster**)  | `211.10 ns` (🚀 **323.48x faster**)  | `1.26 us` (🚀 **54.09x faster**)   |
| **`deserialize_uncompressed`**           | `246.76 us` (✅ **1.00x**)        | `879.76 us` (❌ *3.57x slower*)   | `51.83 ns` (🚀 **4761.42x faster**) | `92.66 ns` (🚀 **2663.21x faster**) | `210.94 ns` (🚀 **1169.82x faster**) | `1.26 us` (🚀 **195.51x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `228.72 ns` (✅ **1.00x**)        | `465.64 ns` (❌ *2.04x slower*)   | `51.81 ns` (🚀 **4.41x faster**)    | `92.59 ns` (🚀 **2.47x faster**)    | `211.13 ns` (✅ **1.08x faster**)    | `1.26 us` (❌ *5.51x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.37 s` (✅ **1.00x**)           | `8.38 s` (❌ *3.54x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.17 us` (✅ **1.00x**) | `67.72 us` (❌ *2.17x slower*)   | `182.74 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.99 us` (✅ **1.00x**) | `31.28 us` (❌ *2.85x slower*)   | `31.38 us` (❌ *2.86x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.27 ns` (✅ **1.00x**)       | `90.21 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `50.23 ns` (✅ **1.00x**)       | `90.28 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.74 ns` (✅ **1.00x**) | `75.10 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `47.04 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

