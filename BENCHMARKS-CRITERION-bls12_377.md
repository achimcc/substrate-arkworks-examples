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
|        | `275.95 us` (✅ **1.00x**)        | `2.58 ms` (❌ *9.36x slower*)      |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.90 us` (✅ **1.00x**)   | `6.02 us` (❌ *3.17x slower*)   | `34.00 ns` (🚀 **55.89x faster**)  | `225.45 ns` (🚀 **8.43x faster**)  | `23.66 ns` (🚀 **80.34x faster**) | `11.22 ns` (🚀 **169.43x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**)   | `6.01 us` (❌ *3.12x slower*)   | `34.36 ns` (🚀 **56.06x faster**)  | `213.65 ns` (🚀 **9.02x faster**)  | `19.88 ns` (🚀 **96.90x faster**) | `11.48 ns` (🚀 **167.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.31 us` (✅ **1.00x**)   | `4.31 us` (❌ *3.28x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.40 us` (✅ **1.00x**)   | `4.34 us` (❌ *3.09x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `896.32 ns` (✅ **1.00x**) | `2.95 us` (❌ *3.30x slower*)   | `16.91 ns` (🚀 **53.02x faster**)  | `133.33 ns` (🚀 **6.72x faster**)  | `9.51 ns` (🚀 **94.23x faster**)  | `10.62 ns` (🚀 **84.37x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `456.54 us` (✅ **1.00x**) | `1.46 ms` (❌ *3.20x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `28.04 ns` (❌ *3.46x slower*)     | `129.97 ns` (❌ *16.05x slower*)   | `22.49 ns` (❌ *2.78x slower*)    | `8.10 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `327.26 ns` (❌ *7.08x slower*)    | `8.34 us` (❌ *180.40x slower*)    | `91.14 ns` (❌ *1.97x slower*)    | `46.25 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `369.56 ns` (❌ *9.34x slower*)    | `6.04 us` (❌ *152.49x slower*)    | `80.63 ns` (❌ *2.04x slower*)    | `39.58 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.20 us` (❌ *2.25x slower*)     | `31.94 us` (❌ *4.43x slower*)     | `15.99 us` (❌ *2.22x slower*)    | `7.21 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `707.97 ns` (❌ *10.79x slower*)   | `17.63 us` (❌ *268.77x slower*)   | `140.52 ns` (❌ *2.14x slower*)   | `65.59 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `689.95 ns` (❌ *6.97x slower*)    | `17.40 us` (❌ *175.79x slower*)   | `228.25 ns` (❌ *2.31x slower*)   | `98.97 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.97 ns` (✅ **1.00x**)  | `10.71 ns` (❌ *1.34x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.36 ns` (✅ **1.00x**) | `14.92 ns` (❌ *1.44x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.58 ns` (✅ **1.00x**)  | `4.79 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.37 ns` (✅ **1.00x**)  | `4.27 ns` (✅ **1.02x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `209.70 ns` (✅ **1.00x**) | `268.52 ns` (❌ *1.28x slower*)   | `37.66 ns` (🚀 **5.57x faster**)    | `64.13 ns` (🚀 **3.27x faster**)     | `121.53 ns` (✅ **1.73x faster**)    | `769.78 ns` (❌ *3.67x slower*)    |
| **`serialize_uncompressed`**             | `263.20 ns` (✅ **1.00x**) | `404.47 ns` (❌ *1.54x slower*)   | `37.36 ns` (🚀 **7.04x faster**)    | `61.77 ns` (🚀 **4.26x faster**)     | `122.67 ns` (🚀 **2.15x faster**)    | `779.29 ns` (❌ *2.96x slower*)    |
| **`deserialize_compressed`**             | `439.24 us` (✅ **1.00x**) | `1.37 ms` (❌ *3.13x slower*)     | `57.57 ns` (🚀 **7629.57x faster**) | `133.20 ns` (🚀 **3297.69x faster**) | `300.32 ns` (🚀 **1462.57x faster**) | `1.79 us` (🚀 **245.07x faster**)  |
| **`deserialize_compressed_unchecked`**   | `96.80 us` (✅ **1.00x**)  | `255.96 us` (❌ *2.64x slower*)   | `58.28 ns` (🚀 **1660.89x faster**) | `131.31 ns` (🚀 **737.17x faster**)  | `299.39 ns` (🚀 **323.31x faster**)  | `1.81 us` (🚀 **53.56x faster**)   |
| **`deserialize_uncompressed`**           | `341.25 us` (✅ **1.00x**) | `1.12 ms` (❌ *3.27x slower*)     | `58.04 ns` (🚀 **5879.79x faster**) | `134.90 ns` (🚀 **2529.55x faster**) | `299.06 ns` (🚀 **1141.06x faster**) | `1.78 us` (🚀 **191.82x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `325.18 ns` (✅ **1.00x**) | `676.46 ns` (❌ *2.08x slower*)   | `58.72 ns` (🚀 **5.54x faster**)    | `132.32 ns` (🚀 **2.46x faster**)    | `297.15 ns` (✅ **1.09x faster**)    | `1.77 us` (❌ *5.43x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `3.47 s` (✅ **1.00x**)  | `10.62 s` (❌ *3.06x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `36.48 us` (✅ **1.00x**) | `93.65 us` (❌ *2.57x slower*)   | `251.53 us` (❌ *6.90x slower*)    |
| **`legendre_for_qr`**    | `12.69 us` (✅ **1.00x**) | `46.01 us` (❌ *3.63x slower*)   | `46.41 us` (❌ *3.66x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.60 ns` (✅ **1.00x**)  | `4.78 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `76.25 ns` (✅ **1.00x**) | `132.36 ns` (❌ *1.74x slower*)    |
| **`from_big-endian_bits`**    | `76.94 ns` (✅ **1.00x**) | `131.80 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.57 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `5.01 ns` (✅ **1.00x**)  | `5.79 ns` (❌ *1.16x slower*)      |
| **`is_zero`**                 | `4.38 ns` (✅ **1.00x**)  | `4.74 ns` (✅ **1.08x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.49 ns` (✅ **1.00x**) | `97.84 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `27.07 ns` (✅ **1.00x**) | `52.49 ns` (❌ *1.94x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

