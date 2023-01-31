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
|        | `222.35 us` (✅ **1.00x**)        | `2.25 ms` (❌ *10.13x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.45 us` (✅ **1.00x**)   | `5.50 us` (❌ *3.79x slower*)   | `30.09 ns` (🚀 **48.27x faster**)  | `248.09 ns` (🚀 **5.86x faster**)  | `22.73 ns` (🚀 **63.89x faster**) | `10.20 ns` (🚀 **142.36x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.49 us` (✅ **1.00x**)   | `5.65 us` (❌ *3.79x slower*)   | `33.07 ns` (🚀 **45.01x faster**)  | `246.49 ns` (🚀 **6.04x faster**)  | `18.37 ns` (🚀 **81.04x faster**) | `12.35 ns` (🚀 **120.57x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `1.05 us` (✅ **1.00x**)   | `4.13 us` (❌ *3.92x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.05 us` (✅ **1.00x**)   | `4.01 us` (❌ *3.80x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `704.00 ns` (✅ **1.00x**) | `2.53 us` (❌ *3.59x slower*)   | `16.94 ns` (🚀 **41.56x faster**)  | `197.19 ns` (🚀 **3.57x faster**)  | `12.87 ns` (🚀 **54.72x faster**) | `6.10 ns` (🚀 **115.42x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `358.99 us` (✅ **1.00x**) | `1.27 ms` (❌ *3.55x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `26.63 ns` (❌ *3.87x slower*)     | `165.53 ns` (❌ *24.05x slower*)   | `22.90 ns` (❌ *3.33x slower*)    | `6.88 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `322.76 ns` (❌ *7.06x slower*)    | `7.86 us` (❌ *172.10x slower*)    | `81.35 ns` (❌ *1.78x slower*)    | `45.69 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `299.43 ns` (❌ *7.93x slower*)    | `5.70 us` (❌ *151.04x slower*)    | `71.68 ns` (❌ *1.90x slower*)    | `37.74 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.44 us` (❌ *2.36x slower*)     | `29.29 us` (❌ *4.21x slower*)     | `15.29 us` (❌ *2.20x slower*)    | `6.96 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `703.30 ns` (❌ *11.49x slower*)   | `17.53 us` (❌ *286.52x slower*)   | `141.00 ns` (❌ *2.30x slower*)   | `61.19 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `678.28 ns` (❌ *7.34x slower*)    | `16.25 us` (❌ *175.97x slower*)   | `183.50 ns` (❌ *1.99x slower*)   | `92.37 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.69 ns` (✅ **1.00x**) | `10.46 ns` (❌ *1.36x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.94 ns` (✅ **1.00x**) | `13.19 ns` (❌ *1.33x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.29 ns` (✅ **1.00x**) | `4.48 ns` (✅ **1.05x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.16 ns` (✅ **1.00x**) | `4.32 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `186.25 ns` (✅ **1.00x**) | `257.09 ns` (❌ *1.38x slower*)   | `33.56 ns` (🚀 **5.55x faster**)    | `57.55 ns` (🚀 **3.24x faster**)     | `114.40 ns` (✅ **1.63x faster**)    | `714.08 ns` (❌ *3.83x slower*)    |
| **`serialize_uncompressed`**             | `234.89 ns` (✅ **1.00x**) | `376.68 ns` (❌ *1.60x slower*)   | `33.75 ns` (🚀 **6.96x faster**)    | `57.93 ns` (🚀 **4.05x faster**)     | `115.82 ns` (🚀 **2.03x faster**)    | `719.16 ns` (❌ *3.06x slower*)    |
| **`deserialize_compressed`**             | `347.60 us` (✅ **1.00x**) | `1.18 ms` (❌ *3.39x slower*)     | `62.54 ns` (🚀 **5557.96x faster**) | `119.45 ns` (🚀 **2910.03x faster**) | `273.81 ns` (🚀 **1269.49x faster**) | `1.59 us` (🚀 **219.18x faster**)  |
| **`deserialize_compressed_unchecked`**   | `83.13 us` (✅ **1.00x**)  | `228.69 us` (❌ *2.75x slower*)   | `62.23 ns` (🚀 **1335.96x faster**) | `118.99 ns` (🚀 **698.64x faster**)  | `277.34 ns` (🚀 **299.75x faster**)  | `1.56 us` (🚀 **53.13x faster**)   |
| **`deserialize_uncompressed`**           | `260.71 us` (✅ **1.00x**) | `1.02 ms` (❌ *3.93x slower*)     | `63.84 ns` (🚀 **4083.68x faster**) | `123.59 ns` (🚀 **2109.45x faster**) | `277.81 ns` (🚀 **938.47x faster**)  | `1.55 us` (🚀 **167.69x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `272.23 ns` (✅ **1.00x**) | `597.79 ns` (❌ *2.20x slower*)   | `62.85 ns` (🚀 **4.33x faster**)    | `119.78 ns` (🚀 **2.27x faster**)    | `275.40 ns` (✅ **1.01x slower**)    | `1.52 us` (❌ *5.59x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.90 s` (✅ **1.00x**)  | `9.88 s` (❌ *3.41x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.95 us` (✅ **1.00x**) | `80.36 us` (❌ *2.51x slower*)   | `210.14 us` (❌ *6.58x slower*)    |
| **`legendre_for_qr`**    | `11.85 us` (✅ **1.00x**) | `39.08 us` (❌ *3.30x slower*)   | `37.71 us` (❌ *3.18x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.38 ns` (✅ **1.00x**)   | `4.62 ns` (✅ **1.06x slower**)    |
| **`from_little-endian_bits`** | `168.42 ns` (✅ **1.00x**) | `268.31 ns` (❌ *1.59x slower*)    |
| **`from_big-endian_bits`**    | `172.60 ns` (✅ **1.00x**) | `280.13 ns` (❌ *1.62x slower*)    |
| **`comparison`**              | `4.46 ns` (✅ **1.00x**)   | `4.69 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.90 ns` (✅ **1.00x**)   | `5.89 ns` (❌ *1.20x slower*)      |
| **`is_zero`**                 | `4.29 ns` (✅ **1.00x**)   | `4.51 ns` (✅ **1.05x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.23 ns` (✅ **1.00x**) | `93.96 ns` (❌ *2.28x slower*)    |
| **`into_bigint`** | `27.84 ns` (✅ **1.00x**) | `49.18 ns` (❌ *1.77x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

