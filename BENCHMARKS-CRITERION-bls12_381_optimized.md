# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381_optimized](#sample_bls12_381_optimized)
    - [arithmetic_for_bls12_381_optimized](#arithmetic_for_bls12_381_optimized)
    - [serialization_for_bls12_381_optimized](#serialization_for_bls12_381_optimized)
    - [msm_for_bls12_381_optimized](#msm_for_bls12_381_optimized)
    - [squareroot_for_bls12_381_optimized](#squareroot_for_bls12_381_optimized)
    - [bitwise_operations_for_bls12_381_optimized](#bitwise_operations_for_bls12_381_optimized)
    - [conversions_for_bls12_381_optimized](#conversions_for_bls12_381_optimized)

## Benchmark Results

### sample_bls12_381_optimized

|        | `g1projectivebls12_381_elements`          | `g2projectivebls12_381_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `196.46 us` (✅ **1.00x**)                 | `1.64 ms` (❌ *8.33x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.12 us` (✅ **1.00x**)          | `3.57 us` (❌ *3.20x slower*)     | `26.77 ns` (🚀 **41.72x faster**) | `180.12 ns` (🚀 **6.20x faster**)  | `19.15 ns` (🚀 **58.33x faster**) | `8.18 ns` (🚀 **136.47x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.15 us` (✅ **1.00x**)          | `3.61 us` (❌ *3.13x slower*)     | `27.21 ns` (🚀 **42.36x faster**) | `172.11 ns` (🚀 **6.70x faster**)  | `16.11 ns` (🚀 **71.54x faster**) | `8.59 ns` (🚀 **134.20x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `815.17 ns` (✅ **1.00x**)        | `2.58 us` (❌ *3.16x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `839.01 ns` (✅ **1.00x**)        | `2.61 us` (❌ *3.11x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `563.60 ns` (✅ **1.00x**)        | `1.63 us` (❌ *2.90x slower*)     | `12.96 ns` (🚀 **43.48x faster**) | `102.10 ns` (🚀 **5.52x faster**)  | `7.63 ns` (🚀 **73.91x faster**)  | `5.29 ns` (🚀 **106.54x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `298.35 us` (✅ **1.00x**)        | `883.12 us` (❌ *2.96x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.88 ns` (❌ *3.84x slower*)    | `107.64 ns` (❌ *18.08x slower*)   | `17.31 ns` (❌ *2.91x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `225.35 ns` (❌ *5.80x slower*)   | `5.76 us` (❌ *148.33x slower*)    | `70.26 ns` (❌ *1.81x slower*)    | `38.86 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `174.86 ns` (❌ *4.91x slower*)   | `4.04 us` (❌ *113.46x slower*)    | `58.45 ns` (❌ *1.64x slower*)    | `35.63 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.85 us` (❌ *2.17x slower*)    | `23.17 us` (❌ *3.63x slower*)     | `13.53 us` (❌ *2.12x slower*)    | `6.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `492.61 ns` (❌ *6.07x slower*)   | `11.79 us` (❌ *145.18x slower*)   | `106.31 ns` (❌ *1.31x slower*)   | `81.19 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `474.03 ns` (❌ *5.83x slower*)   | `11.66 us` (❌ *143.41x slower*)   | `156.86 ns` (❌ *1.93x slower*)   | `81.33 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.74 ns` (✅ **1.00x**)        | `192.39 ns` (❌ *1.36x slower*)   | `30.12 ns` (🚀 **4.71x faster**)    | `49.60 ns` (🚀 **2.86x faster**)    | `97.95 ns` (✅ **1.45x faster**)    | `637.81 ns` (❌ *4.50x slower*)    |
| **`serialize_uncompressed`**             | `178.85 ns` (✅ **1.00x**)        | `268.51 ns` (❌ *1.50x slower*)   | `30.05 ns` (🚀 **5.95x faster**)    | `49.63 ns` (🚀 **3.60x faster**)    | `97.89 ns` (🚀 **1.83x faster**)    | `632.43 ns` (❌ *3.54x slower*)    |
| **`deserialize_compressed`**             | `148.22 us` (✅ **1.00x**)        | `257.87 us` (❌ *1.74x slower*)   | `46.56 ns` (🚀 **3183.51x faster**) | `93.83 ns` (🚀 **1579.60x faster**) | `210.91 ns` (🚀 **702.75x faster**) | `1.27 us` (🚀 **117.04x faster**)  |
| **`deserialize_compressed_unchecked`**   | `36.06 us` (✅ **1.00x**)         | `122.70 us` (❌ *3.40x slower*)   | `46.56 ns` (🚀 **774.45x faster**)  | `94.79 ns` (🚀 **380.37x faster**)  | `211.12 ns` (🚀 **170.78x faster**) | `1.27 us` (🚀 **28.49x faster**)   |
| **`deserialize_uncompressed`**           | `112.01 us` (✅ **1.00x**)        | `134.74 us` (❌ *1.20x slower*)   | `46.52 ns` (🚀 **2407.98x faster**) | `93.83 ns` (🚀 **1193.77x faster**) | `211.06 ns` (🚀 **530.70x faster**) | `1.27 us` (🚀 **87.90x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `231.55 ns` (✅ **1.00x**)        | `492.40 ns` (❌ *2.13x slower*)   | `46.51 ns` (🚀 **4.98x faster**)    | `93.82 ns` (🚀 **2.47x faster**)    | `211.00 ns` (✅ **1.10x faster**)   | `1.27 us` (❌ *5.47x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.23 s` (✅ **1.00x**)           | `6.64 s` (❌ *2.99x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.17 us` (✅ **1.00x**) | `35.60 us` (❌ *1.61x slower*)   | `121.81 us` (❌ *5.49x slower*)    |
| **`legendre_for_qr`**    | `12.48 us` (✅ **1.00x**) | `35.91 us` (❌ *2.88x slower*)   | `35.90 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.81 ns` (✅ **1.00x**)       | `108.86 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.83 ns` (✅ **1.00x**)       | `108.70 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.01 ns` (✅ **1.00x**) | `78.79 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.43 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

