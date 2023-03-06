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
|        | `193.24 us` (✅ **1.00x**)                 | `1.88 ms` (❌ *9.71x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.15 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.72x slower*)     | `29.50 ns` (🚀 **38.91x faster**)  | `179.51 ns` (🚀 **6.39x faster**)  | `19.04 ns` (🚀 **60.28x faster**) | `8.30 ns` (🚀 **138.32x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.18 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.65x slower*)     | `26.89 ns` (🚀 **43.98x faster**)  | `169.31 ns` (🚀 **6.99x faster**)  | `14.80 ns` (🚀 **79.89x faster**) | `8.62 ns` (🚀 **137.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `847.09 ns` (✅ **1.00x**)        | `3.07 us` (❌ *3.63x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `872.81 ns` (✅ **1.00x**)        | `3.09 us` (❌ *3.54x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `588.61 ns` (✅ **1.00x**)        | `2.04 us` (❌ *3.47x slower*)     | `12.90 ns` (🚀 **45.65x faster**)  | `103.48 ns` (🚀 **5.69x faster**)  | `7.48 ns` (🚀 **78.67x faster**)  | `9.08 ns` (🚀 **64.85x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.79 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.88 ns` (❌ *3.85x slower*)     | `109.30 ns` (❌ *18.37x slower*)   | `16.75 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `266.58 ns` (❌ *7.15x slower*)    | `6.68 us` (❌ *179.06x slower*)    | `69.36 ns` (❌ *1.86x slower*)    | `37.30 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `245.73 ns` (❌ *7.71x slower*)    | `4.71 us` (❌ *147.77x slower*)    | `59.12 ns` (❌ *1.86x slower*)    | `31.86 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.78 us` (❌ *2.19x slower*)     | `25.05 us` (❌ *3.98x slower*)     | `13.47 us` (❌ *2.14x slower*)    | `6.29 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `568.96 ns` (❌ *10.72x slower*)   | `13.63 us` (❌ *256.79x slower*)   | `110.51 ns` (❌ *2.08x slower*)   | `53.06 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `552.63 ns` (❌ *6.72x slower*)    | `13.52 us` (❌ *164.47x slower*)   | `156.85 ns` (❌ *1.91x slower*)   | `82.21 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `144.83 ns` (✅ **1.00x**)        | `211.07 ns` (❌ *1.46x slower*)   | `27.94 ns` (🚀 **5.18x faster**)    | `50.30 ns` (🚀 **2.88x faster**)    | `100.15 ns` (✅ **1.45x faster**)    | `625.56 ns` (❌ *4.32x slower*)    |
| **`serialize_uncompressed`**             | `198.20 ns` (✅ **1.00x**)        | `321.82 ns` (❌ *1.62x slower*)   | `27.81 ns` (🚀 **7.13x faster**)    | `50.05 ns` (🚀 **3.96x faster**)    | `100.10 ns` (🚀 **1.98x faster**)    | `625.90 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `281.97 us` (✅ **1.00x**)        | `974.77 us` (❌ *3.46x slower*)   | `46.48 ns` (🚀 **6066.28x faster**) | `93.63 ns` (🚀 **3011.40x faster**) | `206.33 ns` (🚀 **1366.63x faster**) | `1.25 us` (🚀 **224.90x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.16 us` (✅ **1.00x**)         | `173.99 us` (❌ *2.67x slower*)   | `46.51 ns` (🚀 **1400.90x faster**) | `93.64 ns` (🚀 **695.86x faster**)  | `206.94 ns` (🚀 **314.87x faster**)  | `1.25 us` (🚀 **52.00x faster**)   |
| **`deserialize_uncompressed`**           | `216.96 us` (✅ **1.00x**)        | `797.56 us` (❌ *3.68x slower*)   | `46.42 ns` (🚀 **4674.37x faster**) | `93.59 ns` (🚀 **2318.25x faster**) | `207.80 ns` (🚀 **1044.07x faster**) | `1.26 us` (🚀 **172.33x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `229.80 ns` (✅ **1.00x**)        | `474.54 ns` (❌ *2.07x slower*)   | `46.40 ns` (🚀 **4.95x faster**)    | `93.60 ns` (🚀 **2.46x faster**)    | `207.85 ns` (✅ **1.11x faster**)    | `1.25 us` (❌ *5.46x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.26 s` (✅ **1.00x**)           | `7.88 s` (❌ *3.49x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.72 us` (✅ **1.00x**) | `64.72 us` (❌ *2.33x slower*)   | `173.29 us` (❌ *6.25x slower*)    |
| **`legendre_for_qr`**    | `9.61 us` (✅ **1.00x**)  | `29.25 us` (❌ *3.04x slower*)   | `29.49 us` (❌ *3.07x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.89 ns` (✅ **1.00x**)       | `108.61 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.86 ns` (✅ **1.00x**)       | `108.64 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.14 ns` (✅ **1.00x**) | `79.28 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.48 ns` (❌ *1.92x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `9.03 ns` (✅ **1.00x**)                          | `11.05 ns` (❌ *1.22x slower*)                    | `869.37 us` (❌ *96227.57x slower*)            | `1.18 ms` (❌ *130377.89x slower*)                      | `2.07 ms` (❌ *229170.00x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

