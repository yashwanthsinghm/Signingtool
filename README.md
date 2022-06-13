# Signingtool
This tool helps to sign the image using private key and verify using the public key from the given [ecc256.der](https://github.com/yashwanthsinghm/Signingtool/blob/main/ecc256.der).


Steps to be followed to generate Boot and Update Firmware


1. Signing Update firmware

command : cargo run --example SignUpdateImage stm32f411_updtfw.bin

```
yashwanthsingh@Yashwanths-MBP Signingtool % cargo run --example SignUpdateImage stm32f411_updtfw.bin

warning: `rbsigner` (lib) generated 1 warning
   Compiling rbsigner v0.1.0 (/Users/yashwanthsingh/Yash/Projects/git_signingtool/Signingtool)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/examples/SignUpdateImage stm32f411_updtfw.bin`

tsv : [96, b0, 8f, 62, 0, 0, 0, 0]
len :728 j : 728
Binary hash1: [61, 127, 108, 132, 184, 33, 157, 243, 207, 85, 167, 77, 167, 157, 85, 12, 134, 122, 42, 14, 229, 93, 229, 161, 124, 254, 250, 59, 231, 27, 206, 184] 

public key and private key : [116, 191, 93, 233, 248, 105, 105, 68, 53, 174, 183, 57, 111, 161, 64, 17, 182, 161, 127, 45, 138, 134, 185, 88, 188, 74, 81, 247, 243, 15, 35, 119, 120, 14, 17, 70, 149, 58, 29, 223, 105, 205, 52, 35, 254, 99, 5, 21, 48, 67, 187, 158, 117, 99, 224, 65, 106, 112, 206, 22, 10, 96, 42, 56, 83, 206, 126, 93, 64, 168, 190, 202, 227, 223, 127, 159, 179, 7, 26, 147, 249, 82, 71, 48, 204, 48, 230, 7, 28, 231, 252, 144, 125, 94, 88, 160] 

signature: ecdsa::Signature<NistP256>([78, 204, 88, 0, 56, 47, 140, 237, 135, 104, 135, 102, 146, 169, 62, 14, 130, 65, 252, 195, 120, 2, 154, 74, 11, 228, 2, 76, 86, 177, 233, 96, 191, 102, 156, 21, 31, 173, 250, 0, 251, 127, 188, 207, 6, 121, 45, 65, 224, 187, 118, 19, 153, 105, 166, 59, 14, 86, 173, 118, 101, 165, 177, 182]) 

signature_fit : ecdsa::Signature<NistP256>([78, 204, 88, 0, 56, 47, 140, 237, 135, 104, 135, 102, 146, 169, 62, 14, 130, 65, 252, 195, 120, 2, 154, 74, 11, 228, 2, 76, 86, 177, 233, 96, 191, 102, 156, 21, 31, 173, 250, 0, 251, 127, 188, 207, 6, 121, 45, 65, 224, 187, 118, 19, 153, 105, 166, 59, 14, 86, 173, 118, 101, 165, 177, 182]) 

signature value [78, 204, 88, 0, 56, 47, 140, 237, 135, 104, 135, 102, 146, 169, 62, 14, 130, 65, 252, 195, 120, 2, 154, 74, 11, 228, 2, 76, 86, 177, 233, 96, 191, 102, 156, 21, 31, 173, 250, 0, 251, 127, 188, 207, 6, 121, 45, 65, 224, 187, 118, 19, 153, 105, 166, 59, 14, 86, 173, 118, 101, 165, 177, 182]
verify_key VerifyingKey { inner: PublicKey { point: AffinePoint { x: FieldElement([3771849175751103923, 6781121818163984926, 10360362025933622409, 7698073866996824426]), y: FieldElement([7664077598525416856, 15725351593430890645, 15020940090350625273, 7256012533965174784]), infinity: 0 } } }
verification result :false
vk : VerifyingKey { inner: PublicKey { point: AffinePoint { x: FieldElement([3771849175751103923, 6781121818163984926, 10360362025933622409, 7698073866996824426]), y: FieldElement([7664077598525416856, 15725351593430890645, 15020940090350625273, 7256012533965174784]), infinity: 0 } } }
into verify function
signature extrated Ok(ecdsa::Signature<NistP256>([78, 204, 88, 0, 56, 47, 140, 237, 135, 104, 135, 102, 146, 169, 62, 14, 130, 65, 252, 195, 120, 2, 154, 74, 11, 228, 2, 76, 86, 177, 233, 96, 191, 102, 156, 21, 31, 173, 250, 0, 251, 127, 188, 207, 6, 121, 45, 65, 224, 187, 118, 19, 153, 105, 166, 59, 14, 86, 173, 118, 101, 165, 177, 182]))
end of verify
 res from ecc256 true
auth_check Ok(true)
rustBoot header RBHeader { buffer: [52, 55, 53, 54, 28, 7, 0, 0, 1, 0, 4, 0, d3, 4, 0, 0, ff, ff, ff, ff, 2, 0, 8, 0, 96, b0, 8f, 62, 0, 0, 0, 0, 4, 0, 2, 0, 1, 2, ff, ff, ff, ff, ff, ff, 3, 0, 20, 0, 3d, 7f, 6c, 84, b8, 21, 9d, f3, cf, 55, a7, 4d, a7, 9d, 55, c, 86, 7a, 2a, e, e5, 5d, e5, a1, 7c, fe, fa, 3b, e7, 1b, ce, b8, 10, 0, 20, 0, 6, 6e, 19, 55, 17, e5, c0, 3, 82, a2, aa, c8, 7d, 8, 9a, 40, ec, 1f, 1a, 47, a2, d7, a5, c9, 82, c9, 63, 92, af, 24, 9d, 2b, 20, 0, 40, 0, 4e, cc, 58, 0, 38, 2f, 8c, ed, 87, 68, 87, 66, 92, a9, 3e, e, 82, 41, fc, c3, 78, 2, 9a, 4a, b, e4, 2, 4c, 56, b1, e9, 60, bf, 66, 9c, 15, 1f, ad, fa, 0, fb, 7f, bc, cf, 6, 79, 2d, 41, e0, bb, 76, 13, 99, 69, a6, 3b, e, 56, ad, 76, 65, a5, b1, b6, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, 0, 0] }

```







2. Signing  the Boot firmware. 

Command : cargo run --example SignBootImage stm32f411_bootfw.bin
```
yashwanthsingh@Yashwanths-MBP Signingtool % cargo run --example SignBootImage stm32f411_bootfw.bin


warning: `rbsigner` (lib) generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/SignBootImage stm32f411_bootfw.bin`

tsv : [96, b0, 8f, 62, 0, 0, 0, 0]
len :6d0 j : 6d0
Binary hash1: [102, 247, 45, 253, 148, 8, 12, 186, 117, 143, 236, 105, 46, 63, 0, 113, 76, 55, 203, 190, 171, 48, 232, 192, 64, 124, 39, 44, 78, 214, 128, 204] 

public key and private key : [116, 191, 93, 233, 248, 105, 105, 68, 53, 174, 183, 57, 111, 161, 64, 17, 182, 161, 127, 45, 138, 134, 185, 88, 188, 74, 81, 247, 243, 15, 35, 119, 120, 14, 17, 70, 149, 58, 29, 223, 105, 205, 52, 35, 254, 99, 5, 21, 48, 67, 187, 158, 117, 99, 224, 65, 106, 112, 206, 22, 10, 96, 42, 56, 83, 206, 126, 93, 64, 168, 190, 202, 227, 223, 127, 159, 179, 7, 26, 147, 249, 82, 71, 48, 204, 48, 230, 7, 28, 231, 252, 144, 125, 94, 88, 160] 

signature: ecdsa::Signature<NistP256>([20, 64, 154, 63, 194, 166, 46, 107, 110, 143, 188, 99, 241, 92, 194, 110, 49, 43, 107, 84, 176, 208, 166, 38, 140, 16, 142, 101, 193, 192, 148, 7, 177, 125, 197, 91, 172, 94, 187, 218, 42, 143, 91, 207, 222, 24, 120, 177, 50, 171, 25, 129, 252, 36, 158, 67, 24, 141, 188, 177, 131, 154, 173, 90]) 

signature_fit : ecdsa::Signature<NistP256>([20, 64, 154, 63, 194, 166, 46, 107, 110, 143, 188, 99, 241, 92, 194, 110, 49, 43, 107, 84, 176, 208, 166, 38, 140, 16, 142, 101, 193, 192, 148, 7, 177, 125, 197, 91, 172, 94, 187, 218, 42, 143, 91, 207, 222, 24, 120, 177, 50, 171, 25, 129, 252, 36, 158, 67, 24, 141, 188, 177, 131, 154, 173, 90]) 

signature value [20, 64, 154, 63, 194, 166, 46, 107, 110, 143, 188, 99, 241, 92, 194, 110, 49, 43, 107, 84, 176, 208, 166, 38, 140, 16, 142, 101, 193, 192, 148, 7, 177, 125, 197, 91, 172, 94, 187, 218, 42, 143, 91, 207, 222, 24, 120, 177, 50, 171, 25, 129, 252, 36, 158, 67, 24, 141, 188, 177, 131, 154, 173, 90]
verify_key VerifyingKey { inner: PublicKey { point: AffinePoint { x: FieldElement([3771849175751103923, 6781121818163984926, 10360362025933622409, 7698073866996824426]), y: FieldElement([7664077598525416856, 15725351593430890645, 15020940090350625273, 7256012533965174784]), infinity: 0 } } }
verification result :false
vk : VerifyingKey { inner: PublicKey { point: AffinePoint { x: FieldElement([3771849175751103923, 6781121818163984926, 10360362025933622409, 7698073866996824426]), y: FieldElement([7664077598525416856, 15725351593430890645, 15020940090350625273, 7256012533965174784]), infinity: 0 } } }
into verify function
signature extrated Ok(ecdsa::Signature<NistP256>([20, 64, 154, 63, 194, 166, 46, 107, 110, 143, 188, 99, 241, 92, 194, 110, 49, 43, 107, 84, 176, 208, 166, 38, 140, 16, 142, 101, 193, 192, 148, 7, 177, 125, 197, 91, 172, 94, 187, 218, 42, 143, 91, 207, 222, 24, 120, 177, 50, 171, 25, 129, 252, 36, 158, 67, 24, 141, 188, 177, 131, 154, 173, 90]))
end of verify
 res from ecc256 true
auth_check Ok(true)
rustBoot header RBHeader { buffer: [52, 55, 53, 54, d0, 6, 0, 0, 1, 0, 4, 0, d2, 4, 0, 0, ff, ff, ff, ff, 2, 0, 8, 0, 96, b0, 8f, 62, 0, 0, 0, 0, 4, 0, 2, 0, 1, 2, ff, ff, ff, ff, ff, ff, 3, 0, 20, 0, 66, f7, 2d, fd, 94, 8, c, ba, 75, 8f, ec, 69, 2e, 3f, 0, 71, 4c, 37, cb, be, ab, 30, e8, c0, 40, 7c, 27, 2c, 4e, d6, 80, cc, 10, 0, 20, 0, 6, 6e, 19, 55, 17, e5, c0, 3, 82, a2, aa, c8, 7d, 8, 9a, 40, ec, 1f, 1a, 47, a2, d7, a5, c9, 82, c9, 63, 92, af, 24, 9d, 2b, 20, 0, 40, 0, 14, 40, 9a, 3f, c2, a6, 2e, 6b, 6e, 8f, bc, 63, f1, 5c, c2, 6e, 31, 2b, 6b, 54, b0, d0, a6, 26, 8c, 10, 8e, 65, c1, c0, 94, 7, b1, 7d, c5, 5b, ac, 5e, bb, da, 2a, 8f, 5b, cf, de, 18, 78, b1, 32, ab, 19, 81, fc, 24, 9e, 43, 18, 8d, bc, b1, 83, 9a, ad, 5a, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, 0, 0] }

```



3. Here STM32F411 board is used for testting.Place the Boot Firmware at 0x8020000 and place the Update Firmware 0X8040000 location of flash respectively using [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html).

4. Running the Bootloader from [rustBoot](https://github.com/yashwanthsinghm/rustBoot) project.Follow [rustBoot](https://github.com/yashwanthsinghm/rustBoot) to complie and download the binary into 0x8000000 location of flash.
5. Testing : 1) Green led should blink first which is from [BootFirmware](https://github.com/yashwanthsinghm/rustBoot/tree/main/boards/firmware/stm32f411/boot_fw_blinky_green)
             2) Red led should blink after green which is from [UpdateFirmware](https://github.com/yashwanthsinghm/rustBoot/tree/main/boards/firmware/stm32f411/updt_fw_blinky_red)

Result: 

https://user-images.githubusercontent.com/97118799/173413785-968641c1-f436-4c5c-a143-956d7b7be12f.mp4



