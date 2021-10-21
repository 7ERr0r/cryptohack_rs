
pub mod my_pretty_der;

use std::io::Cursor;
use rusticata_macros::debug;
use core::mem::swap;
use num_bigint::BigInt;
use std::io::BufReader;
use byteorder::{BigEndian, ReadBytesExt};
use num_bigint::Sign;
use serde::{Deserialize, Serialize};



pub fn a_ascii() {
    let s: &[u8] = &[
        99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98,
        108, 51, 125,
    ];
    println!("{}", String::from_utf8_lossy(s));
}

pub fn b_hex() {
    let my_str = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let my_bytes = hex::decode(my_str).unwrap();
    println!("{}", String::from_utf8_lossy(&my_bytes));
}

pub fn c_base64() {
    let my_str = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let my_bytes = hex::decode(my_str).unwrap();
    let my_b64 = base64::encode(my_bytes);
    println!("{}", my_b64);
}

pub fn d_bytes_big_integers() {
    let x = BigInt::parse_bytes(
        b"11515195063862318899931685488813747395775516287289682636499965282714637259206269",
        10,
    )
    .unwrap();
    let my_bytes = x.to_bytes_be().1;
    println!("{}", String::from_utf8_lossy(&my_bytes));
}



#[derive(Serialize, Deserialize)]
struct EncodedAndType {
    encoded: Option<serde_json::Value>,

    #[serde(rename = "type")]
    typ: Option<String>,

    flag: Option<String>,
}

// Encoding Challenge
pub fn e_tcp() {
    use serde_json::json;
    use std::io::BufRead;
    use std::io::Write;
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("socket.cryptohack.org:13377").unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut json_payload = String::new();
        if reader.read_line(&mut json_payload).unwrap() <= 0 {
            break;
        }
        //println!("json_payload: {}", json_payload);

        let enc_type: EncodedAndType = serde_json::from_str(&json_payload).unwrap();
        if let Some(flag) = enc_type.flag {
            println!("\n\nflag: {}", flag);
            break;
        }

        {
            print!("{}, ", enc_type.typ.as_ref().unwrap());
            std::io::stdout().flush().unwrap();
        }
        let encoded = enc_type.encoded.as_ref().unwrap();
        let encoded_string = encoded.as_str().map(|s| s.into()).unwrap_or(String::new());
        let encoded_array: Vec<u8> = if let Some(arr) = encoded.as_array() {
            arr.into_iter()
                .map(|element| element.as_i64().unwrap() as u8)
                .collect()
        } else {
            Vec::new()
        };
        let result = match enc_type.typ.as_ref().unwrap().as_str() {
            "bigint" => {
                let x =
                    BigInt::parse_bytes(encoded_string.strip_prefix("0x").unwrap().as_bytes(), 16)
                        .unwrap();
                let my_bytes = x.to_bytes_be().1;
                utf8(&my_bytes)
            }
            "hex" => utf8(&hex::decode(encoded_string).unwrap()),
            "utf-8" => utf8(&encoded_array),
            "base64" => utf8(&base64::decode(&encoded_string).unwrap()),
            "rot13" => my_rot13(&encoded_string),
            _other => encoded_string,
        };

        let response = json!({ "decoded": result });
        let response = response.to_string();
        stream.write(response.as_bytes()).unwrap();
    }
}

pub fn my_rot13(input: &str) -> String {
    let shift = 13;
    let mut output = String::new();

    for c in input.chars() {
        let o = if c >= 'a' && c <= 'z' {
            ((c as u8 - 'a' as u8 + shift) % 26 + 'a' as u8) as char
        } else if c >= 'A' && c <= 'Z' {
            ((c as u8 - 'A' as u8 + shift) % 26 + 'A' as u8) as char
        } else {
            c
        };
        output.push(o as char);
    }

    output
}

pub fn utf8(some_bytes: &[u8]) -> String {
    String::from_utf8_lossy(some_bytes).into_owned()
}

pub fn f_xor_starter() {
    let ciphertext = "label";
    let mut plaintext = String::new();

    for c in ciphertext.chars() {
        let cc = (c as u8 ^ 13) as char;
        plaintext.push(cc);
    }

    println!("crypto{{{}}}", plaintext);
}

#[allow(non_snake_case)]
pub fn g_xor_properties() {
    let KEY1 = hex::decode("a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313").unwrap();
    let KEY2_KEY1 = hex::decode("37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e").unwrap();
    let KEY2_KEY3 = hex::decode("c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1").unwrap();
    let FLAG_KEY1_KEY3_KEY2 =
        hex::decode("04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf").unwrap();

    let KEY2 = xor_arr(&KEY2_KEY1, &KEY1);
    let KEY3 = xor_arr(&KEY2_KEY3, &KEY2);

    let FLAG = FLAG_KEY1_KEY3_KEY2;
    let FLAG = xor_arr(&FLAG, &KEY1);
    let FLAG = xor_arr(&FLAG, &KEY2);
    let FLAG = xor_arr(&FLAG, &KEY3);

    println!("flag: {}", String::from_utf8_lossy(&FLAG));
}

pub fn h_xor_favourite_byte() {
    let ciphertext =
        hex::decode("73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d").unwrap();

    for b in 0..255 as u8 {
        let plaintext: Vec<u8> = ciphertext.iter().map(|c| c ^ b).collect();

        let plaintext = String::from_utf8_lossy(&plaintext);
        if plaintext.contains("crypto{") {
            println!("plaintext: {}", plaintext);
            println!("byte: {}", b);
        }
    }
}

pub fn i_xor_either() {
    let ciphertext = hex::decode(
        "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104",
    )
    .unwrap();

    let guesstext = "crypto{".as_bytes();

    let guess_key = xor_arr(&ciphertext, guesstext);
    println!("guess_key: {:?}", guess_key);
    println!("guess_key: {:?}", String::from_utf8_lossy(&guess_key));

    let guessed_then = "myXORkey".as_bytes();

    let plaintext: Vec<u8> = ciphertext
        .iter()
        .enumerate()
        .map(|(i, c)| c ^ guessed_then[i % guessed_then.len()])
        .collect();

    let plaintext = String::from_utf8_lossy(&plaintext);
    println!("plaintext: {}", plaintext);
}

pub fn xor_arr(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x1, &x2)| x1 ^ x2).collect()
}

pub fn j_math_gcd() {
    //let result = my_gcd(15, 26);
    let result = my_gcd(66528, 52920);

    println!("gcd: {}", result);
}

pub fn my_gcd(mut a: i64, mut b: i64) -> i64 {
    if b > a {
        swap(&mut a, &mut b);
    }
    assert!(a > b);
    // a is bigger

    while b > 0 {
        let q = a / b;
        let r = a % b;
        println!("{:5} = {}({:5}) + {:5}", a, q, b, r);
        a = b;
        b = r;
    }

    a
}

pub fn k_math_gcd_ext() {
    let x = 26513;
    let n = 32321;
    let result = my_gcd_ext(x, n);

    println!("gcd: {}", result.gcd);
    println!("p: {}", result.p);
    println!("s: {}", result.s);
    let check = result.p * x + result.s * n;
    println!("check: {}*{} + {}*{} = {}", result.p, x, result.s, n, check);
}

pub struct GcdExtended {
    gcd: i64,
    p: i64,
    s: i64,
}

pub fn my_gcd_ext(x: i64, n: i64) -> GcdExtended {
    let mut a = x;
    let mut b = n;

    let (mut p2, mut p1, mut q2, mut q1) = (0, 0, 0, 0);
    let mut step = 0;
    while b > 0 {
        let q = a / b;
        let r = a % b;

        let pi = if step >= 2 {
            (p2 - p1 * q2 as i64).rem_euclid(n)
        } else {
            step
        };

        if step >= 2 {
            println!(
                "{:5} = {}({:5}) + {:5}      p = {:5} - {:5}({:5}) mod {:5} = {:5}",
                a, q, b, r, p2, p1, q2, n, pi
            );
        } else {
            println!("{:5} = {}({:5}) + {:5}      p = {:5}", a, q, b, r, pi);
        }
        a = b;
        b = r;

        q2 = q1;
        q1 = q;

        p2 = p1;
        p1 = pi;

        step += 1;
    }

    let pk2 = (p2 - p1 * q2 as i64).rem_euclid(n);
    println!(
        "                              p = {:5} - {:5}({:5}) mod {:5} = {:5}",
        p2, p1, q2, n, pk2
    );
    let s = (1 - pk2 * x) / n;

    GcdExtended {
        gcd: a,
        p: pk2,
        s: s,
    }
}

pub fn l_math_modular_arithmetic_1() {
    let x = 11 as u64 % 6;
    let y = 8146798528947 as u64 % 17;

    println!("{}", x.min(y));
}

macro_rules! big {
    ($mynum:expr) => {
        BigInt::parse_bytes(stringify!($mynum).as_bytes(), 10).unwrap();
    };
}

pub fn m_math_modular_arithmetic_2() {
    let p = big!(65537);
    let p_minus_1 = &p - 1;
    let x = big!(27324678765465536);

    let result = x.modpow(&p_minus_1, &p);
    println!("{}^{} mod {} = {}", x, p_minus_1, p, result);
}

pub fn n_math_modular_inverting() {
    let a = big!(3);
    let b = big!(13);

    let d = a.modpow(&(&b - 2), &b);
    println!("{}^{} mod {} = {}", a, &b - 2, b, d);
}

pub fn o_data_formats_pem_privacy_enchanced_mail() {
    let pem_bytes = include_bytes!("assets/privacy_enhanced_mail.pem");

    let der_bytes = my_pem_decode(pem_bytes);
    //println!("{}", String::from_utf8_lossy(&der_bytes));

    let (rem, der_obj) = der_parser::parse_der(&der_bytes).expect("could not parse DER data");
    // RSAPrivateKey ::= SEQUENCE {
    //  0  version           Version,
    //  1  modulus           INTEGER,  -- n
    //  2  publicExponent    INTEGER,  -- e
    //  3  privateExponent   INTEGER,  -- d
    //  4  prime1            INTEGER,  -- p
    //  5  prime2            INTEGER,  -- q
    //  6  exponent1         INTEGER,  -- d mod (p-1)
    //  7  exponent2         INTEGER,  -- d mod (q-1)
    //  8  coefficient       INTEGER,  -- (inverse of q) mod p
    //  9  otherPrimeInfos   OtherPrimeInfos OPTIONAL
    //   }
    //println!("{:?}", der_obj.as_pretty(0, 2));

    if !rem.is_empty() {
        println!("WARNING: extra bytes after BER/DER object:\n{:x?}", rem);
    }

    let der_sequence = der_obj.as_sequence().unwrap();
    let private_exponent = &der_sequence[3]; //  3  privateExponent   INTEGER,  -- d
    let private_exponent = private_exponent.as_biguint().unwrap();
    println!("Extract the private key d: {}", private_exponent);
}

pub fn my_pem_decode(pem_string: &[u8]) -> Vec<u8> {
    use std::io::BufRead;
    let lines = pem_string.lines();

    let mut base64encoded = String::new();
    for line in lines {
        let line = line.unwrap();
        if !line.starts_with("-----") {
            base64encoded.push_str(&line);
        }
    }

    let base64decoded = base64::decode(base64encoded).unwrap();
    base64decoded
}



pub fn p_data_formats_certainly_not() {
    
    let der_bytes = include_bytes!("assets/2048b-rsa-example-cert.der");
    let der_bytes = &der_bytes[..];
    let (rem, der_obj) = der_parser::parse_der(&der_bytes).expect("could not parse DER data");
    println!("{:?}", my_pretty_der::as_pretty(&der_obj, 0, 2));
    if !rem.is_empty() {
        println!("WARNING: extra bytes after BER/DER object:\n{:x?}", rem);
    }

    let der_elem = der_obj.as_sequence().unwrap();
    let der_elem = &der_elem[0].as_sequence().unwrap();
    let der_elem = &der_elem.last().unwrap().as_sequence().unwrap();
    
    let der_elem = &der_elem.last().unwrap();
    

    //println!("{:?}", my_pretty_der::as_pretty(&der_elem, 0, 2));
    
    let der_elem = der_elem.as_bitstring().unwrap();
    let end = der_elem.data.len();

    // nie wiem dlaczego od 8 bajtu i obcinamy 5 ostatnich...
    let modulus_bytes = &der_elem.data[8..end-5];
    println!("     modulus: {:?}", debug::HexSlice(modulus_bytes));
    println!("flag modulus: {:?}", BigInt::from_bytes_be(Sign::Plus, modulus_bytes));

    // Nie udalo sie przeczytac certu poprzez x509_signature
    // let cert = x509_signature::parse_certificate(&der_bytes).unwrap();
    // println!("{:?}", cert);
}




pub fn r_ssh_keys() {
    use std::io::BufRead;
    use std::io::Read;

    let ssh_rsa = include_str!("assets/bruce_rsa.pub");
    let ssh_rsa = ssh_rsa.split(" ").nth(1).unwrap();
    let base64decoded = base64::decode(ssh_rsa).unwrap();


    let mut rdr = Cursor::new(base64decoded);
    let _prefix = rdr.read_u32::<BigEndian>().unwrap();
    rdr.consume(7); // "ssh-rsa"

    let exponent = {
        let length = rdr.read_u32::<BigEndian>().unwrap();
        let mut v = vec![0; length as usize];
        rdr.read(&mut v).unwrap();
        v
    };
    let modulus = {
        let length = rdr.read_u32::<BigEndian>().unwrap();
        let mut v = vec![0; length as usize];
        rdr.read(&mut v).unwrap();
        v
    };

    println!("    exponent: {:?}", BigInt::from_bytes_be(Sign::Plus, &exponent));
    println!("flag modulus: {:?}", BigInt::from_bytes_be(Sign::Plus, &modulus));
    
}




