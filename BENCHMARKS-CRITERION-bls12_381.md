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
|        | `181.47 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.93x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.63 us` (❌ *3.24x slower*)     | `26.55 ns` (🚀 **42.21x faster**) | `179.39 ns` (🚀 **6.25x faster**)  | `19.42 ns` (🚀 **57.71x faster**) | `8.20 ns` (🚀 **136.76x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.18x slower*)     | `28.11 ns` (🚀 **41.15x faster**) | `169.58 ns` (🚀 **6.82x faster**)  | `14.68 ns` (🚀 **78.79x faster**) | `8.55 ns` (🚀 **135.25x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `820.14 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `842.68 ns` (✅ **1.00x**) | `2.64 us` (❌ *3.13x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `559.37 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.94x slower*)     | `13.01 ns` (🚀 **42.99x faster**) | `99.87 ns` (🚀 **5.60x faster**)   | `7.62 ns` (🚀 **73.43x faster**)  | `5.43 ns` (🚀 **103.09x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.16 us` (✅ **1.00x**) | `868.88 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.85 ns` (❌ *3.84x slower*)    | `101.14 ns` (❌ *17.01x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.93 ns` (❌ *5.79x slower*)   | `5.73 us` (❌ *148.11x slower*)    | `70.25 ns` (❌ *1.82x slower*)    | `38.71 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.79 ns` (❌ *4.95x slower*)   | `4.02 us` (❌ *113.16x slower*)    | `58.44 ns` (❌ *1.65x slower*)    | `35.52 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.83 us` (❌ *2.15x slower*)    | `23.03 us` (❌ *3.58x slower*)     | `13.54 us` (❌ *2.10x slower*)    | `6.44 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.93 ns` (❌ *6.04x slower*)   | `11.69 us` (❌ *142.88x slower*)   | `107.12 ns` (❌ *1.31x slower*)   | `81.82 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `476.73 ns` (❌ *5.89x slower*)   | `11.62 us` (❌ *143.56x slower*)   | `156.94 ns` (❌ *1.94x slower*)   | `80.93 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `139.64 ns` (✅ **1.00x**) | `191.01 ns` (❌ *1.37x slower*)   | `30.10 ns` (🚀 **4.64x faster**)    | `49.58 ns` (🚀 **2.82x faster**)    | `98.05 ns` (✅ **1.42x faster**)    | `635.97 ns` (❌ *4.55x slower*)    |
| **`serialize_uncompressed`**             | `178.80 ns` (✅ **1.00x**) | `267.04 ns` (❌ *1.49x slower*)   | `30.02 ns` (🚀 **5.96x faster**)    | `49.48 ns` (🚀 **3.61x faster**)    | `98.06 ns` (🚀 **1.82x faster**)    | `631.31 ns` (❌ *3.53x slower*)    |
| **`deserialize_compressed`**             | `117.90 us` (✅ **1.00x**) | `241.62 us` (❌ *2.05x slower*)   | `46.51 ns` (🚀 **2535.05x faster**) | `95.74 ns` (🚀 **1231.51x faster**) | `205.98 ns` (🚀 **572.40x faster**) | `1.28 us` (🚀 **92.07x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.33 us` (✅ **1.00x**)  | `123.18 us` (❌ *3.39x slower*)   | `46.51 ns` (🚀 **781.00x faster**)  | `95.75 ns` (🚀 **379.42x faster**)  | `205.94 ns` (🚀 **176.40x faster**) | `1.28 us` (🚀 **28.37x faster**)   |
| **`deserialize_uncompressed`**           | `81.60 us` (✅ **1.00x**)  | `118.12 us` (❌ *1.45x slower*)   | `46.42 ns` (🚀 **1757.92x faster**) | `95.73 ns` (🚀 **852.37x faster**)  | `205.85 ns` (🚀 **396.38x faster**) | `1.28 us` (🚀 **63.79x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `200.36 ns` (✅ **1.00x**) | `417.62 ns` (❌ *2.08x slower*)   | `46.55 ns` (🚀 **4.30x faster**)    | `95.74 ns` (🚀 **2.09x faster**)    | `205.83 ns` (✅ **1.03x slower**)   | `1.28 us` (❌ *6.39x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.23 s` (✅ **1.00x**)  | `6.60 s` (❌ *2.96x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.09 us` (✅ **1.00x**) | `35.95 us` (❌ *1.63x slower*)   | `122.35 us` (❌ *5.54x slower*)    |
| **`legendre_for_qr`**    | `12.45 us` (✅ **1.00x**) | `35.84 us` (❌ *2.88x slower*)   | `35.94 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.51 ns` (✅ **1.00x**) | `108.74 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `60.68 ns` (✅ **1.00x**) | `108.86 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.06 ns` (✅ **1.00x**) | `78.87 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `41.42 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

