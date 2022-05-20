# Signingtool
This tool helps to sign the image using private key and verify using the public key from the given [ecc256.der](https://github.com/yashwanthsinghm/Signingtool/blob/main/ecc256.der).


```
PS D:\Bosch\git_signingtool\Signingtool> cargo run --example test .\stm32f411_bootfw.bin
   Compiling rbsigner v0.1.0 (D:\Bosch\git_signingtool\Signingtool)
    Finished dev [unoptimized + debuginfo] target(s) in 1.74s       
     Running `target\debug\examples\test.exe .\stm32f411_bootfw.bin`
Binary hash1: [167, 142, 69, 57, 38, 138, 112, 67, 201, 232, 9, 106, 19, 129, 146, 61, 184, 135, 37, 157, 203, 47, 125, 205, 90, 250, 130, 112, 99, 
48, 60, 217] 

public key and private key : [116, 191, 93, 233, 248, 105, 105, 68, 53, 174, 183, 57, 111, 161, 64, 17, 182, 161, 127, 45, 138, 134, 185, 88, 188, 74, 81, 247, 243, 15, 35, 119, 120, 14, 17, 70, 149, 58, 29, 223, 105, 205, 52, 35, 254, 99, 5, 21, 48, 67, 187, 158, 117, 99, 224, 65, 106, 112, 206, 22, 10, 96, 42, 56, 83, 206, 126, 93, 64, 168, 190, 202, 227, 223, 127, 159, 179, 7, 26, 147, 249, 82, 71, 48, 204, 48, 230, 7, 28, 231, 252, 144, 125, 94, 88, 160] 

signature: ecdsa::Signature<NistP256>([189, 242, 103, 160, 160, 43, 254, 104, 167, 217, 16, 76, 181, 46, 104, 193, 51, 29, 121, 201, 254, 123, 235, 
118, 201, 209, 248, 157, 62, 215, 55, 92, 77, 189, 31, 197, 204, 104, 248, 215, 21, 225, 6, 80, 244, 222, 230, 157, 241, 251, 36, 190, 155, 65, 102, 159, 110, 226, 60, 204, 105, 101, 201, 156]) 

signature : BDF267A0A02BFE68A7D9104CB52E68C1331D79C9FE7BEB76C9D1F89D3ED7375C4DBD1FC5CC68F8D715E10650F4DEE69DF1FB24BE9B41669F6EE23CCC6965C99C        

verification result :true
rustBoot header RBHeader { buffer: [52, 55, 53, 54, 28, 7, 0, 0, 1, 0, 4, 0, d3, 4, 0, 0, ff, ff, ff, ff, 2, 0, 8, 0, de, 2b, 16, 62, 0, 0, 0, 0, 4, 0, 2, 0, 1, 2, ff, ff, ff, ff, ff, ff, 3, 0, 20, 0, a7, 8e, 45, 39, 26, 8a, 70, 43, c9, e8, 9, 6a, 13, 81, 92, 3d, b8, 87, 25, 9d, cb, 2f, 7d, cd, 
5a, fa, 82, 70, 63, 30, 3c, d9, 10, 0, 20, 0, 6, 6e, 19, 55, 17, e5, c0, 3, 82, a2, aa, c8, 7d, 8, 9a, 40, ec, 1f, 1a, 47, a2, d7, a5, c9, 82, c9, 63, 92, af, 24, 9d, 2b, 20, 0, 40, 0, bd, f2, 67, a0, a0, 2b, fe, 68, a7, d9, 10, 4c, b5, 2e, 68, c1, 33, 1d, 79, c9, fe, 7b, eb, 76, c9, d1, f8, 9d, 3e, d7, 37, 5c, 4d, bd, 1f, c5, cc, 68, f8, d7, 15, e1, 6, 50, f4, de, e6, 9d, f1, fb, 24, be, 9b, 41, 66, 9f, 6e, e2, 3c, cc, 69, 65, c9, 9c, ff, 
ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, 
ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, 0, 0] }```
