# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377](#sample_bls12_377)
    - [arithmetic_for_bls12_377](#arithmetic_for_bls12_377)
    - [serialization_for_bls12_377](#serialization_for_bls12_377)
    - [msm_for_bls12_377](#msm_for_bls12_377)
    - [squareroot_for_bls12_377](#squareroot_for_bls12_377)
    - [bitwise_operations_for_bls12_377](#bitwise_operations_for_bls12_377)
    - [conversions_for_bls12_377](#conversions_for_bls12_377)
    - [pairing_for_bls12_377](#pairing_for_bls12_377)

## Benchmark Results

### sample_bls12_377

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `177.07 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.44x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.44 us` (❌ *3.91x slower*)   | `27.09 ns` (🚀 **41.91x faster**)  | `177.17 ns` (🚀 **6.41x faster**)  | `19.30 ns` (🚀 **58.82x faster**) | `8.30 ns` (🚀 **136.83x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.49 us` (❌ *3.84x slower*)   | `27.33 ns` (🚀 **42.82x faster**)  | `172.33 ns` (🚀 **6.79x faster**)  | `15.65 ns` (🚀 **74.77x faster**) | `8.62 ns` (🚀 **135.75x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `836.34 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.80x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `857.74 ns` (✅ **1.00x**) | `3.21 us` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `568.77 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.64x slower*)   | `12.88 ns` (🚀 **44.14x faster**)  | `99.24 ns` (🚀 **5.73x faster**)   | `7.48 ns` (🚀 **76.07x faster**)  | `9.11 ns` (🚀 **62.44x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.94 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.62 ns` (❌ *3.80x slower*)     | `107.45 ns` (❌ *18.07x slower*)   | `16.75 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.40 ns` (❌ *7.17x slower*)    | `6.66 us` (❌ *178.51x slower*)    | `71.06 ns` (❌ *1.91x slower*)    | `37.30 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `249.30 ns` (❌ *7.86x slower*)    | `4.68 us` (❌ *147.65x slower*)    | `59.20 ns` (❌ *1.87x slower*)    | `31.72 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.60 us` (❌ *2.14x slower*)     | `24.95 us` (❌ *3.92x slower*)     | `13.28 us` (❌ *2.09x slower*)    | `6.37 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `570.19 ns` (❌ *10.74x slower*)   | `13.56 us` (❌ *255.41x slower*)   | `112.20 ns` (❌ *2.11x slower*)   | `53.11 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `554.46 ns` (❌ *6.97x slower*)    | `13.46 us` (❌ *169.16x slower*)   | `156.80 ns` (❌ *1.97x slower*)   | `79.57 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.26 ns` (✅ **1.00x**) | `212.79 ns` (❌ *1.44x slower*)   | `27.99 ns` (🚀 **5.26x faster**)    | `50.13 ns` (🚀 **2.94x faster**)    | `100.38 ns` (✅ **1.47x faster**)    | `631.30 ns` (❌ *4.29x slower*)    |
| **`serialize_uncompressed`**             | `198.02 ns` (✅ **1.00x**) | `319.35 ns` (❌ *1.61x slower*)   | `27.74 ns` (🚀 **7.14x faster**)    | `50.17 ns` (🚀 **3.95x faster**)    | `100.39 ns` (🚀 **1.97x faster**)    | `630.02 ns` (❌ *3.18x slower*)    |
| **`deserialize_compressed`**             | `280.25 us` (✅ **1.00x**) | `966.71 us` (❌ *3.45x slower*)   | `46.39 ns` (🚀 **6041.41x faster**) | `95.56 ns` (🚀 **2932.66x faster**) | `207.41 ns` (🚀 **1351.20x faster**) | `1.27 us` (🚀 **220.72x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.94 us` (✅ **1.00x**)  | `172.78 us` (❌ *2.66x slower*)   | `46.40 ns` (🚀 **1399.72x faster**) | `95.57 ns` (🚀 **679.53x faster**)  | `207.39 ns` (🚀 **313.14x faster**)  | `1.27 us` (🚀 **51.12x faster**)   |
| **`deserialize_uncompressed`**           | `215.66 us` (✅ **1.00x**) | `792.19 us` (❌ *3.67x slower*)   | `46.34 ns` (🚀 **4653.59x faster**) | `95.58 ns` (🚀 **2256.27x faster**) | `207.10 ns` (🚀 **1041.31x faster**) | `1.27 us` (🚀 **169.90x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.73 ns` (✅ **1.00x**) | `469.00 ns` (❌ *2.08x slower*)   | `46.34 ns` (🚀 **4.87x faster**)    | `95.57 ns` (🚀 **2.36x faster**)    | `207.07 ns` (✅ **1.09x faster**)    | `1.27 us` (❌ *5.63x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.31 s` (✅ **1.00x**)  | `7.94 s` (❌ *3.44x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.60 us` (✅ **1.00x**) | `64.41 us` (❌ *2.33x slower*)   | `171.91 us` (❌ *6.23x slower*)    |
| **`legendre_for_qr`**    | `9.53 us` (✅ **1.00x**)  | `29.01 us` (❌ *3.04x slower*)   | `29.81 us` (❌ *3.13x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.17 ns` (✅ **1.00x**) | `107.93 ns` (❌ *1.76x slower*)    |
| **`from_big-endian_bits`**    | `61.18 ns` (✅ **1.00x**) | `107.84 ns` (❌ *1.76x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.33 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.93 ns` (✅ **1.00x**) | `79.23 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `41.51 ns` (❌ *1.92x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.03 ns` (✅ **1.00x**)                 | `233.83 us` (❌ *25891.94x slower*)      | `620.08 us` (❌ *68661.54x slower*)   | `1.16 ms` (❌ *128847.17x slower*)             | `2.04 ms` (❌ *225904.43x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

