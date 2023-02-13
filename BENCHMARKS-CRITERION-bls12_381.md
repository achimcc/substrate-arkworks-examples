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
|        | `182.47 us` (✅ **1.00x**)        | `1.63 ms` (❌ *8.91x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.12 us` (✅ **1.00x**)   | `3.63 us` (❌ *3.23x slower*)     | `26.62 ns` (🚀 **42.22x faster**) | `180.10 ns` (🚀 **6.24x faster**)  | `19.41 ns` (🚀 **57.90x faster**) | `8.23 ns` (🚀 **136.61x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.18x slower*)     | `27.78 ns` (🚀 **41.65x faster**) | `170.48 ns` (🚀 **6.79x faster**)  | `14.88 ns` (🚀 **77.72x faster**) | `8.58 ns` (🚀 **134.80x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `820.54 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.19x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `845.36 ns` (✅ **1.00x**) | `2.64 us` (❌ *3.12x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `560.40 ns` (✅ **1.00x**) | `1.64 us` (❌ *2.93x slower*)     | `13.00 ns` (🚀 **43.10x faster**) | `103.79 ns` (🚀 **5.40x faster**)  | `7.62 ns` (🚀 **73.53x faster**)  | `5.39 ns` (🚀 **103.98x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.61 us` (✅ **1.00x**) | `868.04 us` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.87 ns` (❌ *3.85x slower*)    | `100.32 ns` (❌ *16.87x slower*)   | `17.13 ns` (❌ *2.88x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `222.93 ns` (❌ *5.76x slower*)   | `5.74 us` (❌ *148.31x slower*)    | `70.27 ns` (❌ *1.82x slower*)    | `38.69 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.57 ns` (❌ *4.95x slower*)   | `4.02 us` (❌ *113.47x slower*)    | `58.54 ns` (❌ *1.65x slower*)    | `35.43 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `13.82 us` (❌ *2.15x slower*)    | `23.07 us` (❌ *3.58x slower*)     | `13.54 us` (❌ *2.10x slower*)    | `6.44 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `493.83 ns` (❌ *6.05x slower*)   | `11.69 us` (❌ *143.15x slower*)   | `107.44 ns` (❌ *1.32x slower*)   | `81.68 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `475.46 ns` (❌ *5.88x slower*)   | `11.62 us` (❌ *143.82x slower*)   | `157.37 ns` (❌ *1.95x slower*)   | `80.79 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.74 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `139.60 ns` (✅ **1.00x**) | `190.17 ns` (❌ *1.36x slower*)   | `29.91 ns` (🚀 **4.67x faster**)    | `49.57 ns` (🚀 **2.82x faster**)    | `98.05 ns` (✅ **1.42x faster**)    | `634.39 ns` (❌ *4.54x slower*)    |
| **`serialize_uncompressed`**             | `180.24 ns` (✅ **1.00x**) | `267.18 ns` (❌ *1.48x slower*)   | `29.82 ns` (🚀 **6.04x faster**)    | `49.48 ns` (🚀 **3.64x faster**)    | `98.06 ns` (🚀 **1.84x faster**)    | `629.98 ns` (❌ *3.50x slower*)    |
| **`deserialize_compressed`**             | `117.90 us` (✅ **1.00x**) | `241.72 us` (❌ *2.05x slower*)   | `46.57 ns` (🚀 **2531.44x faster**) | `95.91 ns` (🚀 **1229.30x faster**) | `206.26 ns` (🚀 **571.60x faster**) | `1.29 us` (🚀 **91.47x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.35 us` (✅ **1.00x**)  | `123.18 us` (❌ *3.39x slower*)   | `46.15 ns` (🚀 **787.62x faster**)  | `95.65 ns` (🚀 **380.02x faster**)  | `207.60 ns` (🚀 **175.09x faster**) | `1.28 us` (🚀 **28.35x faster**)   |
| **`deserialize_uncompressed`**           | `81.67 us` (✅ **1.00x**)  | `118.30 us` (❌ *1.45x slower*)   | `46.48 ns` (🚀 **1757.17x faster**) | `95.84 ns` (🚀 **852.18x faster**)  | `207.64 ns` (🚀 **393.32x faster**) | `1.28 us` (🚀 **63.70x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `200.85 ns` (✅ **1.00x**) | `417.65 ns` (❌ *2.08x slower*)   | `46.48 ns` (🚀 **4.32x faster**)    | `95.83 ns` (🚀 **2.10x faster**)    | `206.31 ns` (✅ **1.03x slower**)   | `1.29 us` (❌ *6.42x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.24 s` (✅ **1.00x**)  | `6.63 s` (❌ *2.96x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.13 us` (✅ **1.00x**) | `35.94 us` (❌ *1.62x slower*)   | `122.34 us` (❌ *5.53x slower*)    |
| **`legendre_for_qr`**    | `12.44 us` (✅ **1.00x**) | `35.81 us` (❌ *2.88x slower*)   | `35.94 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.79 ns` (✅ **1.00x**) | `108.78 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.63 ns` (✅ **1.00x**) | `108.68 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.04 ns` (✅ **1.00x**) | `78.94 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.67 ns` (✅ **1.00x**) | `41.43 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

