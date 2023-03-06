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
|        | `286.76 us` (✅ **1.00x**)        | `2.29 ms` (❌ *8.00x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.75 us` (✅ **1.00x**)   | `4.84 us` (❌ *2.76x slower*)   | `34.77 ns` (🚀 **50.47x faster**) | `225.26 ns` (🚀 **7.79x faster**)  | `24.46 ns` (🚀 **71.73x faster**) | `10.87 ns` (🚀 **161.46x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.80 us` (✅ **1.00x**)   | `4.91 us` (❌ *2.73x slower*)   | `35.69 ns` (🚀 **50.42x faster**) | `211.05 ns` (🚀 **8.53x faster**)  | `20.08 ns` (🚀 **89.62x faster**) | `11.87 ns` (🚀 **151.58x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `3.54 us` (❌ *2.84x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `3.55 us` (❌ *2.74x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `858.40 ns` (✅ **1.00x**) | `2.38 us` (❌ *2.77x slower*)   | `17.11 ns` (🚀 **50.17x faster**) | `130.15 ns` (🚀 **6.60x faster**)  | `9.76 ns` (🚀 **87.96x faster**)  | `6.78 ns` (🚀 **126.60x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `441.13 us` (✅ **1.00x**) | `1.21 ms` (❌ *2.73x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.29 ns` (❌ *3.34x slower*)    | `139.39 ns` (❌ *17.07x slower*)   | `22.11 ns` (❌ *2.71x slower*)    | `8.17 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `280.23 ns` (❌ *5.62x slower*)   | `7.28 us` (❌ *145.87x slower*)    | `89.32 ns` (❌ *1.79x slower*)    | `49.88 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `256.90 ns` (❌ *5.93x slower*)   | `5.07 us` (❌ *117.08x slower*)    | `75.76 ns` (❌ *1.75x slower*)    | `43.31 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.35 us` (❌ *2.24x slower*)    | `28.91 us` (❌ *3.96x slower*)     | `17.08 us` (❌ *2.34x slower*)    | `7.31 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `611.58 ns` (❌ *5.87x slower*)   | `14.56 us` (❌ *139.85x slower*)   | `145.80 ns` (❌ *1.40x slower*)   | `104.13 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `588.08 ns` (❌ *5.47x slower*)   | `15.18 us` (❌ *141.25x slower*)   | `229.28 ns` (❌ *2.13x slower*)   | `107.48 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.05 ns` (✅ **1.00x**)  | `10.46 ns` (❌ *1.30x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.45 ns` (✅ **1.00x**) | `14.97 ns` (❌ *1.43x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.57 ns` (✅ **1.00x**)  | `4.70 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.25 ns` (✅ **1.00x**)  | `4.27 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `197.09 ns` (✅ **1.00x**) | `268.62 ns` (❌ *1.36x slower*)   | `38.14 ns` (🚀 **5.17x faster**)    | `62.96 ns` (🚀 **3.13x faster**)     | `122.14 ns` (✅ **1.61x faster**)   | `795.58 ns` (❌ *4.04x slower*)    |
| **`serialize_uncompressed`**             | `260.41 ns` (✅ **1.00x**) | `380.98 ns` (❌ *1.46x slower*)   | `42.50 ns` (🚀 **6.13x faster**)    | `62.55 ns` (🚀 **4.16x faster**)     | `127.45 ns` (🚀 **2.04x faster**)   | `824.18 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `185.89 us` (✅ **1.00x**) | `357.96 us` (❌ *1.93x slower*)   | `60.62 ns` (🚀 **3066.53x faster**) | `135.58 ns` (🚀 **1371.07x faster**) | `271.46 ns` (🚀 **684.78x faster**) | `1.82 us` (🚀 **102.22x faster**)  |
| **`deserialize_compressed_unchecked`**   | `56.68 us` (✅ **1.00x**)  | `190.33 us` (❌ *3.36x slower*)   | `60.89 ns` (🚀 **930.97x faster**)  | `133.29 ns` (🚀 **425.25x faster**)  | `276.74 ns` (🚀 **204.82x faster**) | `1.84 us` (🚀 **30.85x faster**)   |
| **`deserialize_uncompressed`**           | `129.39 us` (✅ **1.00x**) | `171.87 us` (❌ *1.33x slower*)   | `60.70 ns` (🚀 **2131.79x faster**) | `129.59 ns` (🚀 **998.42x faster**)  | `280.79 ns` (🚀 **460.81x faster**) | `1.79 us` (🚀 **72.47x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `282.10 ns` (✅ **1.00x**) | `587.32 ns` (❌ *2.08x slower*)   | `61.08 ns` (🚀 **4.62x faster**)    | `130.34 ns` (🚀 **2.16x faster**)    | `295.46 ns` (✅ **1.05x slower**)   | `1.81 us` (❌ *6.40x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `3.44 s` (✅ **1.00x**)  | `9.15 s` (❌ *2.66x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.61 us` (✅ **1.00x**) | `55.96 us` (❌ *2.03x slower*)   | `192.46 us` (❌ *6.97x slower*)    |
| **`legendre_for_qr`**    | `16.07 us` (✅ **1.00x**) | `55.44 us` (❌ *3.45x slower*)   | `56.60 us` (❌ *3.52x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.56 ns` (✅ **1.00x**)  | `4.78 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `74.93 ns` (✅ **1.00x**) | `132.96 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `74.83 ns` (✅ **1.00x**) | `134.63 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.60 ns` (✅ **1.00x**)  | `4.92 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `4.93 ns` (✅ **1.00x**)  | `5.99 ns` (❌ *1.21x slower*)      |
| **`is_zero`**                 | `4.46 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.06x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `46.01 ns` (✅ **1.00x**) | `108.87 ns` (❌ *2.37x slower*)    |
| **`into_bigint`** | `27.09 ns` (✅ **1.00x**) | `50.71 ns` (❌ *1.87x slower*)     |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `10.60 ns` (✅ **1.00x**)                | `283.74 us` (❌ *26774.53x slower*)      | `704.88 us` (❌ *66514.54x slower*)   | `1.24 ms` (❌ *117396.46x slower*)             | `2.22 ms` (❌ *209573.83x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

