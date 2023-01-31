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
|        | `620.15 us` (✅ **1.00x**)                   | `5.74 ms` (❌ *9.26x slower*)                 |

### arithmetic

|                                       | `for bls12_381::fr::bigint`          | `for bls12_381::fq::bigint`          | `for bls12_381::g1projective`          | `for bls12_381::g2projective`          | `for bls12_381::fq2`              | `for bls12_381::fq12`             | `for bls12_381::fq`               | `for bls12_381::fr`                |
|:--------------------------------------|:-------------------------------------|:-------------------------------------|:---------------------------------------|:---------------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                                | `N/A`                                | `4.89 us` (✅ **1.00x**)                | `10.82 us` (❌ *2.21x slower*)          | `27.90 ns` (🚀 **175.15x faster**) | `216.91 ns` (🚀 **22.53x faster**) | `20.25 ns` (🚀 **241.29x faster**) | `10.03 ns` (🚀 **487.32x faster**)  |
| **`subtraction`**                     | `N/A`                                | `N/A`                                | `3.90 us` (✅ **1.00x**)                | `10.30 us` (❌ *2.64x slower*)          | `28.98 ns` (🚀 **134.60x faster**) | `215.73 ns` (🚀 **18.08x faster**) | `18.32 ns` (🚀 **212.98x faster**) | `12.11 ns` (🚀 **322.17x faster**)  |
| **`mixed addition`**                  | `N/A`                                | `N/A`                                | `2.68 us` (✅ **1.00x**)                | `8.09 us` (❌ *3.02x slower*)           | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed subtraction`**               | `N/A`                                | `N/A`                                | `4.87 us` (✅ **1.00x**)                | `7.36 us` (❌ *1.51x slower*)           | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                                | `N/A`                                | `1.83 us` (✅ **1.00x**)                | `4.70 us` (❌ *2.56x slower*)           | `13.76 ns` (🚀 **133.19x faster**) | `185.53 ns` (🚀 **9.88x faster**)  | `8.22 ns` (🚀 **222.76x faster**)  | `7.69 ns` (🚀 **238.21x faster**)   |
| **`scalar multiplication`**           | `N/A`                                | `N/A`                                | `2.33 ms` (✅ **1.00x**)                | `4.31 ms` (❌ *1.85x slower*)           | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `24.87 ns` (❌ *3.27x slower*)     | `143.24 ns` (❌ *18.85x slower*)   | `17.60 ns` (❌ *2.32x slower*)     | `7.60 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `242.00 ns` (❌ *5.18x slower*)    | `6.39 us` (❌ *136.78x slower*)    | `72.74 ns` (❌ *1.56x slower*)     | `46.69 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `182.03 ns` (❌ *4.32x slower*)    | `4.53 us` (❌ *107.68x slower*)    | `60.40 ns` (❌ *1.43x slower*)     | `42.12 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `15.58 us` (❌ *1.97x slower*)     | `25.66 us` (❌ *3.24x slower*)     | `15.37 us` (❌ *1.94x slower*)     | `7.91 us` (✅ **1.00x**)            |
| **`sum of products of size 2`**       | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `527.97 ns` (❌ *5.05x slower*)    | `12.97 us` (❌ *124.11x slower*)   | `114.09 ns` (✅ **1.09x slower**)  | `104.51 ns` (✅ **1.00x**)          |
| **`naive sum of products of size 2`** | `N/A`                                | `N/A`                                | `N/A`                                  | `N/A`                                  | `511.98 ns` (❌ *5.09x slower*)    | `12.85 us` (❌ *127.63x slower*)   | `160.33 ns` (❌ *1.59x slower*)    | `100.68 ns` (✅ **1.00x**)          |
| **`addition with carry`**             | `6.97 ns` (✅ **1.00x**)              | `8.58 ns` (❌ *1.23x slower*)         | `N/A`                                  | `N/A`                                  | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction with borrow`**         | `8.34 ns` (✅ **1.00x**)              | `10.46 ns` (❌ *1.25x slower*)        | `N/A`                                  | `N/A`                                  | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication by 2`**             | `4.12 ns` (✅ **1.00x**)              | `4.26 ns` (✅ **1.03x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division by 2`**                   | `3.95 ns` (✅ **1.00x**)              | `3.96 ns` (✅ **1.00x slower**)       | `N/A`                                  | `N/A`                                  | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization

|                                          | `for bls12_381::g1projective`          | `for bls12_381::g2projective`          | `for bls12_381::fr`                | `for bls12_381::fq`                 | `for bls12_381::fq2`                | `for bls12_381::fq12`             |
|:-----------------------------------------|:---------------------------------------|:---------------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize compressed`**               | `416.20 ns` (✅ **1.00x**)              | `706.44 ns` (❌ *1.70x slower*)         | `34.72 ns` (🚀 **11.99x faster**)   | `50.32 ns` (🚀 **8.27x faster**)     | `103.37 ns` (🚀 **4.03x faster**)    | `653.10 ns` (❌ *1.57x slower*)    |
| **`serialize uncompressed`**             | `1.33 us` (✅ **1.00x**)                | `994.38 ns` (✅ **1.34x faster**)       | `39.13 ns` (🚀 **33.96x faster**)   | `50.83 ns` (🚀 **26.14x faster**)    | `103.39 ns` (🚀 **12.85x faster**)   | `665.29 ns` (🚀 **2.00x faster**)  |
| **`deserialize compressed`**             | `504.99 us` (✅ **1.00x**)              | `836.25 us` (❌ *1.66x slower*)         | `55.24 ns` (🚀 **9141.25x faster**) | `108.48 ns` (🚀 **4655.03x faster**) | `237.30 ns` (🚀 **2128.06x faster**) | `1.45 us` (🚀 **348.57x faster**)  |
| **`deserialize compressed unchecked`**   | `163.38 us` (✅ **1.00x**)              | `512.20 us` (❌ *3.14x slower*)         | `58.85 ns` (🚀 **2776.26x faster**) | `107.32 ns` (🚀 **1522.31x faster**) | `226.49 ns` (🚀 **721.35x faster**)  | `1.43 us` (🚀 **114.09x faster**)  |
| **`deserialize uncompressed`**           | `226.01 us` (✅ **1.00x**)              | `477.20 us` (❌ *2.11x slower*)         | `55.43 ns` (🚀 **4077.61x faster**) | `104.92 ns` (🚀 **2154.09x faster**) | `226.27 ns` (🚀 **998.89x faster**)  | `1.42 us` (🚀 **158.71x faster**)  |
| **`deserialize uncompressed unchecked`** | `565.44 ns` (✅ **1.00x**)              | `1.50 us` (❌ *2.66x slower*)           | `64.15 ns` (🚀 **8.81x faster**)    | `106.77 ns` (🚀 **5.30x faster**)    | `242.25 ns` (🚀 **2.33x faster**)    | `1.41 us` (❌ *2.50x slower*)      |

### msm

|        | `for bls12_381::g1projective`          | `for bls12_381::g2projective`           |
|:-------|:---------------------------------------|:--------------------------------------- |
|        | `6.89 s` (✅ **1.00x**)                 | `9.77 s` (❌ *1.42x slower*)             |

### squareroot

|                          | `for bls12_381::fr`          | `for bls12_381::fq`             | `for bls12_381::fq2`              |
|:-------------------------|:-----------------------------|:--------------------------------|:--------------------------------- |
| **`square root for qr`** | `26.72 us` (✅ **1.00x**)     | `37.12 us` (❌ *1.39x slower*)   | `129.67 us` (❌ *4.85x slower*)    |
| **`legendre for qr`**    | `13.09 us` (✅ **1.00x**)     | `37.20 us` (❌ *2.84x slower*)   | `37.85 us` (❌ *2.89x slower*)     |

### bitwise

|                               | `operations for bls12_381::fr::bigint`          | `operations for bls12_381::fq::bigint`           |
|:------------------------------|:------------------------------------------------|:------------------------------------------------ |
| **`number of bits`**          | `4.20 ns` (✅ **1.00x**)                         | `4.38 ns` (✅ **1.04x slower**)                   |
| **`from little-endian bits`** | `159.33 ns` (✅ **1.00x**)                       | `234.77 ns` (❌ *1.47x slower*)                   |
| **`from big-endian bits`**    | `157.69 ns` (✅ **1.00x**)                       | `233.18 ns` (❌ *1.48x slower*)                   |
| **`comparison`**              | `4.15 ns` (✅ **1.00x**)                         | `4.44 ns` (✅ **1.07x slower**)                   |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)                         | `4.92 ns` (✅ **1.05x slower**)                   |
| **`is zero`**                 | `4.09 ns` (✅ **1.00x**)                         | `4.22 ns` (✅ **1.03x slower**)                   |

### conversions

|                   | `for bls12_381::fr`          | `for bls12_381::fq`              |
|:------------------|:-----------------------------|:-------------------------------- |
| **`from bigint`** | `38.23 ns` (✅ **1.00x**)     | `84.67 ns` (❌ *2.21x slower*)    |
| **`into bigint`** | `26.81 ns` (✅ **1.00x**)     | `43.57 ns` (❌ *1.63x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

