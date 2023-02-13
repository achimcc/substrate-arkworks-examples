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
|        | `180.74 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.97x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.64 us` (❌ *3.24x slower*)     | `26.80 ns` (🚀 **41.83x faster**) | `179.44 ns` (🚀 **6.25x faster**)  | `19.39 ns` (🚀 **57.79x faster**) | `8.20 ns` (🚀 **136.73x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.69 us` (❌ *3.19x slower*)     | `27.55 ns` (🚀 **42.00x faster**) | `170.34 ns` (🚀 **6.79x faster**)  | `14.69 ns` (🚀 **78.78x faster**) | `8.56 ns` (🚀 **135.21x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `824.60 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `847.17 ns` (✅ **1.00x**) | `2.64 us` (❌ *3.12x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `559.85 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.93x slower*)     | `12.97 ns` (🚀 **43.16x faster**) | `100.33 ns` (🚀 **5.58x faster**)  | `7.62 ns` (🚀 **73.42x faster**)  | `5.44 ns` (🚀 **102.86x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.23 us` (✅ **1.00x**) | `868.43 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.87 ns` (❌ *3.85x slower*)    | `101.21 ns` (❌ *17.02x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.79 ns` (❌ *5.77x slower*)   | `5.75 us` (❌ *148.21x slower*)    | `70.30 ns` (❌ *1.81x slower*)    | `38.78 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.74 ns` (❌ *4.95x slower*)   | `4.03 us` (❌ *113.39x slower*)    | `58.43 ns` (❌ *1.64x slower*)    | `35.54 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.84 us` (❌ *2.15x slower*)    | `23.03 us` (❌ *3.57x slower*)     | `13.55 us` (❌ *2.10x slower*)    | `6.44 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `494.30 ns` (❌ *6.05x slower*)   | `11.71 us` (❌ *143.47x slower*)   | `107.19 ns` (❌ *1.31x slower*)   | `81.64 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `475.76 ns` (❌ *5.88x slower*)   | `11.63 us` (❌ *143.76x slower*)   | `156.93 ns` (❌ *1.94x slower*)   | `80.92 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.30 ns` (❌ *1.31x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `139.74 ns` (✅ **1.00x**) | `190.15 ns` (❌ *1.36x slower*)   | `30.11 ns` (🚀 **4.64x faster**)    | `49.59 ns` (🚀 **2.82x faster**)    | `99.70 ns` (✅ **1.40x faster**)    | `635.95 ns` (❌ *4.55x slower*)    |
| **`serialize_uncompressed`**             | `178.86 ns` (✅ **1.00x**) | `270.59 ns` (❌ *1.51x slower*)   | `30.03 ns` (🚀 **5.96x faster**)    | `49.50 ns` (🚀 **3.61x faster**)    | `99.71 ns` (✅ **1.79x faster**)    | `631.32 ns` (❌ *3.53x slower*)    |
| **`deserialize_compressed`**             | `117.91 us` (✅ **1.00x**) | `241.59 us` (❌ *2.05x slower*)   | `46.48 ns` (🚀 **2536.59x faster**) | `95.62 ns` (🚀 **1233.14x faster**) | `207.17 ns` (🚀 **569.14x faster**) | `1.28 us` (🚀 **91.91x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.34 us` (✅ **1.00x**)  | `123.18 us` (❌ *3.39x slower*)   | `46.49 ns` (🚀 **781.80x faster**)  | `95.65 ns` (🚀 **379.97x faster**)  | `206.24 ns` (🚀 **176.22x faster**) | `1.28 us` (🚀 **28.33x faster**)   |
| **`deserialize_uncompressed`**           | `81.65 us` (✅ **1.00x**)  | `118.14 us` (❌ *1.45x slower*)   | `46.42 ns` (🚀 **1758.95x faster**) | `95.93 ns` (🚀 **851.11x faster**)  | `206.37 ns` (🚀 **395.62x faster**) | `1.28 us` (🚀 **63.74x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `200.88 ns` (✅ **1.00x**) | `418.05 ns` (❌ *2.08x slower*)   | `46.42 ns` (🚀 **4.33x faster**)    | `95.57 ns` (🚀 **2.10x faster**)    | `206.33 ns` (✅ **1.03x slower**)   | `1.28 us` (❌ *6.38x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.25 s` (✅ **1.00x**)  | `6.70 s` (❌ *2.98x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.09 us` (✅ **1.00x**) | `35.97 us` (❌ *1.63x slower*)   | `122.32 us` (❌ *5.54x slower*)    |
| **`legendre_for_qr`**    | `12.45 us` (✅ **1.00x**) | `35.83 us` (❌ *2.88x slower*)   | `35.96 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.99 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.65 ns` (✅ **1.00x**) | `108.77 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.69 ns` (✅ **1.00x**) | `108.77 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.10 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.87 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.04x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.10 ns` (✅ **1.00x**) | `79.05 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.58 ns` (✅ **1.00x**) | `41.44 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

