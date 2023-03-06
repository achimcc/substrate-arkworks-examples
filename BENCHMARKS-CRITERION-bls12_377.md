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
|        | `269.79 us` (✅ **1.00x**)        | `2.71 ms` (❌ *10.04x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.85 us` (✅ **1.00x**)   | `6.21 us` (❌ *3.36x slower*)   | `34.21 ns` (🚀 **54.04x faster**)  | `230.29 ns` (🚀 **8.03x faster**)  | `24.09 ns` (🚀 **76.75x faster**) | `10.95 ns` (🚀 **168.80x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.92 us` (✅ **1.00x**)   | `6.26 us` (❌ *3.26x slower*)   | `35.47 ns` (🚀 **54.13x faster**)  | `219.25 ns` (🚀 **8.76x faster**)  | `20.67 ns` (🚀 **92.89x faster**) | `11.44 ns` (🚀 **167.87x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.35 us` (✅ **1.00x**)   | `4.39 us` (❌ *3.26x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.36 us` (✅ **1.00x**)   | `4.39 us` (❌ *3.24x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `883.57 ns` (✅ **1.00x**) | `3.03 us` (❌ *3.43x slower*)   | `16.95 ns` (🚀 **52.12x faster**)  | `135.74 ns` (🚀 **6.51x faster**)  | `9.66 ns` (🚀 **91.48x faster**)  | `10.66 ns` (🚀 **82.92x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `451.28 us` (✅ **1.00x**) | `1.49 ms` (❌ *3.31x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.98 ns` (❌ *3.42x slower*)     | `141.96 ns` (❌ *17.35x slower*)   | `22.13 ns` (❌ *2.70x slower*)    | `8.18 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `328.32 ns` (❌ *7.12x slower*)    | `8.59 us` (❌ *186.23x slower*)    | `90.53 ns` (❌ *1.96x slower*)    | `46.12 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `377.24 ns` (❌ *9.41x slower*)    | `6.15 us` (❌ *153.34x slower*)    | `79.26 ns` (❌ *1.98x slower*)    | `40.09 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.86 us` (❌ *2.38x slower*)     | `32.55 us` (❌ *4.59x slower*)     | `16.22 us` (❌ *2.29x slower*)    | `7.09 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `717.02 ns` (❌ *11.13x slower*)   | `17.79 us` (❌ *276.12x slower*)   | `142.47 ns` (❌ *2.21x slower*)   | `64.42 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `701.40 ns` (❌ *7.05x slower*)    | `17.42 us` (❌ *175.00x slower*)   | `221.99 ns` (❌ *2.23x slower*)   | `99.56 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `8.30 ns` (✅ **1.00x**)  | `11.73 ns` (❌ *1.41x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.45 ns` (✅ **1.00x**) | `14.83 ns` (❌ *1.42x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.57 ns` (✅ **1.00x**)  | `4.76 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.29 ns` (✅ **1.00x**)  | `4.25 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `195.40 ns` (✅ **1.00x**) | `271.25 ns` (❌ *1.39x slower*)   | `37.92 ns` (🚀 **5.15x faster**)    | `62.90 ns` (🚀 **3.11x faster**)     | `122.42 ns` (✅ **1.60x faster**)    | `790.49 ns` (❌ *4.05x slower*)    |
| **`serialize_uncompressed`**             | `266.68 ns` (✅ **1.00x**) | `416.92 ns` (❌ *1.56x slower*)   | `38.70 ns` (🚀 **6.89x faster**)    | `62.66 ns` (🚀 **4.26x faster**)     | `122.80 ns` (🚀 **2.17x faster**)    | `780.10 ns` (❌ *2.93x slower*)    |
| **`deserialize_compressed`**             | `428.82 us` (✅ **1.00x**) | `1.39 ms` (❌ *3.24x slower*)     | `61.77 ns` (🚀 **6942.50x faster**) | `130.45 ns` (🚀 **3287.36x faster**) | `302.59 ns` (🚀 **1417.15x faster**) | `1.87 us` (🚀 **229.10x faster**)  |
| **`deserialize_compressed_unchecked`**   | `96.92 us` (✅ **1.00x**)  | `252.71 us` (❌ *2.61x slower*)   | `62.08 ns` (🚀 **1561.21x faster**) | `131.22 ns` (🚀 **738.64x faster**)  | `304.35 ns` (🚀 **318.47x faster**)  | `1.82 us` (🚀 **53.11x faster**)   |
| **`deserialize_uncompressed`**           | `336.33 us` (✅ **1.00x**) | `1.13 ms` (❌ *3.37x slower*)     | `58.58 ns` (🚀 **5741.03x faster**) | `127.01 ns` (🚀 **2647.93x faster**) | `302.48 ns` (🚀 **1111.91x faster**) | `1.81 us` (🚀 **185.80x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `305.63 ns` (✅ **1.00x**) | `650.61 ns` (❌ *2.13x slower*)   | `59.31 ns` (🚀 **5.15x faster**)    | `130.71 ns` (🚀 **2.34x faster**)    | `303.88 ns` (✅ **1.01x faster**)    | `1.84 us` (❌ *6.03x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `3.53 s` (✅ **1.00x**)  | `11.09 s` (❌ *3.15x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `35.62 us` (✅ **1.00x**) | `93.55 us` (❌ *2.63x slower*)   | `254.99 us` (❌ *7.16x slower*)    |
| **`legendre_for_qr`**    | `12.57 us` (✅ **1.00x**) | `46.66 us` (❌ *3.71x slower*)   | `46.70 us` (❌ *3.72x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.51 ns` (✅ **1.00x**)  | `4.83 ns` (✅ **1.07x slower**)    |
| **`from_little-endian_bits`** | `77.18 ns` (✅ **1.00x**) | `132.18 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `74.67 ns` (✅ **1.00x**) | `131.06 ns` (❌ *1.76x slower*)    |
| **`comparison`**              | `4.71 ns` (✅ **1.00x**)  | `5.01 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.95 ns` (✅ **1.00x**)  | `5.82 ns` (❌ *1.18x slower*)      |
| **`is_zero`**                 | `4.38 ns` (✅ **1.00x**)  | `4.77 ns` (✅ **1.09x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.46 ns` (✅ **1.00x**) | `95.34 ns` (❌ *2.10x slower*)    |
| **`into_bigint`** | `27.51 ns` (✅ **1.00x**) | `53.61 ns` (❌ *1.95x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `10.82 ns` (✅ **1.00x**)                | `343.00 us` (❌ *31705.85x slower*)      | `827.54 us` (❌ *76495.61x slower*)   | `1.50 ms` (❌ *138585.54x slower*)             | `2.76 ms` (❌ *254975.85x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

