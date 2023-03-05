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
|        | `193.65 us` (✅ **1.00x**)                 | `1.88 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.77x slower*)     | `29.10 ns` (🚀 **38.94x faster**)  | `177.41 ns` (🚀 **6.39x faster**)  | `19.51 ns` (🚀 **58.07x faster**) | `8.29 ns` (🚀 **136.77x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.32 us` (❌ *3.70x slower*)     | `27.21 ns` (🚀 **42.89x faster**)  | `167.25 ns` (🚀 **6.98x faster**)  | `15.02 ns` (🚀 **77.70x faster**) | `8.68 ns` (🚀 **134.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `831.06 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.70x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `855.97 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.63x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `568.58 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.60x slower*)     | `12.82 ns` (🚀 **44.37x faster**)  | `100.62 ns` (🚀 **5.65x faster**)  | `7.47 ns` (🚀 **76.09x faster**)  | `9.06 ns` (🚀 **62.79x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.12 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.69 ns` (❌ *3.81x slower*)     | `107.26 ns` (❌ *18.02x slower*)   | `16.77 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.45 ns` (❌ *7.18x slower*)    | `6.68 us` (❌ *179.17x slower*)    | `69.36 ns` (❌ *1.86x slower*)    | `37.27 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `249.37 ns` (❌ *7.82x slower*)    | `4.72 us` (❌ *148.12x slower*)    | `59.31 ns` (❌ *1.86x slower*)    | `31.87 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.78 us` (❌ *2.16x slower*)     | `25.04 us` (❌ *3.93x slower*)     | `13.44 us` (❌ *2.11x slower*)    | `6.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `571.38 ns` (❌ *10.76x slower*)   | `13.63 us` (❌ *256.77x slower*)   | `112.28 ns` (❌ *2.12x slower*)   | `53.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `552.72 ns` (❌ *6.93x slower*)    | `13.54 us` (❌ *169.72x slower*)   | `157.26 ns` (❌ *1.97x slower*)   | `79.80 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `143.65 ns` (✅ **1.00x**)        | `211.68 ns` (❌ *1.47x slower*)   | `27.86 ns` (🚀 **5.16x faster**)    | `50.29 ns` (🚀 **2.86x faster**)    | `99.84 ns` (✅ **1.44x faster**)     | `630.35 ns` (❌ *4.39x slower*)    |
| **`serialize_uncompressed`**             | `197.33 ns` (✅ **1.00x**)        | `319.81 ns` (❌ *1.62x slower*)   | `27.97 ns` (🚀 **7.06x faster**)    | `50.16 ns` (🚀 **3.93x faster**)    | `99.84 ns` (🚀 **1.98x faster**)     | `626.38 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `281.89 us` (✅ **1.00x**)        | `973.60 us` (❌ *3.45x slower*)   | `47.33 ns` (🚀 **5955.41x faster**) | `93.78 ns` (🚀 **3005.68x faster**) | `206.92 ns` (🚀 **1362.33x faster**) | `1.25 us` (🚀 **224.94x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.54 us` (✅ **1.00x**)         | `174.80 us` (❌ *2.67x slower*)   | `47.40 ns` (🚀 **1382.85x faster**) | `93.80 ns` (🚀 **698.76x faster**)  | `206.88 ns` (🚀 **316.81x faster**)  | `1.26 us` (🚀 **52.11x faster**)   |
| **`deserialize_uncompressed`**           | `216.57 us` (✅ **1.00x**)        | `795.84 us` (❌ *3.67x slower*)   | `47.23 ns` (🚀 **4585.60x faster**) | `93.75 ns` (🚀 **2310.06x faster**) | `206.75 ns` (🚀 **1047.50x faster**) | `1.25 us` (🚀 **172.88x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `228.32 ns` (✅ **1.00x**)        | `468.19 ns` (❌ *2.05x slower*)   | `46.93 ns` (🚀 **4.86x faster**)    | `93.77 ns` (🚀 **2.43x faster**)    | `206.76 ns` (✅ **1.10x faster**)    | `1.25 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.26 s` (✅ **1.00x**)           | `7.92 s` (❌ *3.51x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.65 us` (✅ **1.00x**) | `65.20 us` (❌ *2.36x slower*)   | `174.00 us` (❌ *6.29x slower*)    |
| **`legendre_for_qr`**    | `9.53 us` (✅ **1.00x**)  | `29.08 us` (❌ *3.05x slower*)   | `29.31 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.33 ns` (✅ **1.00x**)       | `108.71 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `60.52 ns` (✅ **1.00x**)       | `108.76 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.06 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.86 ns` (✅ **1.00x**) | `79.72 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `21.73 ns` (✅ **1.00x**) | `41.47 ns` (❌ *1.91x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `9.03 ns` (✅ **1.00x**)                          | `10.67 ns` (❌ *1.18x slower*)                    | `869.16 us` (❌ *96295.35x slower*)            | `1.17 ms` (❌ *130089.14x slower*)                      | `2.06 ms` (❌ *228013.29x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

