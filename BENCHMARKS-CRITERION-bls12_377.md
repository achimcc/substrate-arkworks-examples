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
|        | `269.94 us` (✅ **1.00x**)        | `2.73 ms` (❌ *10.11x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.87 us` (✅ **1.00x**)   | `6.15 us` (❌ *3.29x slower*)   | `36.48 ns` (🚀 **51.23x faster**)  | `226.14 ns` (🚀 **8.26x faster**)  | `24.73 ns` (🚀 **75.58x faster**) | `11.04 ns` (🚀 **169.25x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.89 us` (✅ **1.00x**)   | `6.22 us` (❌ *3.29x slower*)   | `35.22 ns` (🚀 **53.67x faster**)  | `221.53 ns` (🚀 **8.53x faster**)  | `21.76 ns` (🚀 **86.88x faster**) | `11.32 ns` (🚀 **166.93x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.37 us` (✅ **1.00x**)   | `4.29 us` (❌ *3.13x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.36 us` (✅ **1.00x**)   | `4.46 us` (❌ *3.29x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `876.51 ns` (✅ **1.00x**) | `2.96 us` (❌ *3.38x slower*)   | `17.07 ns` (🚀 **51.36x faster**)  | `130.01 ns` (🚀 **6.74x faster**)  | `13.62 ns` (🚀 **64.36x faster**) | `10.60 ns` (🚀 **82.69x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `445.71 us` (✅ **1.00x**) | `1.46 ms` (❌ *3.28x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `48.28 ns` (❌ *5.90x slower*)     | `137.66 ns` (❌ *16.82x slower*)   | `22.35 ns` (❌ *2.73x slower*)    | `8.18 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `332.04 ns` (❌ *7.29x slower*)    | `8.63 us` (❌ *189.42x slower*)    | `89.59 ns` (❌ *1.97x slower*)    | `45.58 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `370.07 ns` (❌ *9.31x slower*)    | `6.08 us` (❌ *152.97x slower*)    | `79.12 ns` (❌ *1.99x slower*)    | `39.76 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.60 us` (❌ *2.33x slower*)     | `32.29 us` (❌ *4.54x slower*)     | `16.24 us` (❌ *2.28x slower*)    | `7.12 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `728.48 ns` (❌ *11.50x slower*)   | `17.32 us` (❌ *273.54x slower*)   | `143.78 ns` (❌ *2.27x slower*)   | `63.33 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `710.77 ns` (❌ *7.11x slower*)    | `17.31 us` (❌ *173.18x slower*)   | `223.70 ns` (❌ *2.24x slower*)   | `99.98 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `8.06 ns` (✅ **1.00x**)  | `20.97 ns` (❌ *2.60x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.66 ns` (✅ **1.00x**) | `15.01 ns` (❌ *1.41x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.55 ns` (✅ **1.00x**)  | `4.76 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.35 ns` (✅ **1.00x**)  | `4.30 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `194.47 ns` (✅ **1.00x**) | `272.54 ns` (❌ *1.40x slower*)   | `37.27 ns` (🚀 **5.22x faster**)    | `63.17 ns` (🚀 **3.08x faster**)     | `123.13 ns` (✅ **1.58x faster**)    | `813.98 ns` (❌ *4.19x slower*)    |
| **`serialize_uncompressed`**             | `265.98 ns` (✅ **1.00x**) | `416.11 ns` (❌ *1.56x slower*)   | `37.15 ns` (🚀 **7.16x faster**)    | `65.53 ns` (🚀 **4.06x faster**)     | `122.88 ns` (🚀 **2.16x faster**)    | `789.19 ns` (❌ *2.97x slower*)    |
| **`deserialize_compressed`**             | `422.02 us` (✅ **1.00x**) | `1.37 ms` (❌ *3.25x slower*)     | `58.17 ns` (🚀 **7254.90x faster**) | `141.38 ns` (🚀 **2985.04x faster**) | `304.55 ns` (🚀 **1385.70x faster**) | `1.75 us` (🚀 **241.72x faster**)  |
| **`deserialize_compressed_unchecked`**   | `95.60 us` (✅ **1.00x**)  | `250.34 us` (❌ *2.62x slower*)   | `58.09 ns` (🚀 **1645.66x faster**) | `135.44 ns` (🚀 **705.86x faster**)  | `293.46 ns` (🚀 **325.78x faster**)  | `1.85 us` (🚀 **51.56x faster**)   |
| **`deserialize_uncompressed`**           | `336.94 us` (✅ **1.00x**) | `1.11 ms` (❌ *3.29x slower*)     | `59.16 ns` (🚀 **5695.64x faster**) | `140.43 ns` (🚀 **2399.31x faster**) | `295.14 ns` (🚀 **1141.63x faster**) | `1.91 us` (🚀 **176.62x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `313.53 ns` (✅ **1.00x**) | `623.70 ns` (❌ *1.99x slower*)   | `58.38 ns` (🚀 **5.37x faster**)    | `138.75 ns` (🚀 **2.26x faster**)    | `289.95 ns` (✅ **1.08x faster**)    | `1.82 us` (❌ *5.79x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `3.61 s` (✅ **1.00x**)  | `10.94 s` (❌ *3.03x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `35.06 us` (✅ **1.00x**) | `95.47 us` (❌ *2.72x slower*)   | `251.49 us` (❌ *7.17x slower*)    |
| **`legendre_for_qr`**    | `13.20 us` (✅ **1.00x**) | `46.84 us` (❌ *3.55x slower*)   | `46.92 us` (❌ *3.56x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.49 ns` (✅ **1.00x**)  | `5.09 ns` (❌ *1.13x slower*)      |
| **`from_little-endian_bits`** | `73.38 ns` (✅ **1.00x**) | `131.98 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `74.70 ns` (✅ **1.00x**) | `132.47 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.54 ns` (✅ **1.00x**)  | `4.93 ns` (✅ **1.09x slower**)    |
| **`equality`**                | `5.02 ns` (✅ **1.00x**)  | `5.95 ns` (❌ *1.19x slower*)      |
| **`is_zero`**                 | `4.42 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.07x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `49.97 ns` (✅ **1.00x**) | `109.20 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `27.29 ns` (✅ **1.00x**) | `53.12 ns` (❌ *1.95x slower*)     |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `11.02 ns` (✅ **1.00x**)                | `344.92 us` (❌ *31296.44x slower*)      | `822.11 us` (❌ *74593.17x slower*)   | `1.53 ms` (❌ *138659.76x slower*)             | `2.68 ms` (❌ *242762.50x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

