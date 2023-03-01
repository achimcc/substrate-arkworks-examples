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
|        | `193.14 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.78x slower*)     | `28.02 ns` (🚀 **40.28x faster**)  | `180.78 ns` (🚀 **6.24x faster**)  | `19.43 ns` (🚀 **58.09x faster**) | `8.26 ns` (🚀 **136.60x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.16 us` (✅ **1.00x**)          | `4.31 us` (❌ *3.70x slower*)     | `27.50 ns` (🚀 **42.32x faster**)  | `171.39 ns` (🚀 **6.79x faster**)  | `14.71 ns` (🚀 **79.16x faster**) | `8.64 ns` (🚀 **134.79x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `832.79 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.69x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `854.36 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.64x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `567.87 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.61x slower*)     | `12.89 ns` (🚀 **44.06x faster**)  | `103.74 ns` (🚀 **5.47x faster**)  | `11.03 ns` (🚀 **51.49x faster**) | `9.06 ns` (🚀 **62.69x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.84 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.61 ns` (❌ *3.80x slower*)     | `104.93 ns` (❌ *17.63x slower*)   | `16.74 ns` (❌ *2.81x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `266.92 ns` (❌ *7.15x slower*)    | `6.66 us` (❌ *178.54x slower*)    | `71.05 ns` (❌ *1.90x slower*)    | `37.32 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `246.45 ns` (❌ *7.53x slower*)    | `4.70 us` (❌ *143.53x slower*)    | `59.17 ns` (❌ *1.81x slower*)    | `32.73 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.87 us` (❌ *2.17x slower*)     | `25.07 us` (❌ *3.92x slower*)     | `13.53 us` (❌ *2.12x slower*)    | `6.39 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `570.63 ns` (❌ *10.76x slower*)   | `13.59 us` (❌ *256.38x slower*)   | `112.15 ns` (❌ *2.12x slower*)   | `53.02 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `554.39 ns` (❌ *6.75x slower*)    | `13.51 us` (❌ *164.37x slower*)   | `156.95 ns` (❌ *1.91x slower*)   | `82.17 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `164.08 ns` (✅ **1.00x**)        | `210.91 ns` (❌ *1.29x slower*)   | `28.27 ns` (🚀 **5.80x faster**)    | `50.41 ns` (🚀 **3.25x faster**)    | `100.14 ns` (✅ **1.64x faster**)    | `628.46 ns` (❌ *3.83x slower*)    |
| **`serialize_uncompressed`**             | `198.93 ns` (✅ **1.00x**)        | `326.49 ns` (❌ *1.64x slower*)   | `28.08 ns` (🚀 **7.08x faster**)    | `50.43 ns` (🚀 **3.94x faster**)    | `100.14 ns` (🚀 **1.99x faster**)    | `627.23 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `282.08 us` (✅ **1.00x**)        | `973.40 us` (❌ *3.45x slower*)   | `45.48 ns` (🚀 **6201.95x faster**) | `93.48 ns` (🚀 **3017.52x faster**) | `206.20 ns` (🚀 **1367.95x faster**) | `1.26 us` (🚀 **224.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.00 us` (✅ **1.00x**)         | `173.93 us` (❌ *2.68x slower*)   | `45.48 ns` (🚀 **1429.22x faster**) | `93.46 ns` (🚀 **695.45x faster**)  | `206.17 ns` (🚀 **315.27x faster**)  | `1.26 us` (🚀 **51.69x faster**)   |
| **`deserialize_uncompressed`**           | `217.12 us` (✅ **1.00x**)        | `795.64 us` (❌ *3.66x slower*)   | `44.62 ns` (🚀 **4866.32x faster**) | `93.29 ns` (🚀 **2327.29x faster**) | `206.05 ns` (🚀 **1053.74x faster**) | `1.26 us` (🚀 **172.74x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.97 ns` (✅ **1.00x**)        | `479.22 ns` (❌ *2.12x slower*)   | `44.63 ns` (🚀 **5.06x faster**)    | `93.30 ns` (🚀 **2.42x faster**)    | `205.97 ns` (✅ **1.10x faster**)    | `1.26 us` (❌ *5.56x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.27 s` (✅ **1.00x**)           | `7.89 s` (❌ *3.48x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.81 us` (✅ **1.00x**) | `64.65 us` (❌ *2.32x slower*)   | `173.59 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `9.54 us` (✅ **1.00x**)  | `29.24 us` (❌ *3.07x slower*)   | `29.62 us` (❌ *3.11x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.10 ns` (✅ **1.00x**)       | `108.60 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `61.13 ns` (✅ **1.00x**)       | `108.59 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.30 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.73 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.85 ns` (✅ **1.00x**) | `79.65 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `21.70 ns` (✅ **1.00x**) | `41.54 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

