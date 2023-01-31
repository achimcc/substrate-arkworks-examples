# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample](#sample)
    - [arithmetic](#arithmetic)
    - [serialization](#serialization)
    - [msm](#msm)
    - [squareroot](#squareroot)
    - [bitwise](#bitwise)
    - [conversions](#conversions)

## Benchmark Results

### sample

|        | `bls12_381::g1projective elements`          | `bls12_381::g2projective elements`           |
|:-------|:--------------------------------------------|:-------------------------------------------- |
|        | `206.63 us` (✅ **1.00x**)                   | `1.85 ms` (❌ *8.98x slower*)                 |

### arithmetic

|                                       | `for bls12_381::fr::bigint`          | `for bls12_381::fq::bigint`          | `for bls12_381::g1projective`          | `for bls12_381::g2projective`          | `for bls12_381::fq2`             | `for bls12_381::fq12`             | `for bls12_381::fq`               | `for bls12_381::fr`               |
|:--------------------------------------|:-------------------------------------|:-------------------------------------|:---------------------------------------|:---------------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                                | `N/A`                                | `1.25 us` (✅ **1.00x**)                | `4.16 us` (❌ *3.34x slower*)           | `23.08 ns` (🚀 **54.04x faster**) | `204.23 ns` (🚀 **6.11x faster**)  | `12.44 ns` (🚀 **100.26x faster**) | `8.83 ns` (🚀 **141.32x faster**)  |
| **`subtraction`**                     | `N/A`                                | `N/A`                                | `1.29 us` (✅ **1.00x**)                | `4.23 us` (❌ *3.28x slower*)           | `23.40 ns` (🚀 **55.05x faster**) | `184.74 ns` (🚀 **6.97x faster**)  | `13.40 ns` (🚀 **96.11x faster**)  | `9.58 ns` (🚀 **134.43x faster**)  |
| **`mixed addition`**                  | `N/A`                                | `N/A`                                | `888.58 ns` (✅ **1.00x**)              | `2.97 us` (❌ *3.34x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed subtraction`**               | `N/A`                                | `N/A`                                | `928.94 ns` (✅ **1.00x**)              | `3.01 us` (❌ *3.24x slower*)           | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                                | `N/A`                                | `589.64 ns` (✅ **1.00x**)              | `1.86 us` (❌ *3.15x slower*)           | `12.60 ns` (🚀 **46.81x faster**) | `153.17 ns` (🚀 **3.85x faster**)  | `7.40 ns` (🚀 **79.64x faster**)   | `5.87 ns` (🚀 **100.37x faster**)  |
| **`scalar multiplication`**           | `N/A`                                | `N/A`                                | `329.20 us` (✅ **1.00x**)              | `999.81 us` (❌ *3.04x slower*)         | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `22.74 ns` (❌ *3.62x slower*)    | `120.44 ns` (❌ *19.18x slower*)   | `18.05 ns` (❌ *2.87x slower*)     | `6.28 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `255.24 ns` (❌ *5.50x slower*)   | `6.52 us` (❌ *140.56x slower*)    | `77.81 ns` (❌ *1.68x slower*)     | `46.37 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `173.75 ns` (❌ *4.61x slower*)   | `4.58 us` (❌ *121.27x slower*)    | `66.72 ns` (❌ *1.77x slower*)     | `37.73 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `15.29 us` (❌ *2.05x slower*)    | `25.89 us` (❌ *3.47x slower*)     | `14.95 us` (❌ *2.00x slower*)     | `7.46 us` (✅ **1.00x**)           |
| **`sum of products of size 2`**       | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `551.61 ns` (❌ *6.38x slower*)   | `13.28 us` (❌ *153.64x slower*)   | `126.25 ns` (❌ *1.46x slower*)    | `86.44 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `542.07 ns` (❌ *6.15x slower*)   | `13.22 us` (❌ *149.94x slower*)   | `163.55 ns` (❌ *1.86x slower*)    | `88.15 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `7.64 ns` (✅ **1.00x**)              | `8.68 ns` (❌ *1.14x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction with borrow`**         | `8.71 ns` (✅ **1.00x**)              | `9.83 ns` (❌ *1.13x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication by 2`**             | `4.69 ns` (✅ **1.00x**)              | `4.87 ns` (✅ **1.04x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division by 2`**                   | `4.79 ns` (✅ **1.00x**)              | `4.78 ns` (✅ **1.00x faster**)       | `N/A`                                  | `N/A`                                  | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization

|                                          | `for bls12_381::g1projective`          | `for bls12_381::g2projective`          | `for bls12_381::fr`                | `for bls12_381::fq`                 | `for bls12_381::fq2`               | `for bls12_381::fq12`             |
|:-----------------------------------------|:---------------------------------------|:---------------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize compressed`**               | `163.45 ns` (✅ **1.00x**)              | `224.64 ns` (❌ *1.37x slower*)         | `29.52 ns` (🚀 **5.54x faster**)    | `56.31 ns` (🚀 **2.90x faster**)     | `117.72 ns` (✅ **1.39x faster**)   | `696.42 ns` (❌ *4.26x slower*)    |
| **`serialize uncompressed`**             | `213.65 ns` (✅ **1.00x**)              | `322.36 ns` (❌ *1.51x slower*)         | `29.51 ns` (🚀 **7.24x faster**)    | `56.22 ns` (🚀 **3.80x faster**)     | `117.50 ns` (🚀 **1.82x faster**)   | `695.03 ns` (❌ *3.25x slower*)    |
| **`deserialize compressed`**             | `134.21 us` (✅ **1.00x**)              | `268.18 us` (❌ *2.00x slower*)         | `58.04 ns` (🚀 **2312.39x faster**) | `102.15 ns` (🚀 **1313.86x faster**) | `213.52 ns` (🚀 **628.56x faster**) | `1.39 us` (🚀 **96.38x faster**)   |
| **`deserialize compressed unchecked`**   | `39.79 us` (✅ **1.00x**)               | `135.20 us` (❌ *3.40x slower*)         | `58.06 ns` (🚀 **685.30x faster**)  | `102.14 ns` (🚀 **389.57x faster**)  | `216.69 ns` (🚀 **183.62x faster**) | `1.37 us` (🚀 **29.04x faster**)   |
| **`deserialize uncompressed`**           | `94.51 us` (✅ **1.00x**)               | `132.72 us` (❌ *1.40x slower*)         | `58.03 ns` (🚀 **1628.68x faster**) | `102.09 ns` (🚀 **925.70x faster**)  | `213.87 ns` (🚀 **441.89x faster**) | `1.37 us` (🚀 **68.75x faster**)   |
| **`deserialize uncompressed unchecked`** | `193.21 ns` (✅ **1.00x**)              | `431.80 ns` (❌ *2.23x slower*)         | `58.04 ns` (🚀 **3.33x faster**)    | `102.05 ns` (🚀 **1.89x faster**)    | `213.41 ns` (✅ **1.10x slower**)   | `1.37 us` (❌ *7.11x slower*)      |

### msm

|        | `for bls12_381::g1projective`          | `for bls12_381::g2projective`           |
|:-------|:---------------------------------------|:--------------------------------------- |
|        | `2.40 s` (✅ **1.00x**)                 | `7.41 s` (❌ *3.09x slower*)             |

### squareroot

|                          | `for bls12_381::fr`          | `for bls12_381::fq`             | `for bls12_381::fq2`              |
|:-------------------------|:-----------------------------|:--------------------------------|:--------------------------------- |
| **`square root for qr`** | `25.48 us` (✅ **1.00x**)     | `39.32 us` (❌ *1.54x slower*)   | `134.08 us` (❌ *5.26x slower*)    |
| **`legendre for qr`**    | `14.26 us` (✅ **1.00x**)     | `39.24 us` (❌ *2.75x slower*)   | `39.64 us` (❌ *2.78x slower*)     |

### bitwise

|                               | `operations for bls12_381::fr::bigint`          | `operations for bls12_381::fq::bigint`           |
|:------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`number of bits`**          | `4.85 ns` (✅ **1.00x**)                         | `5.03 ns` (✅ **1.04x slower**)                   |
| **`from little-endian bits`** | `131.46 ns` (✅ **1.00x**)                       | `189.42 ns` (❌ *1.44x slower*)                   |
| **`from big-endian bits`**    | `131.43 ns` (✅ **1.00x**)                       | `189.41 ns` (❌ *1.44x slower*)                   |
| **`comparison`**              | `4.91 ns` (✅ **1.00x**)                         | `5.09 ns` (✅ **1.04x slower**)                   |
| **`equality`**                | `5.44 ns` (✅ **1.00x**)                         | `5.72 ns` (✅ **1.05x slower**)                   |
| **`is zero`**                 | `4.90 ns` (✅ **1.00x**)                         | `5.21 ns` (✅ **1.06x slower**)                   |

### conversions

|                   | `for bls12_381::fr`          | `for bls12_381::fq`              |
|:------------------|:-----------------------------|:-------------------------------- |
| **`from bigint`** | `40.26 ns` (✅ **1.00x**)     | `76.54 ns` (❌ *1.90x slower*)    |
| **`into bigint`** | `26.68 ns` (✅ **1.00x**)     | `47.68 ns` (❌ *1.79x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

