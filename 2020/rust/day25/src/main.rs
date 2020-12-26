const P: u64 = 20201227;
const G: u64 = 7;
const INPUT: (u64, u64) = (14205034, 18047856);
// const TEST_INPUT: (u64, u64) = (5764801, 17807724);


fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut base = base % modulus;
    let mut exponent = exponent;
    let mut c = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            c = (c * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    c
}

fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

fn crack(pub_a: u64, pub_b: u64) -> u64 {
    let a = crack_one(pub_a);
    secret(P, pub_b, a)
}

fn crack_one(public: u64) -> u64 {
    let mut pub_a = 1;
    for a in 1.. {
        pub_a = (pub_a * G) % P;
        if pub_a == public {
            return a;
        }
    }
    panic!("not found");
}

fn main() {
    println!("Part 1: {:?}", crack(INPUT.0, INPUT.1));
}
