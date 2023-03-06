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
|        | `179.70 us` (✅ **1.00x**)        | `1.62 ms` (❌ *9.01x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.64 us` (❌ *3.26x slower*)     | `29.99 ns` (🚀 **37.22x faster**) | `175.29 ns` (🚀 **6.37x faster**)  | `19.08 ns` (🚀 **58.51x faster**) | `8.18 ns` (🚀 **136.38x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.15 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.19x slower*)     | `27.30 ns` (🚀 **42.23x faster**) | `167.38 ns` (🚀 **6.89x faster**)  | `14.48 ns` (🚀 **79.60x faster**) | `8.56 ns` (🚀 **134.63x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `817.29 ns` (✅ **1.00x**) | `2.62 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `839.07 ns` (✅ **1.00x**) | `2.65 us` (❌ *3.16x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `559.70 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.94x slower*)     | `12.98 ns` (🚀 **43.11x faster**) | `102.39 ns` (🚀 **5.47x faster**)  | `11.43 ns` (🚀 **48.95x faster**) | `5.43 ns` (🚀 **103.17x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `282.52 us` (✅ **1.00x**) | `867.49 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.89 ns` (❌ *3.85x slower*)    | `108.50 ns` (❌ *18.23x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `221.72 ns` (❌ *5.73x slower*)   | `5.74 us` (❌ *148.33x slower*)    | `70.24 ns` (❌ *1.82x slower*)    | `38.69 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.86 ns` (❌ *4.93x slower*)   | `4.05 us` (❌ *114.39x slower*)    | `58.32 ns` (❌ *1.65x slower*)    | `35.43 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.84 us` (❌ *2.17x slower*)    | `23.06 us` (❌ *3.61x slower*)     | `13.54 us` (❌ *2.12x slower*)    | `6.39 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.64 ns` (❌ *6.00x slower*)   | `11.78 us` (❌ *143.15x slower*)   | `107.24 ns` (❌ *1.30x slower*)   | `82.31 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `474.27 ns` (❌ *5.73x slower*)   | `11.76 us` (❌ *142.14x slower*)   | `156.36 ns` (❌ *1.89x slower*)   | `82.72 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.62 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `140.72 ns` (✅ **1.00x**) | `190.67 ns` (❌ *1.35x slower*)   | `29.90 ns` (🚀 **4.71x faster**)    | `49.63 ns` (🚀 **2.84x faster**)    | `100.97 ns` (✅ **1.39x faster**)   | `642.70 ns` (❌ *4.57x slower*)    |
| **`serialize_uncompressed`**             | `179.03 ns` (✅ **1.00x**) | `266.19 ns` (❌ *1.49x slower*)   | `29.83 ns` (🚀 **6.00x faster**)    | `49.56 ns` (🚀 **3.61x faster**)    | `100.99 ns` (✅ **1.77x faster**)   | `642.25 ns` (❌ *3.59x slower*)    |
| **`deserialize_compressed`**             | `117.34 us` (✅ **1.00x**) | `240.55 us` (❌ *2.05x slower*)   | `46.47 ns` (🚀 **2525.26x faster**) | `94.60 ns` (🚀 **1240.44x faster**) | `211.67 ns` (🚀 **554.38x faster**) | `1.26 us` (🚀 **92.98x faster**)   |
| **`deserialize_compressed_unchecked`**   | `35.82 us` (✅ **1.00x**)  | `121.85 us` (❌ *3.40x slower*)   | `46.45 ns` (🚀 **771.08x faster**)  | `95.77 ns` (🚀 **373.97x faster**)  | `211.67 ns` (🚀 **169.21x faster**) | `1.26 us` (🚀 **28.38x faster**)   |
| **`deserialize_uncompressed`**           | `81.60 us` (✅ **1.00x**)  | `118.26 us` (❌ *1.45x slower*)   | `46.42 ns` (🚀 **1757.94x faster**) | `94.67 ns` (🚀 **862.03x faster**)  | `211.25 ns` (🚀 **386.29x faster**) | `1.26 us` (🚀 **64.69x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `200.88 ns` (✅ **1.00x**) | `415.88 ns` (❌ *2.07x slower*)   | `46.42 ns` (🚀 **4.33x faster**)    | `94.66 ns` (🚀 **2.12x faster**)    | `211.26 ns` (✅ **1.05x slower**)   | `1.26 us` (❌ *6.28x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.21 s` (✅ **1.00x**)  | `6.59 s` (❌ *2.98x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.17 us` (✅ **1.00x**) | `35.45 us` (❌ *1.60x slower*)   | `120.98 us` (❌ *5.46x slower*)    |
| **`legendre_for_qr`**    | `12.31 us` (✅ **1.00x**) | `35.54 us` (❌ *2.89x slower*)   | `35.87 us` (❌ *2.91x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.84 ns` (✅ **1.00x**) | `110.04 ns` (❌ *1.81x slower*)    |
| **`from_big-endian_bits`**    | `60.91 ns` (✅ **1.00x**) | `110.04 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.73 ns` (✅ **1.06x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.10 ns` (✅ **1.00x**) | `79.01 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `41.42 ns` (❌ *1.92x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.03 ns` (✅ **1.00x**)                 | `189.98 us` (❌ *21036.36x slower*)      | `539.80 us` (❌ *59770.17x slower*)   | `967.75 us` (❌ *107156.17x slower*)           | `1.72 ms` (❌ *190304.54x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

