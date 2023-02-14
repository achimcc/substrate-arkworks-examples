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
|        | `281.62 us` (✅ **1.00x**)        | `2.30 ms` (❌ *8.17x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.81 us` (✅ **1.00x**)   | `4.81 us` (❌ *2.67x slower*)   | `34.08 ns` (🚀 **52.97x faster**) | `219.61 ns` (🚀 **8.22x faster**)  | `23.61 ns` (🚀 **76.47x faster**) | `10.69 ns` (🚀 **168.90x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.80 us` (✅ **1.00x**)   | `4.88 us` (❌ *2.71x slower*)   | `34.82 ns` (🚀 **51.75x faster**) | `207.99 ns` (🚀 **8.66x faster**)  | `19.34 ns` (🚀 **93.18x faster**) | `11.26 ns` (🚀 **160.06x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.28 us` (✅ **1.00x**)   | `3.43 us` (❌ *2.67x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.32 us` (✅ **1.00x**)   | `3.51 us` (❌ *2.66x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `848.57 ns` (✅ **1.00x**) | `2.34 us` (❌ *2.76x slower*)   | `16.53 ns` (🚀 **51.34x faster**) | `124.94 ns` (🚀 **6.79x faster**)  | `9.70 ns` (🚀 **87.52x faster**)  | `6.41 ns` (🚀 **132.43x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `436.78 us` (✅ **1.00x**) | `1.20 ms` (❌ *2.76x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.25 ns` (❌ *3.43x slower*)    | `125.53 ns` (❌ *15.82x slower*)   | `21.85 ns` (❌ *2.75x slower*)    | `7.93 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `285.48 ns` (❌ *5.82x slower*)   | `7.13 us` (❌ *145.18x slower*)    | `85.73 ns` (❌ *1.75x slower*)    | `49.09 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `247.38 ns` (❌ *5.95x slower*)   | `4.98 us` (❌ *119.83x slower*)    | `74.93 ns` (❌ *1.80x slower*)    | `41.58 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.17 us` (❌ *2.29x slower*)    | `28.89 us` (❌ *4.09x slower*)     | `15.65 us` (❌ *2.22x slower*)    | `7.06 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `606.66 ns` (❌ *6.06x slower*)   | `14.94 us` (❌ *149.26x slower*)   | `140.13 ns` (❌ *1.40x slower*)   | `100.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `593.42 ns` (❌ *5.76x slower*)   | `14.54 us` (❌ *141.18x slower*)   | `217.08 ns` (❌ *2.11x slower*)   | `102.96 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.91 ns` (✅ **1.00x**)  | `11.38 ns` (❌ *1.44x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.39 ns` (✅ **1.00x**) | `13.97 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.40 ns` (✅ **1.00x**)  | `4.62 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.12 ns` (✅ **1.00x**)  | `4.18 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `189.25 ns` (✅ **1.00x**) | `264.69 ns` (❌ *1.40x slower*)   | `38.48 ns` (🚀 **4.92x faster**)    | `60.07 ns` (🚀 **3.15x faster**)     | `116.88 ns` (✅ **1.62x faster**)   | `746.75 ns` (❌ *3.95x slower*)    |
| **`serialize_uncompressed`**             | `246.69 ns` (✅ **1.00x**) | `362.16 ns` (❌ *1.47x slower*)   | `38.16 ns` (🚀 **6.47x faster**)    | `60.33 ns` (🚀 **4.09x faster**)     | `116.27 ns` (🚀 **2.12x faster**)   | `743.25 ns` (❌ *3.01x slower*)    |
| **`deserialize_compressed`**             | `185.24 us` (✅ **1.00x**) | `356.50 us` (❌ *1.92x slower*)   | `60.48 ns` (🚀 **3063.03x faster**) | `112.36 ns` (🚀 **1648.60x faster**) | `281.44 ns` (🚀 **658.20x faster**) | `1.65 us` (🚀 **112.59x faster**)  |
| **`deserialize_compressed_unchecked`**   | `57.76 us` (✅ **1.00x**)  | `189.32 us` (❌ *3.28x slower*)   | `61.01 ns` (🚀 **946.66x faster**)  | `111.14 ns` (🚀 **519.71x faster**)  | `280.54 ns` (🚀 **205.89x faster**) | `1.69 us` (🚀 **34.19x faster**)   |
| **`deserialize_uncompressed`**           | `125.19 us` (✅ **1.00x**) | `162.01 us` (❌ *1.29x slower*)   | `60.74 ns` (🚀 **2061.09x faster**) | `113.25 ns` (🚀 **1105.46x faster**) | `283.03 ns` (🚀 **442.34x faster**) | `1.66 us` (🚀 **75.47x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `274.77 ns` (✅ **1.00x**) | `566.11 ns` (❌ *2.06x slower*)   | `60.65 ns` (🚀 **4.53x faster**)    | `110.59 ns` (🚀 **2.48x faster**)    | `286.00 ns` (✅ **1.04x slower**)   | `1.68 us` (❌ *6.10x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `3.42 s` (✅ **1.00x**)  | `8.87 s` (❌ *2.59x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.58 us` (✅ **1.00x**) | `56.28 us` (❌ *2.04x slower*)   | `190.57 us` (❌ *6.91x slower*)    |
| **`legendre_for_qr`**    | `15.52 us` (✅ **1.00x**) | `57.97 us` (❌ *3.73x slower*)   | `57.11 us` (❌ *3.68x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.39 ns` (✅ **1.00x**)  | `4.68 ns` (✅ **1.07x slower**)    |
| **`from_little-endian_bits`** | `72.81 ns` (✅ **1.00x**) | `130.87 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `72.71 ns` (✅ **1.00x**) | `132.08 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.55 ns` (✅ **1.00x**)  | `4.91 ns` (✅ **1.08x slower**)    |
| **`equality`**                | `4.90 ns` (✅ **1.00x**)  | `5.63 ns` (❌ *1.15x slower*)      |
| **`is_zero`**                 | `4.24 ns` (✅ **1.00x**)  | `4.62 ns` (✅ **1.09x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.12 ns` (✅ **1.00x**) | `99.53 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `26.80 ns` (✅ **1.00x**) | `49.37 ns` (❌ *1.84x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

