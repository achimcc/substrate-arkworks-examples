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
|        | `220.64 us` (✅ **1.00x**)                 | `2.20 ms` (❌ *9.95x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.91 us` (❌ *3.80x slower*)     | `33.03 ns` (🚀 **39.09x faster**)  | `203.97 ns` (🚀 **6.33x faster**)  | `22.76 ns` (🚀 **56.73x faster**) | `9.76 ns` (🚀 **132.34x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.36 us` (✅ **1.00x**)          | `4.94 us` (❌ *3.62x slower*)     | `31.80 ns` (🚀 **42.88x faster**)  | `203.41 ns` (🚀 **6.70x faster**)  | `17.32 ns` (🚀 **78.75x faster**) | `9.73 ns` (🚀 **140.14x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `971.41 ns` (✅ **1.00x**)        | `3.64 us` (❌ *3.75x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.01 us` (✅ **1.00x**)          | `3.60 us` (❌ *3.57x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `645.57 ns` (✅ **1.00x**)        | `2.49 us` (❌ *3.85x slower*)     | `15.20 ns` (🚀 **42.46x faster**)  | `116.07 ns` (🚀 **5.56x faster**)  | `13.11 ns` (🚀 **49.24x faster**) | `6.24 ns` (🚀 **103.49x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `358.55 us` (✅ **1.00x**)        | `1.23 ms` (❌ *3.43x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `26.70 ns` (❌ *3.89x slower*)     | `132.13 ns` (❌ *19.23x slower*)   | `19.79 ns` (❌ *2.88x slower*)    | `6.87 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `317.87 ns` (❌ *7.28x slower*)    | `7.78 us` (❌ *178.26x slower*)    | `84.80 ns` (❌ *1.94x slower*)    | `43.64 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `283.12 ns` (❌ *7.31x slower*)    | `5.41 us` (❌ *139.72x slower*)    | `71.15 ns` (❌ *1.84x slower*)    | `38.74 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.27 us` (❌ *2.22x slower*)     | `29.54 us` (❌ *4.02x slower*)     | `15.70 us` (❌ *2.14x slower*)    | `7.34 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `674.71 ns` (❌ *10.69x slower*)   | `16.00 us` (❌ *253.53x slower*)   | `132.41 ns` (❌ *2.10x slower*)   | `63.10 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `641.38 ns` (❌ *6.67x slower*)    | `16.09 us` (❌ *167.26x slower*)   | `185.07 ns` (❌ *1.92x slower*)   | `96.21 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.08 ns` (✅ **1.00x**)        | `9.63 ns` (❌ *1.19x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.27 ns` (✅ **1.00x**)        | `13.37 ns` (❌ *1.44x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.52 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.07x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.42 ns` (✅ **1.00x**)        | `4.36 ns` (✅ **1.01x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `172.69 ns` (✅ **1.00x**)        | `234.59 ns` (❌ *1.36x slower*)   | `32.76 ns` (🚀 **5.27x faster**)    | `58.88 ns` (🚀 **2.93x faster**)     | `114.73 ns` (✅ **1.51x faster**)    | `743.40 ns` (❌ *4.30x slower*)    |
| **`serialize_uncompressed`**             | `230.75 ns` (✅ **1.00x**)        | `370.94 ns` (❌ *1.61x slower*)   | `32.62 ns` (🚀 **7.07x faster**)    | `61.30 ns` (🚀 **3.76x faster**)     | `115.87 ns` (🚀 **1.99x faster**)    | `781.71 ns` (❌ *3.39x slower*)    |
| **`deserialize_compressed`**             | `339.76 us` (✅ **1.00x**)        | `1.12 ms` (❌ *3.30x slower*)     | `54.63 ns` (🚀 **6218.81x faster**) | `120.82 ns` (🚀 **2812.23x faster**) | `240.09 ns` (🚀 **1415.16x faster**) | `1.49 us` (🚀 **228.03x faster**)  |
| **`deserialize_compressed_unchecked`**   | `75.21 us` (✅ **1.00x**)         | `211.98 us` (❌ *2.82x slower*)   | `53.72 ns` (🚀 **1400.09x faster**) | `127.49 ns` (🚀 **589.92x faster**)  | `234.66 ns` (🚀 **320.50x faster**)  | `1.50 us` (🚀 **49.98x faster**)   |
| **`deserialize_uncompressed`**           | `252.83 us` (✅ **1.00x**)        | `920.08 us` (❌ *3.64x slower*)   | `53.65 ns` (🚀 **4712.74x faster**) | `121.89 ns` (🚀 **2074.34x faster**) | `239.52 ns` (🚀 **1055.60x faster**) | `1.51 us` (🚀 **167.18x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `278.21 ns` (✅ **1.00x**)        | `559.69 ns` (❌ *2.01x slower*)   | `54.11 ns` (🚀 **5.14x faster**)    | `122.44 ns` (🚀 **2.27x faster**)    | `239.39 ns` (✅ **1.16x faster**)    | `1.50 us` (❌ *5.40x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.65 s` (✅ **1.00x**)           | `9.07 s` (❌ *3.42x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `33.57 us` (✅ **1.00x**) | `76.82 us` (❌ *2.29x slower*)   | `208.07 us` (❌ *6.20x slower*)    |
| **`legendre_for_qr`**    | `11.80 us` (✅ **1.00x**) | `34.58 us` (❌ *2.93x slower*)   | `35.43 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.65 ns` (✅ **1.00x**)        | `5.23 ns` (❌ *1.12x slower*)      |
| **`from_little-endian_bits`** | `70.96 ns` (✅ **1.00x**)       | `129.10 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `71.26 ns` (✅ **1.00x**)       | `129.96 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `5.00 ns` (✅ **1.00x**)        | `5.00 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.30 ns` (✅ **1.00x**)        | `5.41 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `4.63 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.02x faster**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.46 ns` (✅ **1.00x**) | `90.43 ns` (❌ *2.13x slower*)    |
| **`into_bigint`** | `25.20 ns` (✅ **1.00x**) | `47.77 ns` (❌ *1.90x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `10.54 ns` (✅ **1.00x**)                         | `21.86 ns` (❌ *2.07x slower*)                    | `1.01 ms` (❌ *95978.56x slower*)              | `1.35 ms` (❌ *127639.71x slower*)                      | `2.35 ms` (❌ *223323.56x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

