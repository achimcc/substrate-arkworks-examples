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
|        | `157.30 us` (✅ **1.00x**)        | `1.85 ms` (❌ *11.74x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.00 us` (✅ **1.00x**)   | `4.44 us` (❌ *4.43x slower*)   | `26.98 ns` (🚀 **37.17x faster**)  | `178.04 ns` (🚀 **5.63x faster**)  | `19.32 ns` (🚀 **51.91x faster**) | `8.29 ns` (🚀 **121.05x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.49 us` (❌ *3.84x slower*)   | `27.04 ns` (🚀 **43.31x faster**)  | `151.68 ns` (🚀 **7.72x faster**)  | `14.97 ns` (🚀 **78.24x faster**) | `8.60 ns` (🚀 **136.18x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `834.02 ns` (✅ **1.00x**) | `3.17 us` (❌ *3.80x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `855.47 ns` (✅ **1.00x**) | `3.21 us` (❌ *3.76x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `571.09 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.63x slower*)   | `12.90 ns` (🚀 **44.27x faster**)  | `100.70 ns` (🚀 **5.67x faster**)  | `7.48 ns` (🚀 **76.34x faster**)  | `5.32 ns` (🚀 **107.33x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.16 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `24.08 ns` (❌ *4.58x slower*)     | `107.35 ns` (❌ *20.42x slower*)   | `16.76 ns` (❌ *3.19x slower*)    | `5.26 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.44 ns` (❌ *7.17x slower*)    | `6.65 us` (❌ *178.30x slower*)    | `61.20 ns` (❌ *1.64x slower*)    | `37.30 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `246.61 ns` (❌ *8.55x slower*)    | `4.15 us` (❌ *143.91x slower*)    | `59.17 ns` (❌ *2.05x slower*)    | `28.85 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.80 us` (❌ *2.48x slower*)     | `22.13 us` (❌ *3.98x slower*)     | `13.48 us` (❌ *2.42x slower*)    | `5.57 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `568.32 ns` (❌ *10.71x slower*)   | `11.97 us` (❌ *225.45x slower*)   | `99.08 ns` (❌ *1.87x slower*)    | `53.09 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `485.96 ns` (❌ *5.90x slower*)    | `13.50 us` (❌ *163.89x slower*)   | `156.06 ns` (❌ *1.89x slower*)   | `82.36 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**) | `10.73 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.36 ns` (✅ **1.00x**) | `188.73 ns` (❌ *1.28x slower*)   | `28.03 ns` (🚀 **5.26x faster**)    | `50.17 ns` (🚀 **2.94x faster**)    | `100.12 ns` (✅ **1.47x faster**)    | `553.36 ns` (❌ *3.76x slower*)    |
| **`serialize_uncompressed`**             | `174.70 ns` (✅ **1.00x**) | `318.28 ns` (❌ *1.82x slower*)   | `28.00 ns` (🚀 **6.24x faster**)    | `44.24 ns` (🚀 **3.95x faster**)    | `100.10 ns` (✅ **1.75x faster**)    | `627.09 ns` (❌ *3.59x slower*)    |
| **`deserialize_compressed`**             | `281.21 us` (✅ **1.00x**) | `969.07 us` (❌ *3.45x slower*)   | `44.70 ns` (🚀 **6291.24x faster**) | `93.11 ns` (🚀 **3020.21x faster**) | `184.32 ns` (🚀 **1525.72x faster**) | `1.11 us` (🚀 **253.99x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.32 us` (✅ **1.00x**)  | `174.25 us` (❌ *2.67x slower*)   | `44.69 ns` (🚀 **1461.55x faster**) | `82.27 ns` (🚀 **793.98x faster**)  | `208.14 ns` (🚀 **313.83x faster**)  | `1.11 us` (🚀 **58.99x faster**)   |
| **`deserialize_uncompressed`**           | `190.74 us` (✅ **1.00x**) | `792.11 us` (❌ *4.15x slower*)   | `44.62 ns` (🚀 **4275.14x faster**) | `93.07 ns` (🚀 **2049.38x faster**) | `183.85 ns` (🚀 **1037.51x faster**) | `1.25 us` (🚀 **152.43x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `228.15 ns` (✅ **1.00x**) | `470.48 ns` (❌ *2.06x slower*)   | `39.40 ns` (🚀 **5.79x faster**)    | `93.08 ns` (🚀 **2.45x faster**)    | `208.11 ns` (✅ **1.10x faster**)    | `1.25 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.30 s` (✅ **1.00x**)  | `8.01 s` (❌ *3.48x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.74 us` (✅ **1.00x**) | `65.00 us` (❌ *2.34x slower*)   | `173.13 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `8.46 us` (✅ **1.00x**)  | `26.02 us` (❌ *3.08x slower*)   | `29.71 us` (❌ *3.51x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.51 ns` (✅ **1.00x**)  | `3.70 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.94 ns` (✅ **1.00x**) | `108.47 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `53.77 ns` (✅ **1.00x**) | `95.61 ns` (❌ *1.78x slower*)     |
| **`comparison`**              | `3.56 ns` (✅ **1.00x**)  | `4.30 ns` (❌ *1.21x slower*)      |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)  | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.88 ns` (✅ **1.00x**) | `69.25 ns` (❌ *1.93x slower*)    |
| **`into_bigint`** | `21.72 ns` (✅ **1.00x**) | `41.54 ns` (❌ *1.91x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.03 ns` (✅ **1.00x**)                 | `233.48 us` (❌ *25848.74x slower*)      | `1.87 ms` (❌ *207578.38x slower*)    | `1.16 ms` (❌ *128822.82x slower*)             | `2.04 ms` (❌ *226009.78x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

