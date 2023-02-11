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
|        | `180.60 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.97x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.64 us` (❌ *3.26x slower*)     | `29.12 ns` (🚀 **38.36x faster**) | `178.89 ns` (🚀 **6.24x faster**)  | `19.18 ns` (🚀 **58.23x faster**) | `8.21 ns` (🚀 **136.08x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.18x slower*)     | `27.55 ns` (🚀 **41.96x faster**) | `168.28 ns` (🚀 **6.87x faster**)  | `14.87 ns` (🚀 **77.72x faster**) | `8.57 ns` (🚀 **134.87x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `817.41 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.19x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `842.10 ns` (✅ **1.00x**) | `2.66 us` (❌ *3.16x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `559.90 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.93x slower*)     | `13.00 ns` (🚀 **43.08x faster**) | `99.77 ns` (🚀 **5.61x faster**)   | `7.63 ns` (🚀 **73.38x faster**)  | `5.39 ns` (🚀 **103.79x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `282.58 us` (✅ **1.00x**) | `867.28 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `23.23 ns` (❌ *3.90x slower*)    | `108.46 ns` (❌ *18.22x slower*)   | `16.78 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `223.98 ns` (❌ *5.79x slower*)   | `5.75 us` (❌ *148.54x slower*)    | `70.40 ns` (❌ *1.82x slower*)    | `38.71 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `174.89 ns` (❌ *4.93x slower*)   | `4.04 us` (❌ *113.92x slower*)    | `58.71 ns` (❌ *1.66x slower*)    | `35.47 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `14.05 us` (❌ *2.20x slower*)    | `23.25 us` (❌ *3.64x slower*)     | `13.77 us` (❌ *2.16x slower*)    | `6.39 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.49 ns` (❌ *6.03x slower*)   | `11.76 us` (❌ *143.61x slower*)   | `107.15 ns` (❌ *1.31x slower*)   | `81.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `475.74 ns` (❌ *5.90x slower*)   | `11.66 us` (❌ *144.49x slower*)   | `156.26 ns` (❌ *1.94x slower*)   | `80.68 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**) | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.23 ns` (✅ **1.00x**) | `191.85 ns` (❌ *1.36x slower*)   | `29.95 ns` (🚀 **4.72x faster**)    | `49.55 ns` (🚀 **2.85x faster**)    | `97.68 ns` (✅ **1.45x faster**)    | `629.62 ns` (❌ *4.46x slower*)    |
| **`serialize_uncompressed`**             | `179.85 ns` (✅ **1.00x**) | `268.08 ns` (❌ *1.49x slower*)   | `29.89 ns` (🚀 **6.02x faster**)    | `49.57 ns` (🚀 **3.63x faster**)    | `97.67 ns` (🚀 **1.84x faster**)    | `634.27 ns` (❌ *3.53x slower*)    |
| **`deserialize_compressed`**             | `117.58 us` (✅ **1.00x**) | `241.23 us` (❌ *2.05x slower*)   | `46.50 ns` (🚀 **2528.43x faster**) | `96.09 ns` (🚀 **1223.69x faster**) | `206.27 ns` (🚀 **570.03x faster**) | `1.30 us` (🚀 **90.18x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.08 us` (✅ **1.00x**)  | `122.90 us` (❌ *3.41x slower*)   | `46.07 ns` (🚀 **783.20x faster**)  | `96.08 ns` (🚀 **375.54x faster**)  | `206.26 ns` (🚀 **174.94x faster**) | `1.30 us` (🚀 **27.68x faster**)   |
| **`deserialize_uncompressed`**           | `81.55 us` (✅ **1.00x**)  | `118.41 us` (❌ *1.45x slower*)   | `46.48 ns` (🚀 **1754.41x faster**) | `95.70 ns` (🚀 **852.19x faster**)  | `207.38 ns` (🚀 **393.24x faster**) | `1.31 us` (🚀 **62.43x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `199.92 ns` (✅ **1.00x**) | `414.17 ns` (❌ *2.07x slower*)   | `46.47 ns` (🚀 **4.30x faster**)    | `95.68 ns` (🚀 **2.09x faster**)    | `206.13 ns` (✅ **1.03x slower**)   | `1.30 us` (❌ *6.52x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.23 s` (✅ **1.00x**)  | `6.72 s` (❌ *3.01x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.24 us` (✅ **1.00x**) | `35.71 us` (❌ *1.61x slower*)   | `122.03 us` (❌ *5.49x slower*)    |
| **`legendre_for_qr`**    | `12.51 us` (✅ **1.00x**) | `35.50 us` (❌ *2.84x slower*)   | `35.69 us` (❌ *2.85x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.58 ns` (✅ **1.00x**) | `107.93 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.54 ns` (✅ **1.00x**) | `108.13 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.18 ns` (✅ **1.00x**) | `79.05 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.58 ns` (✅ **1.00x**) | `41.45 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

