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
|        | `182.00 us` (✅ **1.00x**)        | `1.62 ms` (❌ *8.91x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                   | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:-------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                          | `1.12 us` (✅ **1.00x**)   | `3.64 us` (❌ *3.25x slower*)     | `29.36 ns` (🚀 **38.13x faster**) | `181.02 ns` (🚀 **6.18x faster**)  | `19.70 ns` (🚀 **56.82x faster**) | `8.22 ns` (🚀 **136.24x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                          | `1.15 us` (✅ **1.00x**)   | `3.68 us` (❌ *3.19x slower*)     | `27.74 ns` (🚀 **41.61x faster**) | `167.10 ns` (🚀 **6.91x faster**)  | `15.37 ns` (🚀 **75.10x faster**) | `13.77 ns` (🚀 **83.81x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                          | `817.49 ns` (✅ **1.00x**) | `2.61 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                          | `743.13 ns` (✅ **1.00x**) | `2.66 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                          | `561.90 ns` (✅ **1.00x**) | `1.65 us` (❌ *2.93x slower*)     | `13.04 ns` (🚀 **43.09x faster**) | `90.71 ns` (🚀 **6.19x faster**)   | `11.44 ns` (🚀 **49.10x faster**) | `4.78 ns` (🚀 **117.51x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                          | `282.88 us` (✅ **1.00x**) | `869.12 us` (❌ *3.07x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                          | `N/A`                     | `N/A`                            | `22.56 ns` (❌ *3.79x slower*)    | `96.32 ns` (❌ *16.19x slower*)    | `17.14 ns` (❌ *2.88x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                          | `N/A`                     | `N/A`                            | `223.17 ns` (❌ *5.58x slower*)   | `5.75 us` (❌ *143.80x slower*)    | `70.39 ns` (❌ *1.76x slower*)    | `39.99 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                          | `N/A`                     | `N/A`                            | `175.89 ns` (❌ *4.95x slower*)   | `4.06 us` (❌ *114.42x slower*)    | `52.01 ns` (❌ *1.46x slower*)    | `35.51 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                          | `N/A`                     | `N/A`                            | `13.67 us` (❌ *2.14x slower*)    | `22.90 us` (❌ *3.58x slower*)     | `13.38 us` (❌ *2.09x slower*)    | `6.40 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                          | `N/A`                     | `N/A`                            | `493.83 ns` (❌ *5.98x slower*)   | `11.80 us` (❌ *142.96x slower*)   | `94.67 ns` (❌ *1.15x slower*)    | `82.54 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                          | `N/A`                     | `N/A`                            | `420.25 ns` (❌ *5.02x slower*)   | `11.71 us` (❌ *139.80x slower*)   | `138.65 ns` (❌ *1.65x slower*)   | `83.78 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `5.76 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `9.48 ns` (❌ *1.21x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**) | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `124.44 ns` (✅ **1.00x**) | `191.24 ns` (❌ *1.54x slower*)   | `30.09 ns` (🚀 **4.14x faster**)    | `49.71 ns` (🚀 **2.50x faster**)    | `100.88 ns` (✅ **1.23x faster**)   | `647.37 ns` (❌ *5.20x slower*)    |
| **`serialize_uncompressed`**             | `178.32 ns` (✅ **1.00x**) | `267.12 ns` (❌ *1.50x slower*)   | `30.01 ns` (🚀 **5.94x faster**)    | `43.87 ns` (🚀 **4.06x faster**)    | `89.00 ns` (🚀 **2.00x faster**)    | `647.26 ns` (❌ *3.63x slower*)    |
| **`deserialize_compressed`**             | `104.13 us` (✅ **1.00x**) | `212.76 us` (❌ *2.04x slower*)   | `46.54 ns` (🚀 **2237.55x faster**) | `94.58 ns` (🚀 **1100.93x faster**) | `207.33 ns` (🚀 **502.25x faster**) | `1.11 us` (🚀 **93.52x faster**)   |
| **`deserialize_compressed_unchecked`**   | `36.11 us` (✅ **1.00x**)  | `122.42 us` (❌ *3.39x slower*)   | `46.54 ns` (🚀 **775.74x faster**)  | `94.55 ns` (🚀 **381.86x faster**)  | `207.34 ns` (🚀 **174.14x faster**) | `1.26 us` (🚀 **28.61x faster**)   |
| **`deserialize_uncompressed`**           | `72.31 us` (✅ **1.00x**)  | `104.62 us` (❌ *1.45x slower*)   | `46.50 ns` (🚀 **1555.26x faster**) | `94.52 ns` (🚀 **765.07x faster**)  | `208.53 ns` (🚀 **346.77x faster**) | `1.26 us` (🚀 **57.33x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `199.93 ns` (✅ **1.00x**) | `420.85 ns` (❌ *2.11x slower*)   | `46.48 ns` (🚀 **4.30x faster**)    | `94.52 ns` (🚀 **2.12x faster**)    | `207.33 ns` (✅ **1.04x slower**)   | `1.26 us` (❌ *6.31x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.02 s` (✅ **1.00x**)  | `6.69 s` (❌ *3.31x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.14 us` (✅ **1.00x**) | `35.72 us` (❌ *1.61x slower*)   | `121.51 us` (❌ *5.49x slower*)    |
| **`legendre_for_qr`**    | `12.58 us` (✅ **1.00x**) | `36.09 us` (❌ *2.87x slower*)   | `35.96 us` (❌ *2.86x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `53.79 ns` (✅ **1.00x**) | `108.23 ns` (❌ *2.01x slower*)    |
| **`from_big-endian_bits`**    | `53.79 ns` (✅ **1.00x**) | `108.22 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `3.59 ns` (✅ **1.00x**)  | `4.32 ns` (❌ *1.20x slower*)      |
| **`equality`**                | `3.97 ns` (✅ **1.00x**)  | `4.66 ns` (❌ *1.17x slower*)      |
| **`is_zero`**                 | `3.45 ns` (✅ **1.00x**)  | `4.01 ns` (❌ *1.16x slower*)      |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.09 ns` (✅ **1.00x**) | `79.06 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `41.42 ns` (❌ *1.92x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.04 ns` (✅ **1.00x**)                 | `189.38 us` (❌ *20958.25x slower*)      | `541.79 us` (❌ *59957.31x slower*)   | `971.40 us` (❌ *107500.53x slower*)           | `1.72 ms` (❌ *190322.83x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

