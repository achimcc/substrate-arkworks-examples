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

## Benchmark Results

### sample_bls12_377

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `264.84 us` (✅ **1.00x**)        | `2.53 ms` (❌ *9.54x slower*)      |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.75 us` (✅ **1.00x**)   | `5.90 us` (❌ *3.36x slower*)   | `33.52 ns` (🚀 **52.35x faster**)  | `223.62 ns` (🚀 **7.85x faster**)  | `24.01 ns` (🚀 **73.07x faster**) | `11.13 ns` (🚀 **157.69x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.84 us` (✅ **1.00x**)   | `5.93 us` (❌ *3.22x slower*)   | `35.04 ns` (🚀 **52.62x faster**)  | `210.27 ns` (🚀 **8.77x faster**)  | `19.43 ns` (🚀 **94.90x faster**) | `11.36 ns` (🚀 **162.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.19 us` (❌ *3.31x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.36 us` (✅ **1.00x**)   | `4.10 us` (❌ *3.01x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `876.55 ns` (✅ **1.00x**) | `2.82 us` (❌ *3.21x slower*)   | `16.45 ns` (🚀 **53.30x faster**)  | `132.96 ns` (🚀 **6.59x faster**)  | `9.26 ns` (🚀 **94.67x faster**)  | `10.71 ns` (🚀 **81.86x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `433.45 us` (✅ **1.00x**) | `1.43 ms` (❌ *3.29x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.17 ns` (❌ *3.43x slower*)     | `135.03 ns` (❌ *17.06x slower*)   | `21.16 ns` (❌ *2.67x slower*)    | `7.92 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `320.00 ns` (❌ *6.90x slower*)    | `8.31 us` (❌ *179.33x slower*)    | `87.44 ns` (❌ *1.89x slower*)    | `46.36 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `365.72 ns` (❌ *9.42x slower*)    | `5.84 us` (❌ *150.33x slower*)    | `78.38 ns` (❌ *2.02x slower*)    | `38.84 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.66 us` (❌ *2.40x slower*)     | `31.03 us` (❌ *4.48x slower*)     | `15.88 us` (❌ *2.29x slower*)    | `6.93 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `685.93 ns` (❌ *10.89x slower*)   | `16.97 us` (❌ *269.35x slower*)   | `141.30 ns` (❌ *2.24x slower*)   | `63.00 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `685.78 ns` (❌ *7.11x slower*)    | `17.25 us` (❌ *178.88x slower*)   | `223.34 ns` (❌ *2.32x slower*)   | `96.42 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.91 ns` (✅ **1.00x**)  | `10.67 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.28 ns` (✅ **1.00x**) | `14.44 ns` (❌ *1.40x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.44 ns` (✅ **1.00x**)  | `4.67 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.12 ns` (✅ **1.00x**)  | `4.24 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `201.70 ns` (✅ **1.00x**) | `265.01 ns` (❌ *1.31x slower*)   | `36.28 ns` (🚀 **5.56x faster**)    | `62.17 ns` (🚀 **3.24x faster**)     | `120.82 ns` (✅ **1.67x faster**)    | `758.51 ns` (❌ *3.76x slower*)    |
| **`serialize_uncompressed`**             | `250.69 ns` (✅ **1.00x**) | `390.56 ns` (❌ *1.56x slower*)   | `36.48 ns` (🚀 **6.87x faster**)    | `62.10 ns` (🚀 **4.04x faster**)     | `121.46 ns` (🚀 **2.06x faster**)    | `764.91 ns` (❌ *3.05x slower*)    |
| **`deserialize_compressed`**             | `421.93 us` (✅ **1.00x**) | `1.33 ms` (❌ *3.15x slower*)     | `57.57 ns` (🚀 **7329.52x faster**) | `130.25 ns` (🚀 **3239.45x faster**) | `291.09 ns` (🚀 **1449.47x faster**) | `1.74 us` (🚀 **242.91x faster**)  |
| **`deserialize_compressed_unchecked`**   | `93.59 us` (✅ **1.00x**)  | `243.05 us` (❌ *2.60x slower*)   | `57.86 ns` (🚀 **1617.56x faster**) | `130.61 ns` (🚀 **716.52x faster**)  | `291.65 ns` (🚀 **320.89x faster**)  | `1.76 us` (🚀 **53.26x faster**)   |
| **`deserialize_uncompressed`**           | `331.34 us` (✅ **1.00x**) | `1.08 ms` (❌ *3.24x slower*)     | `56.06 ns` (🚀 **5910.10x faster**) | `130.88 ns` (🚀 **2531.68x faster**) | `294.16 ns` (🚀 **1126.38x faster**) | `1.71 us` (🚀 **193.23x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `313.12 ns` (✅ **1.00x**) | `669.70 ns` (❌ *2.14x slower*)   | `56.66 ns` (🚀 **5.53x faster**)    | `130.50 ns` (🚀 **2.40x faster**)    | `293.12 ns` (✅ **1.07x faster**)    | `1.73 us` (❌ *5.54x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `3.39 s` (✅ **1.00x**)  | `10.46 s` (❌ *3.08x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `35.18 us` (✅ **1.00x**) | `93.80 us` (❌ *2.67x slower*)   | `246.65 us` (❌ *7.01x slower*)    |
| **`legendre_for_qr`**    | `12.49 us` (✅ **1.00x**) | `44.54 us` (❌ *3.57x slower*)   | `45.81 us` (❌ *3.67x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.51 ns` (✅ **1.00x**)  | `4.67 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `73.33 ns` (✅ **1.00x**) | `127.45 ns` (❌ *1.74x slower*)    |
| **`from_big-endian_bits`**    | `73.71 ns` (✅ **1.00x**) | `130.17 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.62 ns` (✅ **1.00x**)  | `4.76 ns` (✅ **1.03x slower**)    |
| **`equality`**                | `4.94 ns` (✅ **1.00x**)  | `5.75 ns` (❌ *1.16x slower*)      |
| **`is_zero`**                 | `4.24 ns` (✅ **1.00x**)  | `4.69 ns` (✅ **1.11x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.87 ns` (✅ **1.00x**) | `93.20 ns` (❌ *2.12x slower*)    |
| **`into_bigint`** | `27.02 ns` (✅ **1.00x**) | `52.37 ns` (❌ *1.94x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

