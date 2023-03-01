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
|        | `176.62 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.47x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.16 us` (✅ **1.00x**)   | `4.44 us` (❌ *3.84x slower*)   | `26.53 ns` (🚀 **43.57x faster**)  | `177.83 ns` (🚀 **6.50x faster**)  | `19.00 ns` (🚀 **60.85x faster**) | `8.29 ns` (🚀 **139.38x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.19 us` (✅ **1.00x**)   | `4.49 us` (❌ *3.78x slower*)   | `26.90 ns` (🚀 **44.15x faster**)  | `170.01 ns` (🚀 **6.99x faster**)  | `14.59 ns` (🚀 **81.44x faster**) | `8.72 ns` (🚀 **136.22x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `832.43 ns` (✅ **1.00x**) | `3.17 us` (❌ *3.81x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `756.32 ns` (✅ **1.00x**) | `3.21 us` (❌ *4.25x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `567.98 ns` (✅ **1.00x**) | `2.07 us` (❌ *3.65x slower*)   | `12.81 ns` (🚀 **44.34x faster**)  | `105.29 ns` (🚀 **5.39x faster**)  | `7.49 ns` (🚀 **75.88x faster**)  | `9.11 ns` (🚀 **62.33x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `283.66 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.09 ns` (❌ *3.88x slower*)     | `102.21 ns` (❌ *17.17x slower*)   | `16.76 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `234.60 ns` (❌ *6.30x slower*)    | `5.86 us` (❌ *157.32x slower*)    | `69.71 ns` (❌ *1.87x slower*)    | `37.27 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `250.27 ns` (❌ *7.89x slower*)    | `4.69 us` (❌ *147.76x slower*)    | `58.94 ns` (❌ *1.86x slower*)    | `31.73 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.75 us` (❌ *2.18x slower*)     | `25.10 us` (❌ *3.98x slower*)     | `11.85 us` (❌ *1.88x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `571.92 ns` (❌ *12.21x slower*)   | `13.55 us` (❌ *289.30x slower*)   | `111.40 ns` (❌ *2.38x slower*)   | `46.83 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `488.43 ns` (❌ *6.92x slower*)    | `11.89 us` (❌ *168.55x slower*)   | `156.73 ns` (❌ *2.22x slower*)   | `70.56 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `5.76 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.36x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.85 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `148.08 ns` (✅ **1.00x**) | `214.02 ns` (❌ *1.45x slower*)   | `27.91 ns` (🚀 **5.31x faster**)    | `50.35 ns` (🚀 **2.94x faster**)    | `100.11 ns` (✅ **1.48x faster**)    | `628.46 ns` (❌ *4.24x slower*)    |
| **`serialize_uncompressed`**             | `199.20 ns` (✅ **1.00x**) | `319.47 ns` (❌ *1.60x slower*)   | `27.73 ns` (🚀 **7.18x faster**)    | `50.16 ns` (🚀 **3.97x faster**)    | `99.64 ns` (🚀 **2.00x faster**)     | `628.40 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `280.01 us` (✅ **1.00x**) | `967.41 us` (❌ *3.45x slower*)   | `46.70 ns` (🚀 **5995.65x faster**) | `93.58 ns` (🚀 **2992.09x faster**) | `207.74 ns` (🚀 **1347.88x faster**) | `1.27 us` (🚀 **219.94x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.99 us` (✅ **1.00x**)  | `173.21 us` (❌ *2.67x slower*)   | `46.70 ns` (🚀 **1391.58x faster**) | `94.06 ns` (🚀 **690.95x faster**)  | `206.37 ns` (🚀 **314.91x faster**)  | `1.27 us` (🚀 **51.05x faster**)   |
| **`deserialize_uncompressed`**           | `215.30 us` (✅ **1.00x**) | `792.20 us` (❌ *3.68x slower*)   | `46.67 ns` (🚀 **4613.51x faster**) | `93.56 ns` (🚀 **2301.24x faster**) | `207.82 ns` (🚀 **1035.98x faster**) | `1.27 us` (🚀 **169.23x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `227.91 ns` (✅ **1.00x**) | `474.01 ns` (❌ *2.08x slower*)   | `46.68 ns` (🚀 **4.88x faster**)    | `82.51 ns` (🚀 **2.76x faster**)    | `207.60 ns` (✅ **1.10x faster**)    | `1.27 us` (❌ *5.57x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.31 s` (✅ **1.00x**)  | `7.93 s` (❌ *3.44x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.67 us` (✅ **1.00x**) | `64.51 us` (❌ *2.33x slower*)   | `172.32 us` (❌ *6.23x slower*)    |
| **`legendre_for_qr`**    | `9.51 us` (✅ **1.00x**)  | `29.19 us` (❌ *3.07x slower*)   | `25.93 us` (❌ *2.73x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.79 ns` (✅ **1.00x**) | `107.82 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `60.82 ns` (✅ **1.00x**) | `108.08 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.72 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `31.73 ns` (✅ **1.00x**) | `79.14 ns` (❌ *2.49x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.52 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

