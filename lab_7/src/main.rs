use rug::Integer;
use num_primes::Generator;
use ring::rand::{SystemRandom, SecureRandom};
use std::{fs, io};
use std::io::Write;

fn encrypt_plaintext(plaintext: Integer, pk: (Integer, Integer, Integer)) -> (Integer, Integer) {
    let rand = SystemRandom::new();
    let (q, g, h) = pk;
    let mut c1 = Integer::new();
    let mut c2 = Integer::new();
    if plaintext >= 0 && plaintext < q {
        let r = random_integer(&rand, q.clone());
        c1 = g.secure_pow_mod(&r, &q);
        c2 = ((plaintext % q.clone()) * h.secure_pow_mod(&r, &q)) % q.clone();
    }
    (c1, c2)
}

fn decrypt_ciphertext(ciphertext: (Integer, Integer), sk: &Integer, q: &Integer) -> Integer {
    let (c1, c2) = ciphertext;
    let theta = c1.secure_pow_mod(sk, q);
    ((c2 % q.clone()) * theta.invert(q).unwrap()) % q.clone()
}

fn generate_keypair() -> ((String, String, String), String) {
    let rand = SystemRandom::new();
    println!("\nGenerating keypair...");
    let p = Integer::from_str_radix(&Generator::safe_prime(512).to_string(), 10).unwrap();
    let q = (p.clone() - Integer::from(1)) / 2;
    let mut a;
    let g;
    loop {
        a = random_integer(&rand, p.clone());
        let asq = a.clone() * a.clone();
        if (asq - Integer::from(1)) % p.clone() != 0 {
               g = Integer::secure_pow_mod(a, &Integer::from(2), &q);
            break;
        }
    }
    let alpha = random_integer(&rand, q.clone());
    let h = g.clone().secure_pow_mod(&alpha, &q);
    ((base64::encode(q.to_string()), base64::encode(g.to_string()), base64::encode(h.to_string())), base64::encode(alpha.to_string()))
}

fn random_integer(rng: &SystemRandom, range: Integer) -> Integer {
    loop {
        let mut bytes = vec![0; ((range.significant_bits() + 7) / 8) as usize];
        rng.fill(&mut bytes).unwrap();
        let num = Integer::from_digits(&bytes, rug::integer::Order::Lsf);
        if num < range {
            return num;
        }
    }
}

pub fn input(kind: &str) -> String {
    print!("Enter {kind}: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    let input = String::from_utf8(base64::decode(input.trim()).unwrap()).unwrap();
    input
}

// q: NjA1MjU2NTIyNDcyMjQ4NTQ1NDk2MjUxMzk5MzM1NzY0MzUyNzA0NTI5ODM0MDk0MDc4MzMwMzIzOTM4ODA0MjU3MDIxNjI1ODM0NDc1MTE5OTA4MzA3Mzk1MDEzNTM3MjI2MDU4NzIxNjgyMjMwOTY2NDc4NTk5Nzg3ODQxNDYxMDE1MTY1MDExMTk4Nzg0MjkwMDMwNTMz
// g: MzM2NTIyMjE5NDEzNTk0Njg5MTk4MTM5NzM3ODQzNTQ0MDMyOTU1NDgxODA1MTgzMDgxNjEzNzA2Nzc0ODM3OTE4Njc2NDM4OTQxMjI0NjUzNTc3ODkyODY1MDE5NzExODAzNjUxMzc5MzU3MjU3OTMwMzA5MDUyOTEwMDYwOTE0NjA5MDM4MTkzMzU3NzYzMTIwOTcyOTA3
// h: NDYwNjM1MjAyNDczNTEwMzQ2OTUwMTM3OTAyMDk4MTEwNDY2OTgwOTc5NDY1Mzk5MTAyMTExMjA0MDg1MTQ1OTEyNDE3MDczOTU1NjgzOTQzOTIyMjc4NjQ1NzIzODY5Mjc3OTE2NDgyODg1MDU0MDQ3NjgyOTgzNDk2MTM4NjcwODMzOTI2MzMzNTM2MjM0MzY2Nzk4NjQw

// private: NTIwMTgwNjk3OTc5MDUwNDgyMjkzNTI3NTYyODc5NDgzODkxODM0MDQyMzkwMzM4MzUxNzgyNzM4MTEyMzExMjY1ODY5ODU5MDM3MjE0ODM4MjI0MDA3MzEwNzA4NzYyMDk2MjE0NjM0OTM5MjU0MzgzNjI3MzM1OTI5NTY4MzQ1MjY5MDE2NzU0NzQ1MDE0NDA2NTI3MDY=
fn main() {
    let ((q, g, h), alpha) = generate_keypair();
    println!("\nPublic key: (q, g, h)");
    println!("q: {q}");
    println!("g: {g}");
    println!("h: {h}");
    println!("\nSecret key: {alpha}");

    let q = Integer::from_str_radix(&input("q"), 10).unwrap();
    let g = Integer::from_str_radix(&input("g"), 10).unwrap();
    let h = Integer::from_str_radix(&input("h"), 10).unwrap();
    let public_key = (q.clone(), g, h);
    let data = fs::read("data.txt").expect("File should exist and path should be right");
    let input_plaintext = Integer::from_str_radix(&hex::encode(data), 16).unwrap();
    let (c1, c2) = encrypt_plaintext(input_plaintext, public_key.clone());
    println!("\nEncrypted ciphertext: (c1, c2)");
    println!("c1 = {}", base64::encode(c1.to_string()));
    println!("c2 = {}", base64::encode(c2.to_string()));
    fs::write("c1.txt", base64::encode(c1.to_string())).expect("Error while writing to file");
    fs::write("c2.txt", base64::encode(c2.to_string())).expect("Error while writing to file");

    let data = fs::read("c1.txt").expect("File should exist and path should be right");
    let c1 = Integer::from_str_radix(&String::from_utf8(base64::decode(data).unwrap()).unwrap(), 10).unwrap();
    let data = fs::read("c2.txt").expect("File should exist and path should be right");
    let c2 = Integer::from_str_radix(&String::from_utf8(base64::decode(data).unwrap()).unwrap(), 10).unwrap();
    let private_key = Integer::from_str_radix(&input("private key"), 10).unwrap();
    let output_plaintext = decrypt_ciphertext((c1, c2), &private_key, &q);
    let output_plaintext = format!("{:X}", &output_plaintext);
    println!("\nDecrypted plaintext: {}", String::from_utf8(hex::decode(output_plaintext).expect("Cant decode from hex")).expect("Cant convert to string"));
}