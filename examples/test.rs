#![allow(warnings)]
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    ops::Add,
};

use p256::ecdsa::signature::DigestVerifier;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use p256::ecdsa::{signature::Verifier, VerifyingKey};
use p256::SecretKey;
use p256::{
    elliptic_curve::{consts::U32, generic_array::GenericArray, FieldSize},
    EncodedPoint, NistP256,
};
use rand::rngs::OsRng;
use rbsigner::{
    fitsigner::{import_signing_key, sign_fit, CurveType},
    signer::RBHeader,
};

fn main() {
    let mut buf = Vec::new();
    let mut key = Vec::new();
    let mut buf1 = [0u8; 5020]; // 4kb array
    let mut i = 0;
    let mut j = 0;
    let mut file = fs::File::open(
        std::env::args()
            .nth(1)
            .expect("Need path to bin file as argument"),
    )
    .unwrap();
    file.read_to_end(&mut buf).unwrap();

    let mut rb_header = RBHeader::<[u8; 256]>::new_checked([0; 256]).unwrap();

    let tsv: [u8; 8] = [0xDE, 0x2B, 0x16, 0x62, 0x00, 0x00, 0x00, 0x00];
    let sha256_digest: [u8; 0x20] = [
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
        0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
        0x33, 0x33,
    ];

    let pubkey = [
        0x74, 0xBF, 0x5D, 0xE9, 0xF8, 0x69, 0x69, 0x44, 0x35, 0xAE, 0xB7, 0x39, 0x6F, 0xA1, 0x40,
        0x11, 0xB6, 0xA1, 0x7F, 0x2D, 0x8A, 0x86, 0xB9, 0x58, 0xBC, 0x4A, 0x51, 0xF7, 0xF3, 0x0F,
        0x23, 0x77, 0x78, 0x0E, 0x11, 0x46, 0x95, 0x3A, 0x1D, 0xDF, 0x69, 0xCD, 0x34, 0x23, 0xFE,
        0x63, 0x05, 0x15, 0x30, 0x43, 0xBB, 0x9E, 0x75, 0x63, 0xE0, 0x41, 0x6A, 0x70, 0xCE, 0x16,
        0x0A, 0x60, 0x2A, 0x38,
    ];

    let mut signature_value: [u8; 0x40] = [
        0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
        0x44, 0x44, 0x44, 0x44,
    ];
    //let tv: [u8; 4] = [0xD2, 0x04, 0x00, 0x00];
    let tv: [u8; 4] = [0xD3, 0x04, 0x00, 0x00];
    let img: [u8; 2] = [0x01, 0x02];

    rb_header.set_magic_key(0x52555354);
    //rb_header.set_magic_key_size(0xD0060000);
    rb_header.set_magic_key_size(0x28070000);
    rb_header.set_version_type_len(0x01000400);
    rb_header.set_version_value(&tv);
    rb_header.set_timestamp_type_len(0x02000800);
    rb_header.set_timestamp_value(&tsv);
    rb_header.set_image_type_len(0x04000200);
    rb_header.set_image_value(&img);
    rb_header.set_digest_type_len(0x03002000);

    let mut buf2: [u8; 44] = [0; 44]; /* holds the header for hashing */
    let mut buf3: [u8; 4096] = [0; 4096]; /* holds the firmware to for hashing */
    for x in rb_header.inner_ref().as_ref()[..44].iter() {
        buf2[j] = *x;
        j = j + 1;
       // println!("{}={},", j, x);
    }

    let data = &buf2;

    j = 0;
    for x in buf {
        buf3[j] = x;
        j = j + 1;
        // println!("{}={},",j,x);
    }
    let data1 = &buf3[0..j];
   // println!("{:?}", &buf3[0..j]);

    /* Hashing start */
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update(data1);

    // Note that calling `finalize()` consumes hasher
    let digest1 = hasher.finalize();
    println!("Binary hash1: {:?} \n", digest1);
    /* hashing the public key */
    let mut hasher1 = Sha256::new();
    hasher1.update(&pubkey);
    let pubkey_digest = hasher1.finalize();

    rb_header.set_sha256_digest_value(&digest1);
    rb_header.set_pubkey_type_len(0x10002000);
    rb_header.set_pubkey_digest_value(&pubkey_digest);
    rb_header.set_signature_type_len(0x20004000);

    /*opening the ecc256.der file to copy the private key */
    let mut file2 = File::open("ecc256.der").expect("couldnt read file2");
    file2.read_to_end(&mut key).unwrap();

    println!("public key and private key : {:?} \n", key);
    let private_key = key.as_slice();
    let sk = import_signing_key(CurveType::NistP256, &private_key[0x40..]).unwrap();
    let signature_fit = sign_fit(CurveType::NistP256, &digest1, sk).unwrap();
    println!("signature : {} \n", signature_fit);

    // let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    // let secret_key = SecretKey::random(&mut OsRng);
    // //let message = b"message to be signed";
    // let signature = signing_key.sign(&digest1);
    // // println!("{}",signature);
    // let signature_val = signature.as_ref();

    let signature_val = signature_fit.as_ref();
    rb_header.set_signatue_value(&signature_val);
    rb_header.set_end_of_header(0x0000);

    for x in rb_header.inner_ref().as_ref()[..256].iter() {
        buf1[i] = *x;
        i = i + 1;
    }
    for x in buf3 {
        buf1[i] = x;
        i = i + 1;
    }

    let untagged_bytes: &GenericArray<u8, <FieldSize<NistP256> as Add>::Output> =
        GenericArray::from_slice(&pubkey[..]);
    let sec1_encoded_pubkey = EncodedPoint::from_untagged_bytes(untagged_bytes);
    let p256_vk = VerifyingKey::from_encoded_point(&sec1_encoded_pubkey);
    //let mut file3 = File::create("stm32f411_updt fw_v1235_signed.bin").expect("couldnt create file3");
    // file3.write_all(&buf1).expect("couldnt write in file3");

    let verify_key = p256_vk.unwrap();
    let res = verify_key.verify(&digest1, &signature_fit).is_ok();
    //verify_key.verify_digest(digest1, &signature_fit);
    println!("verification result :{}", res);
    println!("rustBoot header {:x?}", rb_header);
}
