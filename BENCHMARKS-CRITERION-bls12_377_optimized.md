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
    - [pairing_for_bls12_377optimized](#pairing_for_bls12_377optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `193.26 us` (✅ **1.00x**)                 | `1.88 ms` (❌ *9.72x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.14 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.76x slower*)     | `28.29 ns` (🚀 **40.14x faster**)  | `177.34 ns` (🚀 **6.40x faster**)  | `19.02 ns` (🚀 **59.71x faster**) | `8.29 ns` (🚀 **137.00x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.31 us` (❌ *3.69x slower*)     | `27.10 ns` (🚀 **43.11x faster**)  | `168.01 ns` (🚀 **6.95x faster**)  | `14.86 ns` (🚀 **78.63x faster**) | `8.62 ns` (🚀 **135.55x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `832.69 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.69x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `855.01 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.64x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `571.81 ns` (✅ **1.00x**)        | `2.04 us` (❌ *3.57x slower*)     | `12.79 ns` (🚀 **44.70x faster**)  | `100.42 ns` (🚀 **5.69x faster**)  | `7.48 ns` (🚀 **76.50x faster**)  | `9.10 ns` (🚀 **62.86x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.71 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.37 ns` (❌ *3.93x slower*)     | `108.46 ns` (❌ *18.22x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `266.94 ns` (❌ *7.15x slower*)    | `6.69 us` (❌ *179.42x slower*)    | `69.39 ns` (❌ *1.86x slower*)    | `37.31 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `245.41 ns` (❌ *7.68x slower*)    | `4.72 us` (❌ *147.88x slower*)    | `59.20 ns` (❌ *1.85x slower*)    | `31.95 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.81 us` (❌ *2.19x slower*)     | `25.14 us` (❌ *3.99x slower*)     | `13.48 us` (❌ *2.14x slower*)    | `6.30 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `568.05 ns` (❌ *10.70x slower*)   | `13.64 us` (❌ *256.88x slower*)   | `111.67 ns` (❌ *2.10x slower*)   | `53.09 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `552.00 ns` (❌ *6.71x slower*)    | `13.53 us` (❌ *164.36x slower*)   | `156.98 ns` (❌ *1.91x slower*)   | `82.32 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `144.28 ns` (✅ **1.00x**)        | `210.98 ns` (❌ *1.46x slower*)   | `28.13 ns` (🚀 **5.13x faster**)    | `50.33 ns` (🚀 **2.87x faster**)    | `100.17 ns` (✅ **1.44x faster**)    | `625.88 ns` (❌ *4.34x slower*)    |
| **`serialize_uncompressed`**             | `198.78 ns` (✅ **1.00x**)        | `321.70 ns` (❌ *1.62x slower*)   | `28.01 ns` (🚀 **7.10x faster**)    | `50.07 ns` (🚀 **3.97x faster**)    | `100.17 ns` (🚀 **1.98x faster**)    | `626.04 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `282.06 us` (✅ **1.00x**)        | `975.06 us` (❌ *3.46x slower*)   | `46.51 ns` (🚀 **6064.87x faster**) | `93.65 ns` (🚀 **3011.82x faster**) | `206.41 ns` (🚀 **1366.55x faster**) | `1.25 us` (🚀 **224.85x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.13 us` (✅ **1.00x**)         | `174.07 us` (❌ *2.67x slower*)   | `46.51 ns` (🚀 **1400.44x faster**) | `93.66 ns` (🚀 **695.41x faster**)  | `206.38 ns` (🚀 **315.61x faster**)  | `1.26 us` (🚀 **51.74x faster**)   |
| **`deserialize_uncompressed`**           | `217.12 us` (✅ **1.00x**)        | `797.87 us` (❌ *3.67x slower*)   | `46.45 ns` (🚀 **4674.41x faster**) | `94.34 ns` (🚀 **2301.50x faster**) | `208.03 ns` (🚀 **1043.72x faster**) | `1.25 us` (🚀 **173.03x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `229.91 ns` (✅ **1.00x**)        | `474.75 ns` (❌ *2.06x slower*)   | `46.44 ns` (🚀 **4.95x faster**)    | `94.25 ns` (🚀 **2.44x faster**)    | `206.56 ns` (✅ **1.11x faster**)    | `1.25 us` (❌ *5.45x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.30 s` (✅ **1.00x**)           | `7.98 s` (❌ *3.47x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.74 us` (✅ **1.00x**) | `64.79 us` (❌ *2.34x slower*)   | `173.20 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `9.56 us` (✅ **1.00x**)  | `29.31 us` (❌ *3.07x slower*)   | `29.56 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `62.00 ns` (✅ **1.00x**)       | `108.56 ns` (❌ *1.75x slower*)    |
| **`from_big-endian_bits`**    | `60.85 ns` (✅ **1.00x**)       | `108.55 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)        | `4.33 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.17 ns` (✅ **1.00x**) | `79.32 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.67 ns` (✅ **1.00x**) | `41.50 ns` (❌ *1.92x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `9.04 ns` (✅ **1.00x**)                          | `11.06 ns` (❌ *1.22x slower*)                    | `870.04 us` (❌ *96205.84x slower*)            | `1.18 ms` (❌ *130350.36x slower*)                      | `2.07 ms` (❌ *229076.63x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

