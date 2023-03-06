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
|        | `282.36 us` (✅ **1.00x**)        | `2.35 ms` (❌ *8.31x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.77 us` (✅ **1.00x**)   | `4.98 us` (❌ *2.81x slower*)   | `36.47 ns` (🚀 **48.59x faster**) | `227.50 ns` (🚀 **7.79x faster**)  | `24.71 ns` (🚀 **71.71x faster**) | `10.91 ns` (🚀 **162.39x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.79 us` (✅ **1.00x**)   | `4.90 us` (❌ *2.73x slower*)   | `35.76 ns` (🚀 **50.11x faster**) | `217.43 ns` (🚀 **8.24x faster**)  | `20.14 ns` (🚀 **88.98x faster**) | `11.31 ns` (🚀 **158.44x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `3.53 us` (❌ *2.77x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `3.65 us` (❌ *2.82x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `861.23 ns` (✅ **1.00x**) | `2.35 us` (❌ *2.72x slower*)   | `17.14 ns` (🚀 **50.26x faster**) | `131.78 ns` (🚀 **6.54x faster**)  | `14.15 ns` (🚀 **60.88x faster**) | `6.85 ns` (🚀 **125.82x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `449.13 us` (✅ **1.00x**) | `1.23 ms` (❌ *2.73x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `28.54 ns` (❌ *3.51x slower*)    | `143.73 ns` (❌ *17.69x slower*)   | `22.24 ns` (❌ *2.74x slower*)    | `8.13 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `288.08 ns` (❌ *5.75x slower*)   | `7.41 us` (❌ *147.93x slower*)    | `87.36 ns` (❌ *1.74x slower*)    | `50.09 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `266.56 ns` (❌ *6.13x slower*)   | `5.26 us` (❌ *120.96x slower*)    | `76.45 ns` (❌ *1.76x slower*)    | `43.52 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.85 us` (❌ *2.32x slower*)    | `28.87 us` (❌ *3.97x slower*)     | `16.15 us` (❌ *2.22x slower*)    | `7.27 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `663.49 ns` (❌ *6.31x slower*)   | `14.93 us` (❌ *142.07x slower*)   | `140.47 ns` (❌ *1.34x slower*)   | `105.09 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `604.79 ns` (❌ *5.66x slower*)   | `14.91 us` (❌ *139.64x slower*)   | `218.66 ns` (❌ *2.05x slower*)   | `106.77 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.24 ns` (✅ **1.00x**)  | `11.37 ns` (❌ *1.38x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.70 ns` (✅ **1.00x**) | `14.51 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.54 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.06x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.30 ns` (✅ **1.00x**)  | `4.26 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `190.13 ns` (✅ **1.00x**) | `271.31 ns` (❌ *1.43x slower*)   | `37.82 ns` (🚀 **5.03x faster**)    | `61.13 ns` (🚀 **3.11x faster**)     | `124.72 ns` (✅ **1.52x faster**)   | `1.11 us` (❌ *5.86x slower*)      |
| **`serialize_uncompressed`**             | `250.47 ns` (✅ **1.00x**) | `366.43 ns` (❌ *1.46x slower*)   | `38.85 ns` (🚀 **6.45x faster**)    | `61.34 ns` (🚀 **4.08x faster**)     | `124.57 ns` (🚀 **2.01x faster**)   | `1.13 us` (❌ *4.51x slower*)      |
| **`deserialize_compressed`**             | `183.84 us` (✅ **1.00x**) | `364.48 us` (❌ *1.98x slower*)   | `61.96 ns` (🚀 **2966.84x faster**) | `118.21 ns` (🚀 **1555.17x faster**) | `288.48 ns` (🚀 **637.25x faster**) | `1.82 us` (🚀 **100.89x faster**)  |
| **`deserialize_compressed_unchecked`**   | `58.16 us` (✅ **1.00x**)  | `187.75 us` (❌ *3.23x slower*)   | `61.67 ns` (🚀 **943.07x faster**)  | `119.78 ns` (🚀 **485.54x faster**)  | `286.28 ns` (🚀 **203.15x faster**) | `1.83 us` (🚀 **31.70x faster**)   |
| **`deserialize_uncompressed`**           | `124.81 us` (✅ **1.00x**) | `170.06 us` (❌ *1.36x slower*)   | `60.88 ns` (🚀 **2050.18x faster**) | `118.00 ns` (🚀 **1057.65x faster**) | `290.46 ns` (🚀 **429.68x faster**) | `1.78 us` (🚀 **69.93x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `277.97 ns` (✅ **1.00x**) | `588.48 ns` (❌ *2.12x slower*)   | `60.48 ns` (🚀 **4.60x faster**)    | `118.84 ns` (🚀 **2.34x faster**)    | `292.77 ns` (✅ **1.05x slower**)   | `1.78 us` (❌ *6.40x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `3.34 s` (✅ **1.00x**)  | `9.08 s` (❌ *2.71x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.69 us` (✅ **1.00x**) | `57.34 us` (❌ *2.07x slower*)   | `189.72 us` (❌ *6.85x slower*)    |
| **`legendre_for_qr`**    | `15.90 us` (✅ **1.00x**) | `54.38 us` (❌ *3.42x slower*)   | `56.17 us` (❌ *3.53x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.53 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.06x slower**)    |
| **`from_little-endian_bits`** | `73.83 ns` (✅ **1.00x**) | `132.55 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `74.83 ns` (✅ **1.00x**) | `134.79 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.77 ns` (✅ **1.00x**)  | `5.00 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.98 ns` (✅ **1.00x**)  | `6.19 ns` (❌ *1.24x slower*)      |
| **`is_zero`**                 | `4.33 ns` (✅ **1.00x**)  | `4.77 ns` (✅ **1.10x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `45.65 ns` (✅ **1.00x**) | `110.17 ns` (❌ *2.41x slower*)    |
| **`into_bigint`** | `27.25 ns` (✅ **1.00x**) | `50.89 ns` (❌ *1.87x slower*)     |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `10.61 ns` (✅ **1.00x**)                | `284.35 us` (❌ *26808.50x slower*)      | `720.41 us` (❌ *67919.48x slower*)   | `1.24 ms` (❌ *116645.50x slower*)             | `2.24 ms` (❌ *210763.60x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

