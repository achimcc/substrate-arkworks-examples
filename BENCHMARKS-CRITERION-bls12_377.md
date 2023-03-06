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
|        | `193.90 us` (✅ **1.00x**)        | `2.04 ms` (❌ *10.53x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.23 us` (✅ **1.00x**)   | `4.83 us` (❌ *3.92x slower*)   | `23.22 ns` (🚀 **53.01x faster**) | `180.79 ns` (🚀 **6.81x faster**)  | `12.49 ns` (🚀 **98.56x faster**) | `8.70 ns` (🚀 **141.57x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.88 us` (❌ *3.84x slower*)   | `23.23 ns` (🚀 **54.70x faster**) | `159.19 ns` (🚀 **7.98x faster**)  | `12.75 ns` (🚀 **99.65x faster**) | `8.80 ns` (🚀 **144.36x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `896.84 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.84x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `924.54 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.76x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `594.02 ns` (✅ **1.00x**) | `2.28 us` (❌ *3.83x slower*)   | `12.37 ns` (🚀 **48.00x faster**) | `67.26 ns` (🚀 **8.83x faster**)   | `7.14 ns` (🚀 **83.20x faster**)  | `5.86 ns` (🚀 **101.38x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `319.07 us` (✅ **1.00x**) | `1.17 ms` (❌ *3.67x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.61 ns` (❌ *3.66x slower*)    | `99.93 ns` (❌ *16.20x slower*)    | `18.30 ns` (❌ *2.97x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.39 ns` (❌ *6.31x slower*)   | `7.13 us` (❌ *165.71x slower*)    | `75.47 ns` (❌ *1.75x slower*)    | `43.04 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `244.06 ns` (❌ *6.60x slower*)   | `5.05 us` (❌ *136.42x slower*)    | `67.05 ns` (❌ *1.81x slower*)    | `37.00 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.12 us` (❌ *2.15x slower*)    | `27.49 us` (❌ *3.91x slower*)     | `14.75 us` (❌ *2.10x slower*)    | `7.03 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.69 ns` (❌ *9.64x slower*)   | `14.64 us` (❌ *239.05x slower*)   | `117.95 ns` (❌ *1.93x slower*)   | `61.24 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `580.48 ns` (❌ *6.34x slower*)   | `14.54 us` (❌ *158.88x slower*)   | `164.12 ns` (❌ *1.79x slower*)   | `91.54 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.41 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.53 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `156.18 ns` (✅ **1.00x**) | `218.62 ns` (❌ *1.40x slower*)   | `31.29 ns` (🚀 **4.99x faster**)    | `58.91 ns` (🚀 **2.65x faster**)    | `109.85 ns` (✅ **1.42x faster**)    | `706.89 ns` (❌ *4.53x slower*)    |
| **`serialize_uncompressed`**             | `210.96 ns` (✅ **1.00x**) | `332.16 ns` (❌ *1.57x slower*)   | `31.72 ns` (🚀 **6.65x faster**)    | `56.69 ns` (🚀 **3.72x faster**)    | `109.83 ns` (🚀 **1.92x faster**)    | `707.83 ns` (❌ *3.36x slower*)    |
| **`deserialize_compressed`**             | `311.46 us` (✅ **1.00x**) | `1.07 ms` (❌ *3.42x slower*)     | `52.42 ns` (🚀 **5941.43x faster**) | `92.87 ns` (🚀 **3353.89x faster**) | `216.83 ns` (🚀 **1436.42x faster**) | `1.27 us` (🚀 **245.09x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.26 us` (✅ **1.00x**)  | `184.10 us` (❌ *2.70x slower*)   | `52.41 ns` (🚀 **1302.39x faster**) | `92.54 ns` (🚀 **737.68x faster**)  | `216.48 ns` (🚀 **315.33x faster**)  | `1.26 us` (🚀 **54.12x faster**)   |
| **`deserialize_uncompressed`**           | `243.34 us` (✅ **1.00x**) | `878.75 us` (❌ *3.61x slower*)   | `52.38 ns` (🚀 **4645.92x faster**) | `93.08 ns` (🚀 **2614.16x faster**) | `216.49 ns` (🚀 **1123.99x faster**) | `1.26 us` (🚀 **192.38x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.21 ns` (✅ **1.00x**) | `493.80 ns` (❌ *2.21x slower*)   | `52.37 ns` (🚀 **4.26x faster**)    | `92.53 ns` (🚀 **2.41x faster**)    | `216.06 ns` (✅ **1.03x faster**)    | `1.27 us` (❌ *5.69x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.35 s` (✅ **1.00x**)  | `8.47 s` (❌ *3.61x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.14 us` (✅ **1.00x**) | `67.81 us` (❌ *2.18x slower*)   | `183.02 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `10.94 us` (✅ **1.00x**) | `31.74 us` (❌ *2.90x slower*)   | `31.70 us` (❌ *2.90x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.30 ns` (✅ **1.00x**) | `83.67 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.24 ns` (✅ **1.00x**) | `83.67 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.02x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.08 ns` (✅ **1.00x**) | `75.20 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `23.38 ns` (✅ **1.00x**) | `47.02 ns` (❌ *2.01x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.14 ns` (✅ **1.00x**)                 | `256.05 us` (❌ *28009.16x slower*)      | `674.32 us` (❌ *73762.85x slower*)   | `1.27 ms` (❌ *138744.10x slower*)             | `2.22 ms` (❌ *243032.23x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

