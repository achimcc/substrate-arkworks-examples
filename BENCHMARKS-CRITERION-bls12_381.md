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
    - [pairing_for_bls12_381](#pairing_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `179.49 us` (✅ **1.00x**)        | `1.62 ms` (❌ *9.02x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.64 us` (❌ *3.26x slower*)     | `28.12 ns` (🚀 **39.71x faster**) | `175.60 ns` (🚀 **6.36x faster**)  | `19.06 ns` (🚀 **58.60x faster**) | `8.19 ns` (🚀 **136.34x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.15 us` (✅ **1.00x**)   | `3.69 us` (❌ *3.21x slower*)     | `27.16 ns` (🚀 **42.33x faster**) | `169.47 ns` (🚀 **6.78x faster**)  | `14.66 ns` (🚀 **78.44x faster**) | `8.59 ns` (🚀 **133.88x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `816.84 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.19x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `837.89 ns` (✅ **1.00x**) | `2.66 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `560.33 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.93x slower*)     | `12.99 ns` (🚀 **43.13x faster**) | `103.37 ns` (🚀 **5.42x faster**)  | `11.42 ns` (🚀 **49.07x faster**) | `5.48 ns` (🚀 **102.17x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `282.32 us` (✅ **1.00x**) | `867.02 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.61 ns` (❌ *3.80x slower*)    | `109.10 ns` (❌ *18.35x slower*)   | `16.75 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.90 ns` (❌ *5.79x slower*)   | `5.75 us` (❌ *148.60x slower*)    | `70.23 ns` (❌ *1.82x slower*)    | `38.68 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.22 ns` (❌ *4.93x slower*)   | `4.06 us` (❌ *114.89x slower*)    | `58.37 ns` (❌ *1.65x slower*)    | `35.37 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.82 us` (❌ *2.16x slower*)    | `23.09 us` (❌ *3.61x slower*)     | `13.53 us` (❌ *2.12x slower*)    | `6.39 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `494.55 ns` (❌ *5.99x slower*)   | `11.74 us` (❌ *142.29x slower*)   | `107.38 ns` (❌ *1.30x slower*)   | `82.52 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `474.32 ns` (❌ *5.73x slower*)   | `11.67 us` (❌ *140.91x slower*)   | `156.91 ns` (❌ *1.89x slower*)   | `82.82 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `7.88 ns` (❌ *1.21x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.71 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `140.77 ns` (✅ **1.00x**) | `190.35 ns` (❌ *1.35x slower*)   | `29.87 ns` (🚀 **4.71x faster**)    | `49.61 ns` (🚀 **2.84x faster**)    | `101.35 ns` (✅ **1.39x faster**)   | `642.92 ns` (❌ *4.57x slower*)    |
| **`serialize_uncompressed`**             | `179.22 ns` (✅ **1.00x**) | `266.81 ns` (❌ *1.49x slower*)   | `29.81 ns` (🚀 **6.01x faster**)    | `49.56 ns` (🚀 **3.62x faster**)    | `101.36 ns` (✅ **1.77x faster**)   | `642.54 ns` (❌ *3.59x slower*)    |
| **`deserialize_compressed`**             | `117.38 us` (✅ **1.00x**) | `240.51 us` (❌ *2.05x slower*)   | `46.27 ns` (🚀 **2536.57x faster**) | `95.78 ns` (🚀 **1225.47x faster**) | `206.71 ns` (🚀 **567.82x faster**) | `1.27 us` (🚀 **92.38x faster**)   |
| **`deserialize_compressed_unchecked`**   | `35.79 us` (✅ **1.00x**)  | `121.96 us` (❌ *3.41x slower*)   | `46.28 ns` (🚀 **773.46x faster**)  | `95.78 ns` (🚀 **373.70x faster**)  | `206.91 ns` (🚀 **172.99x faster**) | `1.27 us` (🚀 **28.17x faster**)   |
| **`deserialize_uncompressed`**           | `81.58 us` (✅ **1.00x**)  | `118.25 us` (❌ *1.45x slower*)   | `44.78 ns` (🚀 **1821.79x faster**) | `95.51 ns` (🚀 **854.14x faster**)  | `206.43 ns` (🚀 **395.18x faster**) | `1.27 us` (🚀 **64.23x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.09 ns` (✅ **1.00x**) | `415.78 ns` (❌ *2.07x slower*)   | `44.78 ns` (🚀 **4.49x faster**)    | `95.53 ns` (🚀 **2.10x faster**)    | `206.43 ns` (✅ **1.03x slower**)   | `1.27 us` (❌ *6.32x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.23 s` (✅ **1.00x**)  | `6.60 s` (❌ *2.95x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.14 us` (✅ **1.00x**) | `35.40 us` (❌ *1.60x slower*)   | `121.08 us` (❌ *5.47x slower*)    |
| **`legendre_for_qr`**    | `12.33 us` (✅ **1.00x**) | `35.51 us` (❌ *2.88x slower*)   | `35.98 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.07 ns` (✅ **1.00x**) | `109.94 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `60.80 ns` (✅ **1.00x**) | `109.75 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.05 ns` (✅ **1.00x**) | `78.99 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.67 ns` (✅ **1.00x**) | `41.41 ns` (❌ *1.91x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.03 ns` (✅ **1.00x**)                 | `189.71 us` (❌ *21010.57x slower*)      | `536.80 us` (❌ *59451.60x slower*)   | `967.42 us` (❌ *107144.39x slower*)           | `1.72 ms` (❌ *189960.94x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

