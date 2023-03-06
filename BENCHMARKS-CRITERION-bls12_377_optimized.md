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
|        | `196.13 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.51x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.11 us` (✅ **1.00x**)          | `4.18 us` (❌ *3.77x slower*)     | `28.52 ns` (🚀 **38.81x faster**)  | `173.13 ns` (🚀 **6.39x faster**)  | `19.03 ns` (🚀 **58.15x faster**) | `7.95 ns` (🚀 **139.29x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.21 us` (✅ **1.00x**)          | `4.09 us` (❌ *3.39x slower*)     | `25.79 ns` (🚀 **46.78x faster**)  | `172.06 ns` (🚀 **7.01x faster**)  | `14.49 ns` (🚀 **83.27x faster**) | `8.72 ns` (🚀 **138.33x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `842.70 ns` (✅ **1.00x**)        | `2.98 us` (❌ *3.54x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `860.49 ns` (✅ **1.00x**)        | `3.27 us` (❌ *3.80x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `560.30 ns` (✅ **1.00x**)        | `1.95 us` (❌ *3.47x slower*)     | `12.22 ns` (🚀 **45.85x faster**)  | `103.73 ns` (🚀 **5.40x faster**)  | `7.29 ns` (🚀 **76.85x faster**)  | `8.89 ns` (🚀 **62.99x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `308.01 us` (✅ **1.00x**)        | `1.03 ms` (❌ *3.35x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.14 ns` (❌ *3.78x slower*)     | `108.40 ns` (❌ *18.51x slower*)   | `16.36 ns` (❌ *2.79x slower*)    | `5.86 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.84 ns` (❌ *7.48x slower*)    | `6.54 us` (❌ *179.76x slower*)    | `66.93 ns` (❌ *1.84x slower*)    | `36.36 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `236.59 ns` (❌ *7.23x slower*)    | `4.66 us` (❌ *142.22x slower*)    | `57.49 ns` (❌ *1.76x slower*)    | `32.73 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.29 us` (❌ *2.22x slower*)     | `24.70 us` (❌ *4.12x slower*)     | `13.31 us` (❌ *2.22x slower*)    | `5.99 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `545.24 ns` (❌ *10.69x slower*)   | `13.54 us` (❌ *265.42x slower*)   | `112.21 ns` (❌ *2.20x slower*)   | `50.99 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `573.02 ns` (❌ *6.78x slower*)    | `13.07 us` (❌ *154.68x slower*)   | `149.64 ns` (❌ *1.77x slower*)   | `84.48 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.49 ns` (✅ **1.00x**)        | `7.57 ns` (❌ *1.17x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.47 ns` (✅ **1.00x**)        | `11.06 ns` (❌ *1.48x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.79 ns` (✅ **1.00x**)        | `3.86 ns` (✅ **1.02x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.58 ns` (✅ **1.00x**)        | `3.68 ns` (✅ **1.03x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `140.44 ns` (✅ **1.00x**)        | `210.77 ns` (❌ *1.50x slower*)   | `27.53 ns` (🚀 **5.10x faster**)    | `48.36 ns` (🚀 **2.90x faster**)    | `98.49 ns` (✅ **1.43x faster**)     | `605.83 ns` (❌ *4.31x slower*)    |
| **`serialize_uncompressed`**             | `189.14 ns` (✅ **1.00x**)        | `310.72 ns` (❌ *1.64x slower*)   | `26.74 ns` (🚀 **7.07x faster**)    | `48.16 ns` (🚀 **3.93x faster**)    | `96.46 ns` (🚀 **1.96x faster**)     | `668.96 ns` (❌ *3.54x slower*)    |
| **`deserialize_compressed`**             | `284.51 us` (✅ **1.00x**)        | `962.33 us` (❌ *3.38x slower*)   | `44.66 ns` (🚀 **6370.24x faster**) | `92.85 ns` (🚀 **3064.05x faster**) | `201.86 ns` (🚀 **1409.42x faster**) | `1.22 us` (🚀 **233.82x faster**)  |
| **`deserialize_compressed_unchecked`**   | `62.49 us` (✅ **1.00x**)         | `169.59 us` (❌ *2.71x slower*)   | `45.40 ns` (🚀 **1376.36x faster**) | `92.21 ns` (🚀 **677.66x faster**)  | `220.16 ns` (🚀 **283.84x faster**)  | `1.24 us` (🚀 **50.26x faster**)   |
| **`deserialize_uncompressed`**           | `211.25 us` (✅ **1.00x**)        | `821.59 us` (❌ *3.89x slower*)   | `44.67 ns` (🚀 **4728.81x faster**) | `93.75 ns` (🚀 **2253.44x faster**) | `197.40 ns` (🚀 **1070.17x faster**) | `1.25 us` (🚀 **169.45x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.23 ns` (✅ **1.00x**)        | `450.70 ns` (❌ *2.02x slower*)   | `44.90 ns` (🚀 **4.97x faster**)    | `95.94 ns` (🚀 **2.33x faster**)    | `199.05 ns` (✅ **1.12x faster**)    | `1.28 us` (❌ *5.75x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.25 s` (✅ **1.00x**)           | `7.77 s` (❌ *3.46x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.59 us` (✅ **1.00x**) | `61.36 us` (❌ *2.22x slower*)   | `171.54 us` (❌ *6.22x slower*)    |
| **`legendre_for_qr`**    | `9.60 us` (✅ **1.00x**)  | `28.10 us` (❌ *2.93x slower*)   | `29.67 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.84 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `60.73 ns` (✅ **1.00x**)       | `110.11 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `59.95 ns` (✅ **1.00x**)       | `104.77 ns` (❌ *1.75x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.17 ns` (✅ **1.02x slower**)    |
| **`equality`**                | `4.36 ns` (✅ **1.00x**)        | `4.60 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `3.94 ns` (✅ **1.01x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `34.96 ns` (✅ **1.00x**) | `79.12 ns` (❌ *2.26x slower*)    |
| **`into_bigint`** | `21.53 ns` (✅ **1.00x**) | `40.54 ns` (❌ *1.88x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `8.76 ns` (✅ **1.00x**)                          | `10.25 ns` (❌ *1.17x slower*)                    | `900.42 us` (❌ *102840.12x slower*)           | `1.14 ms` (❌ *129805.51x slower*)                      | `2.01 ms` (❌ *229210.52x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

