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
|        | `193.24 us` (✅ **1.00x**)                 | `1.88 ms` (❌ *9.72x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.77x slower*)     | `29.65 ns` (🚀 **38.20x faster**)  | `173.34 ns` (🚀 **6.53x faster**)  | `19.47 ns` (🚀 **58.17x faster**) | `8.29 ns` (🚀 **136.71x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.70x slower*)     | `27.30 ns` (🚀 **42.77x faster**)  | `166.98 ns` (🚀 **6.99x faster**)  | `16.43 ns` (🚀 **71.07x faster**) | `8.60 ns` (🚀 **135.69x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `835.16 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.69x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `852.87 ns` (✅ **1.00x**)        | `3.12 us` (❌ *3.66x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `568.46 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.60x slower*)     | `12.87 ns` (🚀 **44.18x faster**)  | `102.74 ns` (🚀 **5.53x faster**)  | `11.06 ns` (🚀 **51.39x faster**) | `5.30 ns` (🚀 **107.30x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `300.28 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.89 ns` (❌ *3.84x slower*)     | `101.86 ns` (❌ *17.08x slower*)   | `17.03 ns` (❌ *2.86x slower*)    | `5.96 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `266.92 ns` (❌ *7.15x slower*)    | `6.66 us` (❌ *178.45x slower*)    | `68.89 ns` (❌ *1.85x slower*)    | `37.34 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `247.41 ns` (❌ *7.77x slower*)    | `4.67 us` (❌ *146.73x slower*)    | `58.95 ns` (❌ *1.85x slower*)    | `31.85 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.58 us` (❌ *2.15x slower*)     | `24.91 us` (❌ *3.95x slower*)     | `13.29 us` (❌ *2.11x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `568.94 ns` (❌ *10.73x slower*)   | `13.58 us` (❌ *256.13x slower*)   | `110.42 ns` (❌ *2.08x slower*)   | `53.03 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `550.02 ns` (❌ *6.67x slower*)    | `13.49 us` (❌ *163.60x slower*)   | `156.11 ns` (❌ *1.89x slower*)   | `82.44 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.46 ns` (✅ **1.00x**)        | `213.42 ns` (❌ *1.41x slower*)   | `30.70 ns` (🚀 **4.93x faster**)    | `50.31 ns` (🚀 **3.01x faster**)    | `99.82 ns` (✅ **1.52x faster**)     | `633.63 ns` (❌ *4.18x slower*)    |
| **`serialize_uncompressed`**             | `198.88 ns` (✅ **1.00x**)        | `323.96 ns` (❌ *1.63x slower*)   | `30.57 ns` (🚀 **6.51x faster**)    | `50.23 ns` (🚀 **3.96x faster**)    | `99.83 ns` (🚀 **1.99x faster**)     | `631.42 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `282.32 us` (✅ **1.00x**)        | `974.71 us` (❌ *3.45x slower*)   | `46.44 ns` (🚀 **6078.67x faster**) | `94.19 ns` (🚀 **2997.25x faster**) | `207.28 ns` (🚀 **1362.04x faster**) | `1.25 us` (🚀 **225.24x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.04 us` (✅ **1.00x**)         | `173.78 us` (❌ *2.67x slower*)   | `46.47 ns` (🚀 **1399.69x faster**) | `93.66 ns` (🚀 **694.39x faster**)  | `207.32 ns` (🚀 **313.71x faster**)  | `1.26 us` (🚀 **51.82x faster**)   |
| **`deserialize_uncompressed`**           | `217.33 us` (✅ **1.00x**)        | `798.66 us` (❌ *3.67x slower*)   | `46.39 ns` (🚀 **4684.59x faster**) | `93.71 ns` (🚀 **2319.09x faster**) | `206.75 ns` (🚀 **1051.16x faster**) | `1.25 us` (🚀 **173.95x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `237.16 ns` (✅ **1.00x**)        | `471.51 ns` (❌ *1.99x slower*)   | `46.40 ns` (🚀 **5.11x faster**)    | `93.73 ns` (🚀 **2.53x faster**)    | `204.99 ns` (✅ **1.16x faster**)    | `1.25 us` (❌ *5.27x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.28 s` (✅ **1.00x**)           | `8.00 s` (❌ *3.50x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.73 us` (✅ **1.00x**) | `64.72 us` (❌ *2.33x slower*)   | `172.56 us` (❌ *6.22x slower*)    |
| **`legendre_for_qr`**    | `9.55 us` (✅ **1.00x**)  | `29.09 us` (❌ *3.05x slower*)   | `29.51 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.28 ns` (✅ **1.00x**)       | `107.87 ns` (❌ *1.76x slower*)    |
| **`from_big-endian_bits`**    | `61.24 ns` (✅ **1.00x**)       | `107.93 ns` (❌ *1.76x slower*)    |
| **`comparison`**              | `4.04 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.00 ns` (✅ **1.00x**) | `78.94 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.55 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

