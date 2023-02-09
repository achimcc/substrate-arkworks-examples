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
|        | `221.85 us` (✅ **1.00x**)        | `2.32 ms` (❌ *10.46x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.51 us` (✅ **1.00x**)   | `5.57 us` (❌ *3.70x slower*)   | `33.22 ns` (🚀 **45.35x faster**)  | `222.59 ns` (🚀 **6.77x faster**)  | `23.77 ns` (🚀 **63.37x faster**) | `10.72 ns` (🚀 **140.48x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.55 us` (✅ **1.00x**)   | `5.78 us` (❌ *3.72x slower*)   | `34.12 ns` (🚀 **45.53x faster**)  | `213.31 ns` (🚀 **7.28x faster**)  | `18.84 ns` (🚀 **82.44x faster**) | `11.16 ns` (🚀 **139.15x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `1.04 us` (✅ **1.00x**)   | `4.27 us` (❌ *4.12x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.07 us` (✅ **1.00x**)   | `4.18 us` (❌ *3.90x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `736.09 ns` (✅ **1.00x**) | `2.72 us` (❌ *3.69x slower*)   | `16.52 ns` (🚀 **44.55x faster**)  | `129.77 ns` (🚀 **5.67x faster**)  | `13.98 ns` (🚀 **52.66x faster**) | `11.33 ns` (🚀 **64.96x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `362.60 us` (✅ **1.00x**) | `1.34 ms` (❌ *3.69x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `29.63 ns` (❌ *3.98x slower*)     | `127.10 ns` (❌ *17.09x slower*)   | `21.41 ns` (❌ *2.88x slower*)    | `7.44 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `347.00 ns` (❌ *7.44x slower*)    | `8.33 us` (❌ *178.54x slower*)    | `88.51 ns` (❌ *1.90x slower*)    | `46.65 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `315.91 ns` (❌ *7.77x slower*)    | `5.85 us` (❌ *143.72x slower*)    | `73.69 ns` (❌ *1.81x slower*)    | `40.68 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `17.46 us` (❌ *2.18x slower*)     | `31.57 us` (❌ *3.94x slower*)     | `17.00 us` (❌ *2.12x slower*)    | `8.01 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `732.68 ns` (❌ *11.05x slower*)   | `18.23 us` (❌ *274.88x slower*)   | `140.24 ns` (❌ *2.11x slower*)   | `66.33 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `691.73 ns` (❌ *6.94x slower*)    | `16.94 us` (❌ *169.89x slower*)   | `197.73 ns` (❌ *1.98x slower*)   | `99.70 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `8.16 ns` (✅ **1.00x**) | `10.55 ns` (❌ *1.29x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.81 ns` (✅ **1.00x**) | `14.09 ns` (❌ *1.44x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.96 ns` (✅ **1.00x**) | `5.05 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.79 ns` (✅ **1.00x**) | `4.68 ns` (✅ **1.02x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `192.88 ns` (✅ **1.00x**) | `280.45 ns` (❌ *1.45x slower*)   | `36.23 ns` (🚀 **5.32x faster**)    | `65.09 ns` (🚀 **2.96x faster**)     | `124.96 ns` (✅ **1.54x faster**)    | `792.47 ns` (❌ *4.11x slower*)    |
| **`serialize_uncompressed`**             | `247.61 ns` (✅ **1.00x**) | `436.74 ns` (❌ *1.76x slower*)   | `34.73 ns` (🚀 **7.13x faster**)    | `62.69 ns` (🚀 **3.95x faster**)     | `124.62 ns` (🚀 **1.99x faster**)    | `828.90 ns` (❌ *3.35x slower*)    |
| **`deserialize_compressed`**             | `360.95 us` (✅ **1.00x**) | `1.22 ms` (❌ *3.37x slower*)     | `59.80 ns` (🚀 **6036.07x faster**) | `118.13 ns` (🚀 **3055.44x faster**) | `259.54 ns` (🚀 **1390.73x faster**) | `1.57 us` (🚀 **229.80x faster**)  |
| **`deserialize_compressed_unchecked`**   | `81.26 us` (✅ **1.00x**)  | `224.97 us` (❌ *2.77x slower*)   | `60.67 ns` (🚀 **1339.52x faster**) | `120.22 ns` (🚀 **675.99x faster**)  | `259.50 ns` (🚀 **313.16x faster**)  | `1.58 us` (🚀 **51.53x faster**)   |
| **`deserialize_uncompressed`**           | `277.48 us` (✅ **1.00x**) | `992.84 us` (❌ *3.58x slower*)   | `59.68 ns` (🚀 **4649.43x faster**) | `117.89 ns` (🚀 **2353.65x faster**) | `272.20 ns` (🚀 **1019.42x faster**) | `1.59 us` (🚀 **174.61x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `310.78 ns` (✅ **1.00x**) | `591.52 ns` (❌ *1.90x slower*)   | `59.55 ns` (🚀 **5.22x faster**)    | `117.91 ns` (🚀 **2.64x faster**)    | `259.07 ns` (✅ **1.20x faster**)    | `1.58 us` (❌ *5.08x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `2.86 s` (✅ **1.00x**)  | `10.04 s` (❌ *3.51x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `35.05 us` (✅ **1.00x**) | `80.76 us` (❌ *2.30x slower*)   | `223.11 us` (❌ *6.37x slower*)    |
| **`legendre_for_qr`**    | `12.34 us` (✅ **1.00x**) | `36.30 us` (❌ *2.94x slower*)   | `37.22 us` (❌ *3.02x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.15 ns` (✅ **1.00x**)  | `5.45 ns` (✅ **1.06x slower**)    |
| **`from_little-endian_bits`** | `79.13 ns` (✅ **1.00x**) | `141.69 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `76.69 ns` (✅ **1.00x**) | `136.70 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `5.11 ns` (✅ **1.00x**)  | `5.39 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `5.64 ns` (✅ **1.00x**)  | `6.13 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.99 ns` (✅ **1.00x**)  | `5.04 ns` (✅ **1.01x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.93 ns` (✅ **1.00x**) | `98.36 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `27.20 ns` (✅ **1.00x**) | `51.62 ns` (❌ *1.90x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

