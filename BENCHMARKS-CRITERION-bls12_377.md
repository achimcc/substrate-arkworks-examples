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
|        | `212.38 us` (✅ **1.00x**)        | `2.22 ms` (❌ *10.44x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.37 us` (✅ **1.00x**)   | `5.34 us` (❌ *3.91x slower*)   | `31.96 ns` (🚀 **42.73x faster**)  | `215.16 ns` (🚀 **6.35x faster**)  | `22.80 ns` (🚀 **59.87x faster**) | `9.95 ns` (🚀 **137.28x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.41 us` (✅ **1.00x**)   | `5.39 us` (❌ *3.82x slower*)   | `32.46 ns` (🚀 **43.45x faster**)  | `203.04 ns` (🚀 **6.95x faster**)  | `17.84 ns` (🚀 **79.08x faster**) | `10.43 ns` (🚀 **135.30x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `1.00 us` (✅ **1.00x**)   | `3.81 us` (❌ *3.81x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.04 us` (✅ **1.00x**)   | `3.86 us` (❌ *3.71x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `682.06 ns` (✅ **1.00x**) | `2.49 us` (❌ *3.65x slower*)   | `15.34 ns` (🚀 **44.47x faster**)  | `126.62 ns` (🚀 **5.39x faster**)  | `8.99 ns` (🚀 **75.83x faster**)  | `11.16 ns` (🚀 **61.14x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `341.54 us` (✅ **1.00x**) | `1.27 ms` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.56 ns` (❌ *3.86x slower*)     | `123.00 ns` (❌ *17.21x slower*)   | `20.12 ns` (❌ *2.82x slower*)    | `7.15 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `320.01 ns` (❌ *7.15x slower*)    | `7.99 us` (❌ *178.52x slower*)    | `83.73 ns` (❌ *1.87x slower*)    | `44.76 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `298.85 ns` (❌ *7.84x slower*)    | `5.62 us` (❌ *147.54x slower*)    | `70.79 ns` (❌ *1.86x slower*)    | `38.12 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.51 us` (❌ *2.18x slower*)     | `30.14 us` (❌ *3.98x slower*)     | `16.13 us` (❌ *2.13x slower*)    | `7.57 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `686.73 ns` (❌ *10.78x slower*)   | `16.26 us` (❌ *255.36x slower*)   | `133.67 ns` (❌ *2.10x slower*)   | `63.69 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `664.20 ns` (❌ *6.92x slower*)    | `16.18 us` (❌ *168.63x slower*)   | `188.30 ns` (❌ *1.96x slower*)   | `95.93 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**) | `9.44 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.42 ns` (✅ **1.00x**) | `12.89 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**) | `4.85 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.50 ns` (✅ **1.00x**) | `4.65 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `177.27 ns` (✅ **1.00x**) | `255.51 ns` (❌ *1.44x slower*)   | `33.52 ns` (🚀 **5.29x faster**)    | `60.44 ns` (🚀 **2.93x faster**)     | `119.45 ns` (✅ **1.48x faster**)    | `751.71 ns` (❌ *4.24x slower*)    |
| **`serialize_uncompressed`**             | `239.26 ns` (✅ **1.00x**) | `393.45 ns` (❌ *1.64x slower*)   | `33.40 ns` (🚀 **7.16x faster**)    | `60.27 ns` (🚀 **3.97x faster**)     | `119.48 ns` (🚀 **2.00x faster**)    | `752.90 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `335.69 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.47x slower*)     | `56.10 ns` (🚀 **5983.69x faster**) | `112.33 ns` (🚀 **2988.44x faster**) | `249.43 ns` (🚀 **1345.84x faster**) | `1.51 us` (🚀 **222.63x faster**)  |
| **`deserialize_compressed_unchecked`**   | `77.88 us` (✅ **1.00x**)  | `207.29 us` (❌ *2.66x slower*)   | `56.11 ns` (🚀 **1388.00x faster**) | `112.31 ns` (🚀 **693.42x faster**)  | `247.80 ns` (🚀 **314.27x faster**)  | `1.51 us` (🚀 **51.67x faster**)   |
| **`deserialize_uncompressed`**           | `258.29 us` (✅ **1.00x**) | `952.62 us` (❌ *3.69x slower*)   | `56.08 ns` (🚀 **4605.96x faster**) | `111.88 ns` (🚀 **2308.66x faster**) | `248.93 ns` (🚀 **1037.60x faster**) | `1.50 us` (🚀 **171.93x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `273.74 ns` (✅ **1.00x**) | `569.13 ns` (❌ *2.08x slower*)   | `56.05 ns` (🚀 **4.88x faster**)    | `111.93 ns` (🚀 **2.45x faster**)    | `247.32 ns` (✅ **1.11x faster**)    | `1.50 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.72 s` (✅ **1.00x**)  | `9.56 s` (❌ *3.52x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `33.21 us` (✅ **1.00x**) | `77.27 us` (❌ *2.33x slower*)   | `206.80 us` (❌ *6.23x slower*)    |
| **`legendre_for_qr`**    | `11.42 us` (✅ **1.00x**) | `35.07 us` (❌ *3.07x slower*)   | `35.36 us` (❌ *3.10x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.78 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `75.58 ns` (✅ **1.00x**) | `129.46 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `73.65 ns` (✅ **1.00x**) | `129.35 ns` (❌ *1.76x slower*)    |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)  | `5.17 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `5.38 ns` (✅ **1.00x**)  | `5.66 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `4.69 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.08 ns` (✅ **1.00x**) | `95.49 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `26.09 ns` (✅ **1.00x**) | `49.91 ns` (❌ *1.91x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

