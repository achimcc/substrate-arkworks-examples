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
|        | `193.64 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.67x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.29 us` (❌ *3.79x slower*)     | `26.88 ns` (🚀 **42.10x faster**)  | `176.87 ns` (🚀 **6.40x faster**)  | `19.06 ns` (🚀 **59.35x faster**) | `8.29 ns` (🚀 **136.46x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.16 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.71x slower*)     | `27.19 ns` (🚀 **42.83x faster**)  | `171.06 ns` (🚀 **6.81x faster**)  | `14.80 ns` (🚀 **78.70x faster**) | `8.60 ns` (🚀 **135.47x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `825.20 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.74x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `852.52 ns` (✅ **1.00x**)        | `3.13 us` (❌ *3.67x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `572.45 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.58x slower*)     | `12.78 ns` (🚀 **44.80x faster**)  | `100.53 ns` (🚀 **5.69x faster**)  | `11.09 ns` (🚀 **51.62x faster**) | `5.32 ns` (🚀 **107.58x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `300.31 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.59x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.88 ns` (❌ *3.85x slower*)     | `100.53 ns` (❌ *16.91x slower*)   | `16.77 ns` (❌ *2.82x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.20 ns` (❌ *7.16x slower*)    | `6.69 us` (❌ *179.25x slower*)    | `69.41 ns` (❌ *1.86x slower*)    | `37.31 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `249.51 ns` (❌ *7.82x slower*)    | `4.69 us` (❌ *147.10x slower*)    | `59.12 ns` (❌ *1.85x slower*)    | `31.90 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.73 us` (❌ *2.17x slower*)     | `25.04 us` (❌ *3.97x slower*)     | `13.40 us` (❌ *2.12x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `570.15 ns` (❌ *10.75x slower*)   | `13.59 us` (❌ *256.06x slower*)   | `111.71 ns` (❌ *2.11x slower*)   | `53.05 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `553.85 ns` (❌ *6.73x slower*)    | `13.50 us` (❌ *164.02x slower*)   | `157.30 ns` (❌ *1.91x slower*)   | `82.30 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.36 ns` (✅ **1.00x**)        | `211.89 ns` (❌ *1.40x slower*)   | `28.05 ns` (🚀 **5.40x faster**)    | `50.33 ns` (🚀 **3.01x faster**)    | `99.72 ns` (✅ **1.52x faster**)     | `626.94 ns` (❌ *4.14x slower*)    |
| **`serialize_uncompressed`**             | `198.73 ns` (✅ **1.00x**)        | `326.60 ns` (❌ *1.64x slower*)   | `27.95 ns` (🚀 **7.11x faster**)    | `50.18 ns` (🚀 **3.96x faster**)    | `99.71 ns` (🚀 **1.99x faster**)     | `626.78 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `282.43 us` (✅ **1.00x**)        | `973.44 us` (❌ *3.45x slower*)   | `46.64 ns` (🚀 **6055.85x faster**) | `95.65 ns` (🚀 **2952.62x faster**) | `205.81 ns` (🚀 **1372.30x faster**) | `1.27 us` (🚀 **222.07x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.10 us` (✅ **1.00x**)         | `173.98 us` (❌ *2.67x slower*)   | `46.64 ns` (🚀 **1395.96x faster**) | `95.65 ns` (🚀 **680.64x faster**)  | `207.40 ns` (🚀 **313.90x faster**)  | `1.27 us` (🚀 **51.31x faster**)   |
| **`deserialize_uncompressed`**           | `217.32 us` (✅ **1.00x**)        | `794.93 us` (❌ *3.66x slower*)   | `46.56 ns` (🚀 **4667.96x faster**) | `95.63 ns` (🚀 **2272.51x faster**) | `207.44 ns` (🚀 **1047.65x faster**) | `1.27 us` (🚀 **171.32x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.23 ns` (✅ **1.00x**)        | `472.62 ns` (❌ *2.10x slower*)   | `46.56 ns` (🚀 **4.84x faster**)    | `96.37 ns` (🚀 **2.34x faster**)    | `206.01 ns` (✅ **1.09x faster**)    | `1.27 us` (❌ *5.63x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.29 s` (✅ **1.00x**)           | `8.05 s` (❌ *3.52x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.69 us` (✅ **1.00x**) | `64.71 us` (❌ *2.34x slower*)   | `172.74 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `9.58 us` (✅ **1.00x**)  | `29.20 us` (❌ *3.05x slower*)   | `29.58 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.76 ns` (✅ **1.00x**)       | `109.16 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `60.85 ns` (✅ **1.00x**)       | `109.28 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.96 ns` (✅ **1.00x**) | `79.29 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `21.67 ns` (✅ **1.00x**) | `41.54 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

