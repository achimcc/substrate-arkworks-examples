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
|        | `193.71 us` (✅ **1.00x**)        | `2.04 ms` (❌ *10.54x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.23 us` (✅ **1.00x**)   | `4.82 us` (❌ *3.92x slower*)   | `23.29 ns` (🚀 **52.83x faster**) | `180.81 ns` (🚀 **6.80x faster**)  | `12.49 ns` (🚀 **98.50x faster**) | `8.70 ns` (🚀 **141.50x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.88 us` (❌ *3.85x slower*)   | `23.24 ns` (🚀 **54.60x faster**) | `159.99 ns` (🚀 **7.93x faster**)  | `12.74 ns` (🚀 **99.56x faster**) | `8.79 ns` (🚀 **144.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `894.64 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.85x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `924.30 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.76x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `593.60 ns` (✅ **1.00x**) | `2.27 us` (❌ *3.83x slower*)   | `12.31 ns` (🚀 **48.22x faster**) | `68.86 ns` (🚀 **8.62x faster**)   | `7.15 ns` (🚀 **83.02x faster**)  | `5.86 ns` (🚀 **101.33x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `318.97 us` (✅ **1.00x**) | `1.17 ms` (❌ *3.66x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.78 ns` (❌ *3.69x slower*)    | `100.50 ns` (❌ *16.30x slower*)   | `18.96 ns` (❌ *3.07x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.30 ns` (❌ *6.29x slower*)   | `7.11 us` (❌ *164.94x slower*)    | `75.48 ns` (❌ *1.75x slower*)    | `43.10 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `243.85 ns` (❌ *6.59x slower*)   | `5.05 us` (❌ *136.50x slower*)    | `66.91 ns` (❌ *1.81x slower*)    | `37.01 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.10 us` (❌ *2.15x slower*)    | `27.50 us` (❌ *3.91x slower*)     | `14.79 us` (❌ *2.10x slower*)    | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.80 ns` (❌ *9.66x slower*)   | `14.64 us` (❌ *239.48x slower*)   | `117.96 ns` (❌ *1.93x slower*)   | `61.15 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `578.91 ns` (❌ *6.32x slower*)   | `14.55 us` (❌ *158.96x slower*)   | `163.66 ns` (❌ *1.79x slower*)   | `91.53 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.63 ns` (✅ **1.00x**) | `10.43 ns` (❌ *1.21x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `156.16 ns` (✅ **1.00x**) | `218.39 ns` (❌ *1.40x slower*)   | `31.30 ns` (🚀 **4.99x faster**)    | `58.88 ns` (🚀 **2.65x faster**)    | `109.90 ns` (✅ **1.42x faster**)    | `706.74 ns` (❌ *4.53x slower*)    |
| **`serialize_uncompressed`**             | `211.81 ns` (✅ **1.00x**) | `332.28 ns` (❌ *1.57x slower*)   | `32.29 ns` (🚀 **6.56x faster**)    | `56.58 ns` (🚀 **3.74x faster**)    | `109.89 ns` (🚀 **1.93x faster**)    | `707.53 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `311.44 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.42x slower*)     | `52.41 ns` (🚀 **5942.24x faster**) | `92.72 ns` (🚀 **3358.93x faster**) | `209.58 ns` (🚀 **1485.98x faster**) | `1.26 us` (🚀 **246.96x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.29 us` (✅ **1.00x**)  | `183.90 us` (❌ *2.69x slower*)   | `52.06 ns` (🚀 **1311.67x faster**) | `93.20 ns` (🚀 **732.71x faster**)  | `210.46 ns` (🚀 **324.48x faster**)  | `1.26 us` (🚀 **54.08x faster**)   |
| **`deserialize_uncompressed`**           | `243.35 us` (✅ **1.00x**) | `878.58 us` (❌ *3.61x slower*)   | `52.34 ns` (🚀 **4649.40x faster**) | `92.82 ns` (🚀 **2621.75x faster**) | `209.48 ns` (🚀 **1161.69x faster**) | `1.26 us` (🚀 **192.72x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.82 ns` (✅ **1.00x**) | `477.39 ns` (❌ *2.14x slower*)   | `52.39 ns` (🚀 **4.25x faster**)    | `93.35 ns` (🚀 **2.39x faster**)    | `210.23 ns` (✅ **1.06x faster**)    | `1.27 us` (❌ *5.71x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.35 s` (✅ **1.00x**)  | `8.40 s` (❌ *3.58x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.13 us` (✅ **1.00x**) | `67.77 us` (❌ *2.18x slower*)   | `182.98 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.75 us` (❌ *2.90x slower*)   | `31.61 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.43 ns` (✅ **1.00x**) | `83.85 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.46 ns` (✅ **1.00x**) | `83.70 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.02x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.87 ns` (✅ **1.00x**) | `74.91 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `23.45 ns` (✅ **1.00x**) | `47.01 ns` (❌ *2.01x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.14 ns` (✅ **1.00x**)                 | `256.54 us` (❌ *28056.93x slower*)      | `673.24 us` (❌ *73630.43x slower*)   | `1.27 ms` (❌ *138885.08x slower*)             | `2.22 ms` (❌ *243027.05x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

