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
|        | `193.33 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.28 us` (❌ *3.78x slower*)     | `27.04 ns` (🚀 **41.87x faster**)  | `178.64 ns` (🚀 **6.34x faster**)  | `19.25 ns` (🚀 **58.83x faster**) | `8.27 ns` (🚀 **136.97x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.16 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.71x slower*)     | `27.16 ns` (🚀 **42.85x faster**)  | `171.65 ns` (🚀 **6.78x faster**)  | `15.19 ns` (🚀 **76.62x faster**) | `8.59 ns` (🚀 **135.50x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `828.22 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.71x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `853.03 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.65x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `566.16 ns` (✅ **1.00x**)        | `2.04 us` (❌ *3.61x slower*)     | `12.72 ns` (🚀 **44.52x faster**)  | `102.35 ns` (🚀 **5.53x faster**)  | `11.08 ns` (🚀 **51.11x faster**) | `9.04 ns` (🚀 **62.66x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `300.07 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.59x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.88 ns` (❌ *3.85x slower*)     | `104.07 ns` (❌ *17.50x slower*)   | `17.24 ns` (❌ *2.90x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `265.46 ns` (❌ *7.12x slower*)    | `6.64 us` (❌ *178.10x slower*)    | `69.75 ns` (❌ *1.87x slower*)    | `37.30 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `246.99 ns` (❌ *7.73x slower*)    | `4.67 us` (❌ *146.26x slower*)    | `59.05 ns` (❌ *1.85x slower*)    | `31.94 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.91 us` (❌ *2.20x slower*)     | `25.17 us` (❌ *3.99x slower*)     | `13.60 us` (❌ *2.15x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `571.25 ns` (❌ *10.76x slower*)   | `13.57 us` (❌ *255.71x slower*)   | `111.62 ns` (❌ *2.10x slower*)   | `53.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `549.86 ns` (❌ *6.71x slower*)    | `13.48 us` (❌ *164.49x slower*)   | `156.12 ns` (❌ *1.91x slower*)   | `81.95 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.13 ns` (✅ **1.00x**)        | `211.03 ns` (❌ *1.40x slower*)   | `27.86 ns` (🚀 **5.42x faster**)    | `50.46 ns` (🚀 **3.00x faster**)    | `100.14 ns` (✅ **1.51x faster**)    | `627.06 ns` (❌ *4.15x slower*)    |
| **`serialize_uncompressed`**             | `198.84 ns` (✅ **1.00x**)        | `321.54 ns` (❌ *1.62x slower*)   | `27.85 ns` (🚀 **7.14x faster**)    | `50.28 ns` (🚀 **3.96x faster**)    | `100.14 ns` (🚀 **1.99x faster**)    | `627.23 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `282.06 us` (✅ **1.00x**)        | `973.35 us` (❌ *3.45x slower*)   | `46.08 ns` (🚀 **6120.55x faster**) | `94.69 ns` (🚀 **2978.68x faster**) | `210.03 ns` (🚀 **1342.99x faster**) | `1.25 us` (🚀 **224.96x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.90 us` (✅ **1.00x**)         | `173.26 us` (❌ *2.67x slower*)   | `46.41 ns` (🚀 **1398.60x faster**) | `93.93 ns` (🚀 **691.00x faster**)  | `207.83 ns` (🚀 **312.30x faster**)  | `1.26 us` (🚀 **51.61x faster**)   |
| **`deserialize_uncompressed`**           | `217.43 us` (✅ **1.00x**)        | `796.40 us` (❌ *3.66x slower*)   | `46.03 ns` (🚀 **4724.07x faster**) | `94.03 ns` (🚀 **2312.37x faster**) | `207.61 ns` (🚀 **1047.29x faster**) | `1.25 us` (🚀 **173.33x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `227.75 ns` (✅ **1.00x**)        | `475.22 ns` (❌ *2.09x slower*)   | `46.10 ns` (🚀 **4.94x faster**)    | `94.03 ns` (🚀 **2.42x faster**)    | `207.62 ns` (✅ **1.10x faster**)    | `1.26 us` (❌ *5.52x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.28 s` (✅ **1.00x**)           | `7.96 s` (❌ *3.49x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.67 us` (✅ **1.00x**) | `64.33 us` (❌ *2.32x slower*)   | `172.13 us` (❌ *6.22x slower*)    |
| **`legendre_for_qr`**    | `9.53 us` (✅ **1.00x**)  | `29.31 us` (❌ *3.08x slower*)   | `29.57 us` (❌ *3.10x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.31 ns` (✅ **1.00x**)       | `107.86 ns` (❌ *1.76x slower*)    |
| **`from_big-endian_bits`**    | `61.38 ns` (✅ **1.00x**)       | `108.02 ns` (❌ *1.76x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.03 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.92 ns` (✅ **1.00x**) | `78.63 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.64 ns` (✅ **1.00x**) | `41.53 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

