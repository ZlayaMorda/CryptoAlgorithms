Simple realizations of some crypto algorithms in Rust

### lab_1 – GOST 28147-89 in mode of generation message authentication code

to encrypt pass mode, key, path to data, path to encrypted data and path to MAC

`cargo run enc qwertyuiopqwertyuiopqwertyuiopjk data.txt encrypted.txt imito.txt`

decrypt in same way using `dec`

### lab_2 – Standard of the RB 34.101.31-2011 in mode of counter

to encrypt pass mode, key, path to data, path to encrypted data and path to sending

`cargo run dec qwertyuiopqwertyuiopqwertyuiopjk encrypted.txt data.txt sending.txt`

decrypt in same way using `dec`

### lab_3 – Rabin crypto system

to encrypt pass mode, keys p and q (prime==3mod4), path to data, path to encrypted data

`cargo run enc 827 859 data.txt encrypted.txt`

decrypt in same way using `dec`

### lab_4 – McEliece crypto system

To generate random public and secret keys `pk.mce` and `sk.mce`:

`cargo run keygen pk.mce sk.mce`

To generate a random plaintext `m` with the public key `pk.mce`:

`cargo run plaintext pk.mce m`

To encrypt plaintext `m` with public key `pk.mce` and output ciphertext `c`:

`cargo run encrypt pk.mce m c`

To decrypt ciphertext `c` with secret key `sk.mce` and output the resulting text in file `d`:

`cargo run decrypt sk.mce c d`

### lab_5 – GOST 34.11 and SHA-1

to encrypt pass path to data, path to encrypted data

`cargo run data.txt hash.txt`

### lab_6 – digital signature based on GOST 34.10

to sign pass mode, path to data, path to left part of signature, path to right part of signature

`cargo run sign data.txt sign_r.txt sign_s.txt`

### lab_7 – ElGamal elliptic curve

At first generate public (q, g, h) and private(secret) keys, use data.txt file for data, c1.txt and c2.txt files for ciphers
It may take a time to generate keys

`cargo run`

### lab_8 – Steganography method of hiding text message in gray JPG using fast fourier transformation

Use font.ttf for your font for the text, may replace for own

Pass mode and name of image file, it will add .jpg, so prepare your image, convert it to jpg format and make even size, also pass text and font size for store mode

Input image may be colored, but it would be converted to the gray

`cargo run store pepe PEPE 24`
