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
|        | `180.87 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.95x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.63 us` (❌ *3.25x slower*)     | `26.90 ns` (🚀 **41.49x faster**) | `177.57 ns` (🚀 **6.29x faster**)  | `19.35 ns` (🚀 **57.68x faster**) | `8.18 ns` (🚀 **136.54x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.18x slower*)     | `28.18 ns` (🚀 **41.00x faster**) | `172.55 ns` (🚀 **6.70x faster**)  | `14.53 ns` (🚀 **79.54x faster**) | `8.56 ns` (🚀 **134.98x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `821.46 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `839.97 ns` (✅ **1.00x**) | `2.64 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `562.79 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.92x slower*)     | `13.03 ns` (🚀 **43.19x faster**) | `101.62 ns` (🚀 **5.54x faster**)  | `11.41 ns` (🚀 **49.32x faster**) | `5.49 ns` (🚀 **102.57x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.90 us` (✅ **1.00x**) | `867.47 us` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.97 ns` (❌ *3.86x slower*)    | `102.74 ns` (❌ *17.26x slower*)   | `16.77 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `224.56 ns` (❌ *5.76x slower*)   | `5.74 us` (❌ *147.32x slower*)    | `70.38 ns` (❌ *1.81x slower*)    | `38.96 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.43 ns` (❌ *4.92x slower*)   | `4.03 us` (❌ *113.53x slower*)    | `58.51 ns` (❌ *1.65x slower*)    | `35.46 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.83 us` (❌ *2.14x slower*)    | `23.03 us` (❌ *3.56x slower*)     | `13.54 us` (❌ *2.10x slower*)    | `6.46 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `495.79 ns` (❌ *6.01x slower*)   | `11.72 us` (❌ *142.11x slower*)   | `107.04 ns` (❌ *1.30x slower*)   | `82.51 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `474.74 ns` (❌ *5.88x slower*)   | `11.63 us` (❌ *143.90x slower*)   | `155.47 ns` (❌ *1.92x slower*)   | `80.80 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `140.14 ns` (✅ **1.00x**) | `191.50 ns` (❌ *1.37x slower*)   | `30.10 ns` (🚀 **4.66x faster**)    | `49.52 ns` (🚀 **2.83x faster**)    | `97.93 ns` (✅ **1.43x faster**)    | `632.32 ns` (❌ *4.51x slower*)    |
| **`serialize_uncompressed`**             | `180.56 ns` (✅ **1.00x**) | `266.89 ns` (❌ *1.48x slower*)   | `30.03 ns` (🚀 **6.01x faster**)    | `49.57 ns` (🚀 **3.64x faster**)    | `97.95 ns` (🚀 **1.84x faster**)    | `631.85 ns` (❌ *3.50x slower*)    |
| **`deserialize_compressed`**             | `118.14 us` (✅ **1.00x**) | `241.90 us` (❌ *2.05x slower*)   | `46.56 ns` (🚀 **2537.40x faster**) | `94.46 ns` (🚀 **1250.74x faster**) | `207.65 ns` (🚀 **568.95x faster**) | `1.26 us` (🚀 **93.97x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.16 us` (✅ **1.00x**)  | `122.97 us` (❌ *3.40x slower*)   | `46.57 ns` (🚀 **776.36x faster**)  | `94.62 ns` (🚀 **382.10x faster**)  | `206.26 ns` (🚀 **175.29x faster**) | `1.26 us` (🚀 **28.61x faster**)   |
| **`deserialize_uncompressed`**           | `81.92 us` (✅ **1.00x**)  | `118.47 us` (❌ *1.45x slower*)   | `46.53 ns` (🚀 **1760.66x faster**) | `94.41 ns` (🚀 **867.71x faster**)  | `207.53 ns` (🚀 **394.74x faster**) | `1.26 us` (🚀 **65.09x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `200.87 ns` (✅ **1.00x**) | `416.38 ns` (❌ *2.07x slower*)   | `46.52 ns` (🚀 **4.32x faster**)    | `93.82 ns` (🚀 **2.14x faster**)    | `206.06 ns` (✅ **1.03x slower**)   | `1.26 us` (❌ *6.29x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.24 s` (✅ **1.00x**)  | `6.64 s` (❌ *2.96x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.07 us` (✅ **1.00x**) | `35.76 us` (❌ *1.62x slower*)   | `121.99 us` (❌ *5.53x slower*)    |
| **`legendre_for_qr`**    | `12.35 us` (✅ **1.00x**) | `35.69 us` (❌ *2.89x slower*)   | `35.78 us` (❌ *2.90x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.94 ns` (✅ **1.00x**) | `108.06 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.92 ns` (✅ **1.00x**) | `108.03 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.17 ns` (✅ **1.00x**) | `79.28 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.59 ns` (✅ **1.00x**) | `41.45 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

