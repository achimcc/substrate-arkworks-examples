# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381](#sample_bls12_381)
    - [arithmetic_for_bls12_381](#arithmetic_for_bls12_381)
    - [serialization_for_bls12_381](#serialization_for_bls12_381)
    - [msm_for_bls12_381](#msm_for_bls12_381)
    - [squareroot_for_bls12_381](#squareroot_for_bls12_381)
    - [bitwise_operations_for_bls12_381](#bitwise_operations_for_bls12_381)
    - [conversions_for_bls12_381](#conversions_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `220.11 us` (✅ **1.00x**)        | `1.96 ms` (❌ *8.92x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.42 us` (✅ **1.00x**)   | `4.32 us` (❌ *3.05x slower*)   | `30.76 ns` (🚀 **46.05x faster**) | `203.37 ns` (🚀 **6.96x faster**)  | `22.60 ns` (🚀 **62.66x faster**) | `9.81 ns` (🚀 **144.33x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.49 us` (✅ **1.00x**)   | `4.51 us` (❌ *3.03x slower*)   | `31.66 ns` (🚀 **47.11x faster**) | `192.50 ns` (🚀 **7.75x faster**)  | `18.11 ns` (🚀 **82.37x faster**) | `10.62 ns` (🚀 **140.44x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `1.03 us` (✅ **1.00x**)   | `3.25 us` (❌ *3.15x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.06 us` (✅ **1.00x**)   | `3.20 us` (❌ *3.02x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `707.83 ns` (✅ **1.00x**) | `1.99 us` (❌ *2.81x slower*)   | `15.38 ns` (🚀 **46.03x faster**) | `120.92 ns` (🚀 **5.85x faster**)  | `9.39 ns` (🚀 **75.34x faster**)  | `6.11 ns` (🚀 **115.80x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `350.80 us` (✅ **1.00x**) | `1.07 ms` (❌ *3.05x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `25.87 ns` (❌ *3.60x slower*)    | `124.32 ns` (❌ *17.28x slower*)   | `20.47 ns` (❌ *2.85x slower*)    | `7.19 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `270.33 ns` (❌ *5.85x slower*)   | `6.65 us` (❌ *143.88x slower*)    | `78.19 ns` (❌ *1.69x slower*)    | `46.20 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `237.50 ns` (❌ *5.70x slower*)   | `5.02 us` (❌ *120.39x slower*)    | `67.79 ns` (❌ *1.63x slower*)    | `41.70 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `17.82 us` (❌ *2.59x slower*)    | `26.38 us` (❌ *3.83x slower*)     | `15.32 us` (❌ *2.23x slower*)    | `6.88 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `599.21 ns` (❌ *6.07x slower*)   | `13.69 us` (❌ *138.62x slower*)   | `125.96 ns` (❌ *1.28x slower*)   | `98.73 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `553.66 ns` (❌ *5.65x slower*)   | `14.32 us` (❌ *146.16x slower*)   | `173.55 ns` (❌ *1.77x slower*)   | `97.96 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.65 ns` (✅ **1.00x**) | `10.70 ns` (❌ *1.40x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.83 ns` (✅ **1.00x**) | `13.63 ns` (❌ *1.39x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.38 ns` (✅ **1.00x**) | `4.36 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.28 ns` (✅ **1.00x**) | `4.11 ns` (✅ **1.04x faster**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `163.65 ns` (✅ **1.00x**) | `236.06 ns` (❌ *1.44x slower*)   | `36.51 ns` (🚀 **4.48x faster**)    | `56.00 ns` (🚀 **2.92x faster**)     | `113.82 ns` (✅ **1.44x faster**)   | `801.35 ns` (❌ *4.90x slower*)    |
| **`serialize_uncompressed`**             | `214.18 ns` (✅ **1.00x**) | `321.76 ns` (❌ *1.50x slower*)   | `36.07 ns` (🚀 **5.94x faster**)    | `59.04 ns` (🚀 **3.63x faster**)     | `114.21 ns` (🚀 **1.88x faster**)   | `703.40 ns` (❌ *3.28x slower*)    |
| **`deserialize_compressed`**             | `145.84 us` (✅ **1.00x**) | `293.78 us` (❌ *2.01x slower*)   | `58.37 ns` (🚀 **2498.56x faster**) | `102.37 ns` (🚀 **1424.65x faster**) | `247.75 ns` (🚀 **588.68x faster**) | `1.44 us` (🚀 **101.27x faster**)  |
| **`deserialize_compressed_unchecked`**   | `47.67 us` (✅ **1.00x**)  | `151.54 us` (❌ *3.18x slower*)   | `60.96 ns` (🚀 **781.92x faster**)  | `100.86 ns` (🚀 **472.62x faster**)  | `247.18 ns` (🚀 **192.85x faster**) | `1.45 us` (🚀 **32.95x faster**)   |
| **`deserialize_uncompressed`**           | `99.48 us` (✅ **1.00x**)  | `141.90 us` (❌ *1.43x slower*)   | `57.10 ns` (🚀 **1742.15x faster**) | `104.39 ns` (🚀 **953.03x faster**)  | `237.09 ns` (🚀 **419.61x faster**) | `1.49 us` (🚀 **66.60x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `244.29 ns` (✅ **1.00x**) | `506.75 ns` (❌ *2.07x slower*)   | `58.12 ns` (🚀 **4.20x faster**)    | `106.07 ns` (🚀 **2.30x faster**)    | `248.45 ns` (✅ **1.02x slower**)   | `1.44 us` (❌ *5.91x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.81 s` (✅ **1.00x**)  | `8.06 s` (❌ *2.87x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.31 us` (✅ **1.00x**) | `43.72 us` (❌ *1.73x slower*)   | `149.19 us` (❌ *5.89x slower*)    |
| **`legendre_for_qr`**    | `14.37 us` (✅ **1.00x**) | `43.16 us` (❌ *3.00x slower*)   | `43.25 us` (❌ *3.01x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.53 ns` (✅ **1.00x**)  | `4.60 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `72.84 ns` (✅ **1.00x**) | `123.19 ns` (❌ *1.69x slower*)    |
| **`from_big-endian_bits`**    | `72.34 ns` (✅ **1.00x**) | `124.20 ns` (❌ *1.72x slower*)    |
| **`comparison`**              | `4.68 ns` (✅ **1.00x**)  | `5.05 ns` (✅ **1.08x slower**)    |
| **`equality`**                | `5.07 ns` (✅ **1.00x**)  | `5.88 ns` (❌ *1.16x slower*)      |
| **`is_zero`**                 | `4.31 ns` (✅ **1.00x**)  | `4.43 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.58 ns` (✅ **1.00x**) | `88.71 ns` (❌ *2.13x slower*)    |
| **`into_bigint`** | `26.08 ns` (✅ **1.00x**) | `47.30 ns` (❌ *1.81x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

