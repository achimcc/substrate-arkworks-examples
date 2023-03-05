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
|        | `218.21 us` (✅ **1.00x**)        | `1.93 ms` (❌ *8.86x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.35 us` (✅ **1.00x**)   | `4.33 us` (❌ *3.22x slower*)   | `33.18 ns` (🚀 **40.61x faster**) | `214.81 ns` (🚀 **6.27x faster**)  | `23.32 ns` (🚀 **57.77x faster**) | `9.68 ns` (🚀 **139.23x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.38 us` (✅ **1.00x**)   | `4.39 us` (❌ *3.18x slower*)   | `32.65 ns` (🚀 **42.24x faster**) | `204.11 ns` (🚀 **6.76x faster**)  | `18.01 ns` (🚀 **76.57x faster**) | `10.15 ns` (🚀 **135.82x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `989.42 ns` (✅ **1.00x**) | `3.13 us` (❌ *3.16x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.01 us` (✅ **1.00x**)   | `3.18 us` (❌ *3.15x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `670.69 ns` (✅ **1.00x**) | `1.96 us` (❌ *2.92x slower*)   | `15.49 ns` (🚀 **43.29x faster**) | `119.47 ns` (🚀 **5.61x faster**)  | `9.15 ns` (🚀 **73.26x faster**)  | `6.44 ns` (🚀 **104.17x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `339.93 us` (✅ **1.00x**) | `1.05 ms` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.06 ns` (❌ *3.83x slower*)    | `128.52 ns` (❌ *18.20x slower*)   | `20.45 ns` (❌ *2.90x slower*)    | `7.06 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.00 ns` (❌ *5.76x slower*)   | `6.88 us` (❌ *148.48x slower*)    | `84.35 ns` (❌ *1.82x slower*)    | `46.35 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `208.70 ns` (❌ *4.96x slower*)   | `4.83 us` (❌ *114.69x slower*)    | `69.91 ns` (❌ *1.66x slower*)    | `42.07 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.56 us` (❌ *2.18x slower*)    | `27.62 us` (❌ *3.63x slower*)     | `16.21 us` (❌ *2.13x slower*)    | `7.60 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `592.14 ns` (❌ *6.10x slower*)   | `14.12 us` (❌ *145.39x slower*)   | `126.84 ns` (❌ *1.31x slower*)   | `97.10 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `564.44 ns` (❌ *5.85x slower*)   | `13.94 us` (❌ *144.62x slower*)   | `186.89 ns` (❌ *1.94x slower*)   | `96.41 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**) | `9.31 ns` (❌ *1.19x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.40 ns` (✅ **1.00x**) | `12.58 ns` (❌ *1.34x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.60 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.49 ns` (✅ **1.00x**) | `4.44 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `168.34 ns` (✅ **1.00x**) | `230.37 ns` (❌ *1.37x slower*)   | `34.36 ns` (🚀 **4.90x faster**)    | `57.23 ns` (🚀 **2.94x faster**)     | `115.10 ns` (✅ **1.46x faster**)   | `759.85 ns` (❌ *4.51x slower*)    |
| **`serialize_uncompressed`**             | `223.92 ns` (✅ **1.00x**) | `317.95 ns` (❌ *1.42x slower*)   | `34.17 ns` (🚀 **6.55x faster**)    | `58.14 ns` (🚀 **3.85x faster**)     | `115.68 ns` (🚀 **1.94x faster**)   | `756.23 ns` (❌ *3.38x slower*)    |
| **`deserialize_compressed`**             | `141.00 us` (✅ **1.00x**) | `287.51 us` (❌ *2.04x slower*)   | `55.09 ns` (🚀 **2559.64x faster**) | `112.37 ns` (🚀 **1254.82x faster**) | `247.28 ns` (🚀 **570.20x faster**) | `1.50 us` (🚀 **94.11x faster**)   |
| **`deserialize_compressed_unchecked`**   | `43.14 us` (✅ **1.00x**)  | `147.01 us` (❌ *3.41x slower*)   | `55.35 ns` (🚀 **779.53x faster**)  | `111.84 ns` (🚀 **385.78x faster**)  | `245.28 ns` (🚀 **175.90x faster**) | `1.51 us` (🚀 **28.58x faster**)   |
| **`deserialize_uncompressed`**           | `97.91 us` (✅ **1.00x**)  | `141.33 us` (❌ *1.44x slower*)   | `55.08 ns` (🚀 **1777.54x faster**) | `113.25 ns` (🚀 **864.57x faster**)  | `246.89 ns` (🚀 **396.56x faster**) | `1.51 us` (🚀 **64.92x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `241.18 ns` (✅ **1.00x**) | `494.80 ns` (❌ *2.05x slower*)   | `55.35 ns` (🚀 **4.36x faster**)    | `112.88 ns` (🚀 **2.14x faster**)    | `246.73 ns` (✅ **1.02x slower**)   | `1.51 us` (❌ *6.26x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.67 s` (✅ **1.00x**)  | `7.89 s` (❌ *2.96x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `26.53 us` (✅ **1.00x**) | `42.32 us` (❌ *1.60x slower*)   | `145.72 us` (❌ *5.49x slower*)    |
| **`legendre_for_qr`**    | `14.80 us` (✅ **1.00x**) | `42.69 us` (❌ *2.88x slower*)   | `42.98 us` (❌ *2.90x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.77 ns` (✅ **1.00x**)  | `5.00 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `72.71 ns` (✅ **1.00x**) | `128.89 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `72.76 ns` (✅ **1.00x**) | `128.75 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `5.39 ns` (✅ **1.00x**)  | `5.56 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `4.69 ns` (✅ **1.00x**)  | `4.78 ns` (✅ **1.02x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.07 ns` (✅ **1.00x**) | `93.99 ns` (❌ *2.18x slower*)    |
| **`into_bigint`** | `25.82 ns` (✅ **1.00x**) | `49.42 ns` (❌ *1.91x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `10.82 ns` (✅ **1.00x**)                | `227.01 us` (❌ *20973.60x slower*)      | `648.29 us` (❌ *59896.55x slower*)   | `1.16 ms` (❌ *107293.20x slower*)             | `2.05 ms` (❌ *189442.89x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

