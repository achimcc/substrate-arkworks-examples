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
|        | `177.98 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.42x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.14 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.90x slower*)   | `26.75 ns` (🚀 **42.65x faster**)  | `178.61 ns` (🚀 **6.39x faster**)  | `18.96 ns` (🚀 **60.17x faster**) | `8.29 ns` (🚀 **137.54x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.18 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.82x slower*)   | `27.28 ns` (🚀 **43.14x faster**)  | `170.90 ns` (🚀 **6.89x faster**)  | `14.80 ns` (🚀 **79.49x faster**) | `8.67 ns` (🚀 **135.71x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `833.88 ns` (✅ **1.00x**) | `3.19 us` (❌ *3.82x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `855.12 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.77x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `570.83 ns` (✅ **1.00x**) | `2.08 us` (❌ *3.64x slower*)   | `12.80 ns` (🚀 **44.61x faster**)  | `99.35 ns` (🚀 **5.75x faster**)   | `7.48 ns` (🚀 **76.33x faster**)  | `9.12 ns` (🚀 **62.57x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.14 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.61 ns` (❌ *3.79x slower*)     | `101.81 ns` (❌ *17.08x slower*)   | `16.76 ns` (❌ *2.81x slower*)    | `5.96 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `266.97 ns` (❌ *7.15x slower*)    | `6.66 us` (❌ *178.40x slower*)    | `69.39 ns` (❌ *1.86x slower*)    | `37.32 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.71 ns` (❌ *7.63x slower*)    | `4.68 us` (❌ *144.30x slower*)    | `59.34 ns` (❌ *1.83x slower*)    | `32.45 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `14.03 us` (❌ *2.23x slower*)     | `25.32 us` (❌ *4.02x slower*)     | `13.71 us` (❌ *2.17x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `569.71 ns` (❌ *10.74x slower*)   | `13.59 us` (❌ *256.35x slower*)   | `111.62 ns` (❌ *2.10x slower*)   | `53.03 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.34 ns` (❌ *6.94x slower*)    | `13.46 us` (❌ *168.78x slower*)   | `158.87 ns` (❌ *1.99x slower*)   | `79.74 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**) | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.63 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.84 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `147.00 ns` (✅ **1.00x**) | `211.24 ns` (❌ *1.44x slower*)   | `28.04 ns` (🚀 **5.24x faster**)    | `50.06 ns` (🚀 **2.94x faster**)    | `99.44 ns` (✅ **1.48x faster**)     | `629.04 ns` (❌ *4.28x slower*)    |
| **`serialize_uncompressed`**             | `197.63 ns` (✅ **1.00x**) | `318.12 ns` (❌ *1.61x slower*)   | `27.99 ns` (🚀 **7.06x faster**)    | `50.07 ns` (🚀 **3.95x faster**)    | `99.51 ns` (🚀 **1.99x faster**)     | `629.06 ns` (❌ *3.18x slower*)    |
| **`deserialize_compressed`**             | `281.29 us` (✅ **1.00x**) | `970.01 us` (❌ *3.45x slower*)   | `46.55 ns` (🚀 **6042.20x faster**) | `93.23 ns` (🚀 **3017.18x faster**) | `207.73 ns` (🚀 **1354.15x faster**) | `1.25 us` (🚀 **225.86x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.05 us` (✅ **1.00x**)  | `173.88 us` (❌ *2.67x slower*)   | `46.54 ns` (🚀 **1397.52x faster**) | `93.22 ns` (🚀 **697.76x faster**)  | `206.53 ns` (🚀 **314.95x faster**)  | `1.25 us` (🚀 **52.05x faster**)   |
| **`deserialize_uncompressed`**           | `216.49 us` (✅ **1.00x**) | `793.92 us` (❌ *3.67x slower*)   | `46.51 ns` (🚀 **4654.90x faster**) | `93.20 ns` (🚀 **2322.83x faster**) | `206.51 ns` (🚀 **1048.33x faster**) | `1.25 us` (🚀 **173.80x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.23 ns` (✅ **1.00x**) | `469.20 ns` (❌ *2.07x slower*)   | `46.52 ns` (🚀 **4.86x faster**)    | `93.20 ns` (🚀 **2.43x faster**)    | `206.48 ns` (✅ **1.10x faster**)    | `1.25 us` (❌ *5.52x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.27 s` (✅ **1.00x**)  | `7.93 s` (❌ *3.49x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.67 us` (✅ **1.00x**) | `64.80 us` (❌ *2.34x slower*)   | `173.06 us` (❌ *6.26x slower*)    |
| **`legendre_for_qr`**    | `9.55 us` (✅ **1.00x**)  | `29.27 us` (❌ *3.07x slower*)   | `29.75 us` (❌ *3.12x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.12 ns` (✅ **1.00x**) | `107.84 ns` (❌ *1.76x slower*)    |
| **`from_big-endian_bits`**    | `60.89 ns` (✅ **1.00x**) | `107.85 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)  | `4.33 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)  | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.94 ns` (✅ **1.00x**) | `79.37 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.64 ns` (✅ **1.00x**) | `41.21 ns` (❌ *1.90x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

