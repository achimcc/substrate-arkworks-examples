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
|        | `177.91 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.41x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.89x slower*)   | `26.58 ns` (🚀 **43.08x faster**)  | `176.25 ns` (🚀 **6.50x faster**)  | `19.31 ns` (🚀 **59.28x faster**) | `8.28 ns` (🚀 **138.25x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.18 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.81x slower*)   | `27.25 ns` (🚀 **43.35x faster**)  | `171.65 ns` (🚀 **6.88x faster**)  | `15.00 ns` (🚀 **78.74x faster**) | `8.66 ns` (🚀 **136.44x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `842.54 ns` (✅ **1.00x**) | `3.18 us` (❌ *3.77x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `865.68 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.72x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `585.17 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.54x slower*)   | `12.84 ns` (🚀 **45.58x faster**)  | `103.48 ns` (🚀 **5.66x faster**)  | `7.47 ns` (🚀 **78.29x faster**)  | `9.06 ns` (🚀 **64.56x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.74 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.71 ns` (❌ *3.82x slower*)     | `107.02 ns` (❌ *17.99x slower*)   | `16.75 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.14 ns` (❌ *7.17x slower*)    | `6.67 us` (❌ *178.91x slower*)    | `69.37 ns` (❌ *1.86x slower*)    | `37.28 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `248.47 ns` (❌ *7.61x slower*)    | `4.69 us` (❌ *143.62x slower*)    | `59.09 ns` (❌ *1.81x slower*)    | `32.65 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.76 us` (❌ *2.18x slower*)     | `25.01 us` (❌ *3.97x slower*)     | `13.44 us` (❌ *2.13x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `569.96 ns` (❌ *10.74x slower*)   | `13.57 us` (❌ *255.66x slower*)   | `111.68 ns` (❌ *2.10x slower*)   | `53.07 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.27 ns` (❌ *6.78x slower*)    | `13.49 us` (❌ *165.29x slower*)   | `157.91 ns` (❌ *1.93x slower*)   | `81.64 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.72 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.23 ns` (✅ **1.00x**) | `212.51 ns` (❌ *1.44x slower*)   | `27.87 ns` (🚀 **5.28x faster**)    | `50.12 ns` (🚀 **2.94x faster**)    | `100.01 ns` (✅ **1.47x faster**)    | `625.69 ns` (❌ *4.25x slower*)    |
| **`serialize_uncompressed`**             | `198.05 ns` (✅ **1.00x**) | `318.23 ns` (❌ *1.61x slower*)   | `27.76 ns` (🚀 **7.13x faster**)    | `50.13 ns` (🚀 **3.95x faster**)    | `100.03 ns` (🚀 **1.98x faster**)    | `625.82 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `281.01 us` (✅ **1.00x**) | `969.46 us` (❌ *3.45x slower*)   | `46.43 ns` (🚀 **6053.00x faster**) | `96.13 ns` (🚀 **2923.25x faster**) | `205.81 ns` (🚀 **1365.38x faster**) | `1.27 us` (🚀 **221.75x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.18 us` (✅ **1.00x**)  | `173.79 us` (❌ *2.67x slower*)   | `46.43 ns` (🚀 **1403.88x faster**) | `95.91 ns` (🚀 **679.58x faster**)  | `205.85 ns` (🚀 **316.62x faster**)  | `1.27 us` (🚀 **51.28x faster**)   |
| **`deserialize_uncompressed`**           | `216.02 us` (✅ **1.00x**) | `792.85 us` (❌ *3.67x slower*)   | `46.36 ns` (🚀 **4660.05x faster**) | `96.19 ns` (🚀 **2245.81x faster**) | `206.26 ns` (🚀 **1047.33x faster**) | `1.27 us` (🚀 **170.55x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.05 ns` (✅ **1.00x**) | `472.44 ns` (❌ *2.10x slower*)   | `46.36 ns` (🚀 **4.85x faster**)    | `96.16 ns` (🚀 **2.34x faster**)    | `206.22 ns` (✅ **1.09x faster**)    | `1.27 us` (❌ *5.63x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.31 s` (✅ **1.00x**)  | `8.05 s` (❌ *3.49x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.66 us` (✅ **1.00x**) | `64.72 us` (❌ *2.34x slower*)   | `173.30 us` (❌ *6.27x slower*)    |
| **`legendre_for_qr`**    | `9.53 us` (✅ **1.00x**)  | `29.36 us` (❌ *3.08x slower*)   | `29.47 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.98 ns` (✅ **1.00x**) | `107.86 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.90 ns` (✅ **1.00x**) | `107.83 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.05 ns` (✅ **1.00x**)  | `4.30 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.65 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.92 ns` (✅ **1.00x**) | `79.35 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.51 ns` (❌ *1.92x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.04 ns` (✅ **1.00x**)                 | `233.69 us` (❌ *25864.93x slower*)      | `623.37 us` (❌ *68995.06x slower*)   | `1.16 ms` (❌ *128752.04x slower*)             | `2.04 ms` (❌ *226104.74x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

