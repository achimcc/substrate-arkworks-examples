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
|        | `178.00 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.92x slower*)   | `26.61 ns` (🚀 **42.73x faster**)  | `174.88 ns` (🚀 **6.50x faster**)  | `19.28 ns` (🚀 **58.96x faster**) | `8.29 ns` (🚀 **137.10x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.83x slower*)   | `27.38 ns` (🚀 **42.86x faster**)  | `169.42 ns` (🚀 **6.93x faster**)  | `16.20 ns` (🚀 **72.43x faster**) | `8.69 ns` (🚀 **135.02x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `834.21 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.81x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `858.69 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.75x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `570.39 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.63x slower*)   | `12.76 ns` (🚀 **44.72x faster**)  | `99.45 ns` (🚀 **5.74x faster**)   | `7.47 ns` (🚀 **76.31x faster**)  | `9.09 ns` (🚀 **62.73x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.86 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.61 ns` (❌ *3.80x slower*)     | `107.21 ns` (❌ *18.01x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `266.90 ns` (❌ *7.16x slower*)    | `6.67 us` (❌ *178.78x slower*)    | `69.37 ns` (❌ *1.86x slower*)    | `37.29 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `248.32 ns` (❌ *7.61x slower*)    | `4.71 us` (❌ *144.34x slower*)    | `59.17 ns` (❌ *1.81x slower*)    | `32.64 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.76 us` (❌ *2.18x slower*)     | `25.06 us` (❌ *3.98x slower*)     | `13.44 us` (❌ *2.13x slower*)    | `6.30 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `567.19 ns` (❌ *10.69x slower*)   | `13.56 us` (❌ *255.50x slower*)   | `111.57 ns` (❌ *2.10x slower*)   | `53.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.84 ns` (❌ *6.74x slower*)    | `13.49 us` (❌ *164.07x slower*)   | `158.81 ns` (❌ *1.93x slower*)   | `82.20 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `8.07 ns` (❌ *1.24x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.90 ns` (❌ *1.39x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.15 ns` (✅ **1.00x**) | `214.48 ns` (❌ *1.46x slower*)   | `27.87 ns` (🚀 **5.28x faster**)    | `50.20 ns` (🚀 **2.93x faster**)    | `99.55 ns` (✅ **1.48x faster**)     | `627.88 ns` (❌ *4.27x slower*)    |
| **`serialize_uncompressed`**             | `197.91 ns` (✅ **1.00x**) | `317.43 ns` (❌ *1.60x slower*)   | `27.75 ns` (🚀 **7.13x faster**)    | `50.15 ns` (🚀 **3.95x faster**)    | `99.53 ns` (🚀 **1.99x faster**)     | `628.92 ns` (❌ *3.18x slower*)    |
| **`deserialize_compressed`**             | `280.85 us` (✅ **1.00x**) | `969.13 us` (❌ *3.45x slower*)   | `46.38 ns` (🚀 **6055.19x faster**) | `93.66 ns` (🚀 **2998.72x faster**) | `206.01 ns` (🚀 **1363.25x faster**) | `1.25 us` (🚀 **224.11x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.14 us` (✅ **1.00x**)  | `174.06 us` (❌ *2.67x slower*)   | `46.17 ns` (🚀 **1410.98x faster**) | `93.66 ns` (🚀 **695.51x faster**)  | `205.95 ns` (🚀 **316.29x faster**)  | `1.25 us` (🚀 **51.97x faster**)   |
| **`deserialize_uncompressed`**           | `215.89 us` (✅ **1.00x**) | `793.20 us` (❌ *3.67x slower*)   | `46.34 ns` (🚀 **4658.82x faster**) | `93.70 ns` (🚀 **2303.93x faster**) | `206.31 ns` (🚀 **1046.39x faster**) | `1.25 us` (🚀 **172.27x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.53 ns` (✅ **1.00x**) | `507.93 ns` (❌ *2.27x slower*)   | `46.34 ns` (🚀 **4.82x faster**)    | `93.68 ns` (🚀 **2.39x faster**)    | `205.70 ns` (✅ **1.09x faster**)    | `1.25 us` (❌ *5.61x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.26 s` (✅ **1.00x**)  | `7.91 s` (❌ *3.50x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.66 us` (✅ **1.00x**) | `64.76 us` (❌ *2.34x slower*)   | `172.90 us` (❌ *6.25x slower*)    |
| **`legendre_for_qr`**    | `9.54 us` (✅ **1.00x**)  | `29.36 us` (❌ *3.08x slower*)   | `29.43 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.75 ns` (✅ **1.00x**) | `107.67 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.69 ns` (✅ **1.00x**) | `107.76 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.86 ns` (✅ **1.00x**) | `79.68 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `21.72 ns` (✅ **1.00x**) | `41.54 ns` (❌ *1.91x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.02 ns` (✅ **1.00x**)                 | `233.68 us` (❌ *25893.57x slower*)      | `623.03 us` (❌ *69037.63x slower*)   | `1.16 ms` (❌ *129064.88x slower*)             | `2.04 ms` (❌ *226459.66x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

