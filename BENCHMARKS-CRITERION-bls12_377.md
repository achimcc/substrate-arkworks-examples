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
|        | `212.75 us` (✅ **1.00x**)        | `2.19 ms` (❌ *10.31x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.36 us` (✅ **1.00x**)   | `5.28 us` (❌ *3.88x slower*)   | `31.90 ns` (🚀 **42.66x faster**)  | `211.96 ns` (🚀 **6.42x faster**)  | `23.18 ns` (🚀 **58.69x faster**) | `9.84 ns` (🚀 **138.26x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.40 us` (✅ **1.00x**)   | `5.37 us` (❌ *3.83x slower*)   | `32.49 ns` (🚀 **43.14x faster**)  | `204.02 ns` (🚀 **6.87x faster**)  | `18.53 ns` (🚀 **75.64x faster**) | `10.29 ns` (🚀 **136.27x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `995.91 ns` (✅ **1.00x**) | `3.81 us` (❌ *3.82x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.02 us` (✅ **1.00x**)   | `3.85 us` (❌ *3.76x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `679.69 ns` (✅ **1.00x**) | `2.48 us` (❌ *3.64x slower*)   | `15.36 ns` (🚀 **44.25x faster**)  | `122.47 ns` (🚀 **5.55x faster**)  | `8.88 ns` (🚀 **76.53x faster**)  | `10.83 ns` (🚀 **62.78x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `339.42 us` (✅ **1.00x**) | `1.27 ms` (❌ *3.74x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.45 ns` (❌ *3.85x slower*)     | `126.39 ns` (❌ *17.74x slower*)   | `20.27 ns` (❌ *2.85x slower*)    | `7.12 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `318.92 ns` (❌ *7.17x slower*)    | `7.88 us` (❌ *177.27x slower*)    | `84.65 ns` (❌ *1.90x slower*)    | `44.46 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `297.33 ns` (❌ *7.88x slower*)    | `5.62 us` (❌ *148.74x slower*)    | `70.50 ns` (❌ *1.87x slower*)    | `37.75 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.04 us` (❌ *2.12x slower*)     | `29.80 us` (❌ *3.95x slower*)     | `15.80 us` (❌ *2.09x slower*)    | `7.55 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `676.89 ns` (❌ *10.73x slower*)   | `16.20 us` (❌ *256.78x slower*)   | `134.19 ns` (❌ *2.13x slower*)   | `63.08 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `658.80 ns` (❌ *6.95x slower*)    | `16.09 us` (❌ *169.62x slower*)   | `187.14 ns` (❌ *1.97x slower*)   | `94.84 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.81 ns` (✅ **1.00x**) | `9.27 ns` (❌ *1.19x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.30 ns` (✅ **1.00x**) | `12.51 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.59 ns` (✅ **1.00x**) | `4.83 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.46 ns` (✅ **1.00x**) | `4.46 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `177.00 ns` (✅ **1.00x**) | `255.51 ns` (❌ *1.44x slower*)   | `33.64 ns` (🚀 **5.26x faster**)    | `59.37 ns` (🚀 **2.98x faster**)     | `119.61 ns` (✅ **1.48x faster**)    | `745.94 ns` (❌ *4.21x slower*)    |
| **`serialize_uncompressed`**             | `235.39 ns` (✅ **1.00x**) | `380.64 ns` (❌ *1.62x slower*)   | `33.29 ns` (🚀 **7.07x faster**)    | `59.83 ns` (🚀 **3.93x faster**)     | `120.06 ns` (🚀 **1.96x faster**)    | `746.05 ns` (❌ *3.17x slower*)    |
| **`deserialize_compressed`**             | `335.28 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.45x slower*)     | `54.96 ns` (🚀 **6100.40x faster**) | `112.11 ns` (🚀 **2990.50x faster**) | `251.11 ns` (🚀 **1335.17x faster**) | `1.51 us` (🚀 **222.71x faster**)  |
| **`deserialize_compressed_unchecked`**   | `77.25 us` (✅ **1.00x**)  | `207.10 us` (❌ *2.68x slower*)   | `55.15 ns` (🚀 **1400.84x faster**) | `112.12 ns` (🚀 **689.01x faster**)  | `248.44 ns` (🚀 **310.96x faster**)  | `1.49 us` (🚀 **51.87x faster**)   |
| **`deserialize_uncompressed`**           | `257.86 us` (✅ **1.00x**) | `947.01 us` (❌ *3.67x slower*)   | `55.32 ns` (🚀 **4661.22x faster**) | `111.45 ns` (🚀 **2313.63x faster**) | `248.77 ns` (🚀 **1036.57x faster**) | `1.50 us` (🚀 **172.05x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `267.48 ns` (✅ **1.00x**) | `558.91 ns` (❌ *2.09x slower*)   | `54.90 ns` (🚀 **4.87x faster**)    | `110.79 ns` (🚀 **2.41x faster**)    | `248.91 ns` (✅ **1.07x faster**)    | `1.49 us` (❌ *5.59x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.71 s` (✅ **1.00x**)  | `9.48 s` (❌ *3.50x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `32.71 us` (✅ **1.00x**) | `77.27 us` (❌ *2.36x slower*)   | `205.38 us` (❌ *6.28x slower*)    |
| **`legendre_for_qr`**    | `11.29 us` (✅ **1.00x**) | `34.54 us` (❌ *3.06x slower*)   | `35.58 us` (❌ *3.15x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.75 ns` (✅ **1.00x**)  | `4.96 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `72.72 ns` (✅ **1.00x**) | `127.86 ns` (❌ *1.76x slower*)    |
| **`from_big-endian_bits`**    | `72.76 ns` (✅ **1.00x**) | `128.89 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.83 ns` (✅ **1.00x**)  | `5.15 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `5.31 ns` (✅ **1.00x**)  | `5.55 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `4.64 ns` (✅ **1.00x**)  | `4.78 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.59 ns` (✅ **1.00x**) | `93.68 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `25.91 ns` (✅ **1.00x**) | `49.27 ns` (❌ *1.90x slower*)    |

### pairing_for_bls12_377

|        | `g1_preparation_for_bls12_377`          | `g2_preparation_for_bls12_377`          | `miller_loop_for_bls12_377`          | `final_exponentiation_for_bls12_377`          | `full_pairing_for_bls12_377`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `10.73 ns` (✅ **1.00x**)                | `277.06 us` (❌ *25829.60x slower*)      | `734.67 us` (❌ *68490.74x slower*)   | `1.38 ms` (❌ *128916.34x slower*)             | `2.43 ms` (❌ *226748.90x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

