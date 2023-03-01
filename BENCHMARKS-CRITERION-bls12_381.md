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
|        | `180.61 us` (✅ **1.00x**)        | `1.63 ms` (❌ *9.01x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.65 us` (❌ *3.27x slower*)     | `29.64 ns` (🚀 **37.72x faster**) | `178.65 ns` (🚀 **6.26x faster**)  | `19.42 ns` (🚀 **57.56x faster**) | `8.17 ns` (🚀 **136.74x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.14x slower*)     | `27.62 ns` (🚀 **42.42x faster**) | `169.78 ns` (🚀 **6.90x faster**)  | `15.08 ns` (🚀 **77.68x faster**) | `8.56 ns` (🚀 **136.91x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `817.10 ns` (✅ **1.00x**) | `2.62 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `842.45 ns` (✅ **1.00x**) | `2.64 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `562.74 ns` (✅ **1.00x**) | `1.65 us` (❌ *2.94x slower*)     | `13.08 ns` (🚀 **43.01x faster**) | `100.28 ns` (🚀 **5.61x faster**)  | `7.63 ns` (🚀 **73.80x faster**)  | `5.44 ns` (🚀 **103.51x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.76 us` (✅ **1.00x**) | `863.30 us` (❌ *3.04x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.62 ns` (❌ *3.80x slower*)    | `104.53 ns` (❌ *17.57x slower*)   | `16.75 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.86 ns` (❌ *5.78x slower*)   | `5.73 us` (❌ *147.81x slower*)    | `70.26 ns` (❌ *1.81x slower*)    | `38.74 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.13 ns` (❌ *4.91x slower*)   | `4.03 us` (❌ *113.67x slower*)    | `58.19 ns` (❌ *1.64x slower*)    | `35.47 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.84 us` (❌ *2.15x slower*)    | `23.07 us` (❌ *3.59x slower*)     | `13.53 us` (❌ *2.11x slower*)    | `6.43 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.78 ns` (❌ *6.00x slower*)   | `11.83 us` (❌ *143.74x slower*)   | `107.22 ns` (❌ *1.30x slower*)   | `82.31 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `474.77 ns` (❌ *5.88x slower*)   | `11.64 us` (❌ *144.25x slower*)   | `155.80 ns` (❌ *1.93x slower*)   | `80.72 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `7.86 ns` (❌ *1.21x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `142.77 ns` (✅ **1.00x**) | `191.51 ns` (❌ *1.34x slower*)   | `30.17 ns` (🚀 **4.73x faster**)    | `49.51 ns` (🚀 **2.88x faster**)    | `97.83 ns` (✅ **1.46x faster**)    | `631.07 ns` (❌ *4.42x slower*)    |
| **`serialize_uncompressed`**             | `181.65 ns` (✅ **1.00x**) | `267.72 ns` (❌ *1.47x slower*)   | `30.05 ns` (🚀 **6.04x faster**)    | `49.35 ns` (🚀 **3.68x faster**)    | `97.80 ns` (🚀 **1.86x faster**)    | `630.93 ns` (❌ *3.47x slower*)    |
| **`deserialize_compressed`**             | `118.04 us` (✅ **1.00x**) | `241.52 us` (❌ *2.05x slower*)   | `46.52 ns` (🚀 **2537.22x faster**) | `93.66 ns` (🚀 **1260.31x faster**) | `207.26 ns` (🚀 **569.53x faster**) | `1.27 us` (🚀 **92.93x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.19 us` (✅ **1.00x**)  | `122.87 us` (❌ *3.39x slower*)   | `46.52 ns` (🚀 **778.05x faster**)  | `93.65 ns` (🚀 **386.47x faster**)  | `207.49 ns` (🚀 **174.44x faster**) | `1.27 us` (🚀 **28.50x faster**)   |
| **`deserialize_uncompressed`**           | `82.07 us` (✅ **1.00x**)  | `118.37 us` (❌ *1.44x slower*)   | `46.45 ns` (🚀 **1766.77x faster**) | `93.68 ns` (🚀 **876.15x faster**)  | `207.58 ns` (🚀 **395.38x faster**) | `1.27 us` (🚀 **64.60x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `200.97 ns` (✅ **1.00x**) | `424.90 ns` (❌ *2.11x slower*)   | `46.46 ns` (🚀 **4.33x faster**)    | `93.66 ns` (🚀 **2.15x faster**)    | `207.51 ns` (✅ **1.03x slower**)   | `1.27 us` (❌ *6.32x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.26 s` (✅ **1.00x**)  | `6.63 s` (❌ *2.94x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.09 us` (✅ **1.00x**) | `35.79 us` (❌ *1.62x slower*)   | `121.96 us` (❌ *5.52x slower*)    |
| **`legendre_for_qr`**    | `12.34 us` (✅ **1.00x**) | `35.70 us` (❌ *2.89x slower*)   | `35.85 us` (❌ *2.90x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.91 ns` (✅ **1.00x**) | `108.04 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.71 ns` (✅ **1.00x**) | `108.08 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.06 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.72 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.95 ns` (✅ **1.00x**) | `79.34 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.60 ns` (✅ **1.00x**) | `41.43 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

