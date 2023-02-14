# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381](#sample_bls12_381)
    - [arithmetic_for_bls12_381](#arithmetic_for_bls12_381)
    - [serialization_for_bls12_381](#serialization_for_bls12_381)
    - [msm_for_bls12_381](#msm_for_bls12_381)
    - [squareroot_for_bls12_381](#squareroot_for_bls12_381)
    - [bitwise_operations_for_bls12_381](#bitwise_operations_for_bls12_381)
    - [conversions_for_bls12_381](#conversions_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `180.84 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.97x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.66 us` (❌ *3.26x slower*)     | `26.83 ns` (🚀 **41.92x faster**) | `177.57 ns` (🚀 **6.33x faster**)  | `19.41 ns` (🚀 **57.95x faster**) | `8.20 ns` (🚀 **137.12x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.69 us` (❌ *3.19x slower*)     | `28.09 ns` (🚀 **41.26x faster**) | `168.93 ns` (🚀 **6.86x faster**)  | `14.66 ns` (🚀 **79.06x faster**) | `8.55 ns` (🚀 **135.49x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `823.02 ns` (✅ **1.00x**) | `2.62 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `845.31 ns` (✅ **1.00x**) | `2.66 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `560.99 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.93x slower*)     | `12.97 ns` (🚀 **43.25x faster**) | `101.11 ns` (🚀 **5.55x faster**)  | `7.61 ns` (🚀 **73.69x faster**)  | `5.42 ns` (🚀 **103.46x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.06 us` (✅ **1.00x**) | `868.98 us` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.87 ns` (❌ *3.84x slower*)    | `101.43 ns` (❌ *17.05x slower*)   | `17.05 ns` (❌ *2.87x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `224.07 ns` (❌ *5.79x slower*)   | `5.73 us` (❌ *147.90x slower*)    | `69.93 ns` (❌ *1.81x slower*)    | `38.72 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.77 ns` (❌ *4.95x slower*)   | `4.02 us` (❌ *113.24x slower*)    | `58.10 ns` (❌ *1.64x slower*)    | `35.49 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.84 us` (❌ *2.15x slower*)    | `23.04 us` (❌ *3.58x slower*)     | `13.57 us` (❌ *2.11x slower*)    | `6.44 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `494.67 ns` (❌ *6.08x slower*)   | `11.72 us` (❌ *144.18x slower*)   | `107.06 ns` (❌ *1.32x slower*)   | `81.31 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `475.49 ns` (❌ *5.88x slower*)   | `11.65 us` (❌ *144.01x slower*)   | `157.50 ns` (❌ *1.95x slower*)   | `80.90 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.99 ns` (❌ *1.22x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.80 ns` (❌ *1.38x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `139.73 ns` (✅ **1.00x**) | `191.90 ns` (❌ *1.37x slower*)   | `29.89 ns` (🚀 **4.67x faster**)    | `49.61 ns` (🚀 **2.82x faster**)    | `97.64 ns` (✅ **1.43x faster**)    | `634.36 ns` (❌ *4.54x slower*)    |
| **`serialize_uncompressed`**             | `179.76 ns` (✅ **1.00x**) | `267.81 ns` (❌ *1.49x slower*)   | `29.81 ns` (🚀 **6.03x faster**)    | `49.51 ns` (🚀 **3.63x faster**)    | `97.64 ns` (🚀 **1.84x faster**)    | `629.99 ns` (❌ *3.50x slower*)    |
| **`deserialize_compressed`**             | `117.95 us` (✅ **1.00x**) | `242.05 us` (❌ *2.05x slower*)   | `46.47 ns` (🚀 **2538.36x faster**) | `95.10 ns` (🚀 **1240.30x faster**) | `206.48 ns` (🚀 **571.26x faster**) | `1.28 us` (🚀 **92.27x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.36 us` (✅ **1.00x**)  | `123.23 us` (❌ *3.39x slower*)   | `46.50 ns` (🚀 **781.84x faster**)  | `93.75 ns` (🚀 **387.80x faster**)  | `206.46 ns` (🚀 **176.10x faster**) | `1.27 us` (🚀 **28.68x faster**)   |
| **`deserialize_uncompressed`**           | `81.67 us` (✅ **1.00x**)  | `118.55 us` (❌ *1.45x slower*)   | `46.46 ns` (🚀 **1757.97x faster**) | `95.01 ns` (🚀 **859.60x faster**)  | `208.01 ns` (🚀 **392.61x faster**) | `1.27 us` (🚀 **64.42x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.52 ns` (✅ **1.00x**) | `419.62 ns` (❌ *2.08x slower*)   | `46.47 ns` (🚀 **4.34x faster**)    | `95.06 ns` (🚀 **2.12x faster**)    | `207.80 ns` (✅ **1.03x slower**)   | `1.28 us` (❌ *6.34x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.26 s` (✅ **1.00x**)  | `6.68 s` (❌ *2.95x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.08 us` (✅ **1.00x**) | `35.97 us` (❌ *1.63x slower*)   | `122.37 us` (❌ *5.54x slower*)    |
| **`legendre_for_qr`**    | `12.45 us` (✅ **1.00x**) | `35.80 us` (❌ *2.88x slower*)   | `35.96 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.94 ns` (✅ **1.00x**) | `110.45 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `60.82 ns` (✅ **1.00x**) | `110.72 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.06x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.08 ns` (✅ **1.00x**) | `78.97 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.58 ns` (✅ **1.00x**) | `41.45 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

