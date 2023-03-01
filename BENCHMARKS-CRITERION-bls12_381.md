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
|        | `182.45 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.89x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.13 us` (✅ **1.00x**)   | `3.63 us` (❌ *3.21x slower*)     | `28.19 ns` (🚀 **40.18x faster**) | `181.28 ns` (🚀 **6.25x faster**)  | `19.50 ns` (🚀 **58.07x faster**) | `7.23 ns` (🚀 **156.75x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `3.69 us` (❌ *3.16x slower*)     | `27.53 ns` (🚀 **42.45x faster**) | `169.18 ns` (🚀 **6.91x faster**)  | `13.27 ns` (🚀 **88.04x faster**) | `7.56 ns` (🚀 **154.50x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `735.34 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.55x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `858.39 ns` (✅ **1.00x**) | `2.65 us` (❌ *3.09x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `587.90 ns` (✅ **1.00x**) | `1.45 us` (❌ *2.47x slower*)     | `13.07 ns` (🚀 **44.98x faster**) | `101.72 ns` (🚀 **5.78x faster**)  | `6.73 ns` (🚀 **87.39x faster**)  | `5.43 ns` (🚀 **108.22x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `253.97 us` (✅ **1.00x**) | `868.39 us` (❌ *3.42x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `20.20 ns` (❌ *3.40x slower*)    | `92.87 ns` (❌ *15.64x slower*)    | `17.24 ns` (❌ *2.90x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.68 ns` (❌ *6.54x slower*)   | `5.74 us` (❌ *167.84x slower*)    | `70.27 ns` (❌ *2.06x slower*)    | `34.18 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `153.57 ns` (❌ *4.91x slower*)   | `4.04 us` (❌ *129.34x slower*)    | `51.42 ns` (❌ *1.65x slower*)    | `31.25 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `12.22 us` (❌ *1.90x slower*)    | `23.04 us` (❌ *3.58x slower*)     | `11.95 us` (❌ *1.86x slower*)    | `6.43 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.71 ns` (❌ *6.03x slower*)   | `11.76 us` (❌ *143.51x slower*)   | `94.58 ns` (❌ *1.15x slower*)    | `81.92 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `475.19 ns` (❌ *5.91x slower*)   | `11.68 us` (❌ *145.34x slower*)   | `155.83 ns` (❌ *1.94x slower*)   | `80.34 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `5.76 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.36x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `6.90 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.54x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.29 ns` (✅ **1.00x**) | `3.30 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `124.51 ns` (✅ **1.00x**) | `169.75 ns` (❌ *1.36x slower*)   | `26.28 ns` (🚀 **4.74x faster**)    | `43.60 ns` (🚀 **2.86x faster**)    | `97.97 ns` (✅ **1.27x faster**)    | `555.83 ns` (❌ *4.46x slower*)    |
| **`serialize_uncompressed`**             | `158.53 ns` (✅ **1.00x**) | `237.29 ns` (❌ *1.50x slower*)   | `29.81 ns` (🚀 **5.32x faster**)    | `49.25 ns` (🚀 **3.22x faster**)    | `97.89 ns` (✅ **1.62x faster**)    | `631.15 ns` (❌ *3.98x slower*)    |
| **`deserialize_compressed`**             | `118.15 us` (✅ **1.00x**) | `241.45 us` (❌ *2.04x slower*)   | `40.92 ns` (🚀 **2887.18x faster**) | `94.58 ns` (🚀 **1249.20x faster**) | `185.12 ns` (🚀 **638.22x faster**) | `1.27 us` (🚀 **92.80x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.15 us` (✅ **1.00x**)  | `122.75 us` (❌ *3.40x slower*)   | `46.53 ns` (🚀 **777.01x faster**)  | `83.49 ns` (🚀 **433.01x faster**)  | `184.74 ns` (🚀 **195.70x faster**) | `1.28 us` (🚀 **28.25x faster**)   |
| **`deserialize_uncompressed`**           | `82.10 us` (✅ **1.00x**)  | `118.25 us` (❌ *1.44x slower*)   | `46.48 ns` (🚀 **1766.55x faster**) | `94.58 ns` (🚀 **868.06x faster**)  | `209.40 ns` (🚀 **392.08x faster**) | `1.28 us` (🚀 **64.18x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `199.59 ns` (✅ **1.00x**) | `413.99 ns` (❌ *2.07x slower*)   | `46.48 ns` (🚀 **4.29x faster**)    | `94.57 ns` (🚀 **2.11x faster**)    | `209.57 ns` (✅ **1.05x slower**)   | `1.27 us` (❌ *6.38x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.02 s` (✅ **1.00x**)  | `5.88 s` (❌ *2.92x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.13 us` (✅ **1.00x**) | `31.56 us` (❌ *1.43x slower*)   | `121.86 us` (❌ *5.51x slower*)    |
| **`legendre_for_qr`**    | `12.31 us` (✅ **1.00x**) | `31.53 us` (❌ *2.56x slower*)   | `35.85 us` (❌ *2.91x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.86 ns` (✅ **1.00x**) | `107.83 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.94 ns` (✅ **1.00x**) | `107.86 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `3.80 ns` (✅ **1.07x faster**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `3.53 ns` (✅ **1.11x faster**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.05 ns` (✅ **1.00x**) | `78.76 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `36.55 ns` (❌ *1.70x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

