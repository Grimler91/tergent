use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MechanismType {
    RsaPkcsKeyPairGen = 0x0000,
    RsaPkcs = 0x0001,
    Rsa9796 = 0x0002,
    RsaX509 = 0x0003,
    Md2RsaPkcs = 0x0004,
    Md5RsaPkcs = 0x0005,
    Sha1RsaPkcs = 0x0006,
    Ripemd128RsaPkcs = 0x0007,
    Ripemd160RsaPkcs = 0x0008,
    RsaPkcsOaep = 0x0009,
    RsaX931KeyPairGen = 0x000a,
    RsaX931 = 0x000b,
    Sha1RsaX931 = 0x000c,
    RsaPkcsPss = 0x000d,
    Sha1RsaPkcsPss = 0x000e,
    DsaKeyPairGen = 0x0010,
    Dsa = 0x0011,
    DsaSha1 = 0x0012,
    DsaSha224 = 0x0013,
    DsaSha256 = 0x0014,
    DsaSha384 = 0x0015,
    DsaSha512 = 0x0016,
    DhPkcsKeyPairGen = 0x0020,
    DhPkcsDerive = 0x0021,
    X942DhKeyPairGen = 0x0030,
    X942DhDerive = 0x0031,
    X942DhHybridDerive = 0x0032,
    X942MqvDerive = 0x0033,
    Sha256RsaPkcs = 0x0040,
    Sha384RsaPkcs = 0x0041,
    Sha512RsaPkcs = 0x0042,
    Sha256RsaPkcsPss = 0x0043,
    Sha384RsaPkcsPss = 0x0044,
    Sha512RsaPkcsPss = 0x0045,
    Sha224RsaPkcs = 0x0046,
    Sha224RsaPkcsPss = 0x0047,
    Sha512224 = 0x0048,
    Sha512224Hmac = 0x0049,
    Sha512224HmacGeneral = 0x004a,
    Sha512224KeyDerivation = 0x004b,
    Sha512256 = 0x004c,
    Sha512256Hmac = 0x004d,
    Sha512256HmacGeneral = 0x004e,
    Sha512256KeyDerivation = 0x004f,
    Sha512T = 0x0050,
    Sha512THmac = 0x0051,
    Sha512THmacGeneral = 0x0052,
    Sha512TKeyDerivation = 0x0053,
    Rc2KeyGen = 0x0100,
    Rc2Ecb = 0x0101,
    Rc2Cbc = 0x0102,
    Rc2Mac = 0x0103,
    Rc2MacGeneral = 0x0104,
    Rc2CbcPad = 0x0105,
    Rc4KeyGen = 0x0110,
    Rc4 = 0x0111,
    DesKeyGen = 0x0120,
    DesEcb = 0x0121,
    DesCbc = 0x0122,
    DesMac = 0x0123,
    DesMacGeneral = 0x0124,
    DesCbcPad = 0x0125,
    Des2KeyGen = 0x0130,
    Des3KeyGen = 0x0131,
    Des3Ecb = 0x0132,
    Des3Cbc = 0x0133,
    Des3Mac = 0x0134,
    Des3MacGeneral = 0x0135,
    Des3CbcPad = 0x0136,
    Des3CmacGeneral = 0x0137,
    Des3Cmac = 0x0138,
    CdmfKeyGen = 0x0140,
    CdmfEcb = 0x0141,
    CdmfCbc = 0x0142,
    CdmfMac = 0x0143,
    CdmfMacGeneral = 0x0144,
    CdmfCbcPad = 0x0145,
    DesOfb64 = 0x0150,
    DesOfb8 = 0x0151,
    DesCfb64 = 0x0152,
    DesCfb8 = 0x0153,
    Md2 = 0x0200,
    Md2Hmac = 0x0201,
    Md2HmacGeneral = 0x0202,
    Md5 = 0x0210,
    Md5Hmac = 0x0211,
    Md5HmacGeneral = 0x0212,
    Sha1 = 0x0220,
    Sha1Hmac = 0x0221,
    Sha1HmacGeneral = 0x0222,
    Ripemd128 = 0x0230,
    Ripemd128Hmac = 0x0231,
    Ripemd128HmacGeneral = 0x0232,
    Ripemd160 = 0x0240,
    Ripemd160Hmac = 0x0241,
    Ripemd160HmacGeneral = 0x0242,
    Sha256 = 0x0250,
    Sha256Hmac = 0x0251,
    Sha256HmacGeneral = 0x0252,
    Sha224 = 0x0255,
    Sha224Hmac = 0x0256,
    Sha224HmacGeneral = 0x0257,
    Sha384 = 0x0260,
    Sha384Hmac = 0x0261,
    Sha384HmacGeneral = 0x0262,
    Sha512 = 0x0270,
    Sha512Hmac = 0x0271,
    Sha512HmacGeneral = 0x0272,
    SecuridKeyGen = 0x0280,
    Securid = 0x0282,
    HotpKeyGen = 0x0290,
    Hotp = 0x0291,
    Acti = 0x02a0,
    ActiKeyGen = 0x02a1,
    CastKeyGen = 0x0300,
    CastEcb = 0x0301,
    CastCbc = 0x0302,
    CastMac = 0x0303,
    CastMacGeneral = 0x0304,
    CastCbcPad = 0x0305,
    Cast3KeyGen = 0x0310,
    Cast3Ecb = 0x0311,
    Cast3Cbc = 0x0312,
    Cast3Mac = 0x0313,
    Cast3MacGeneral = 0x0314,
    Cast3CbcPad = 0x0315,
    Cast128KeyGen = 0x0320,
    Cast128Ecb = 0x0321,
    Cast128Cbc = 0x0322,
    Cast128Mac = 0x0323,
    Cast128MacGeneral = 0x0324,
    Cast128CbcPad = 0x0325,
    Rc5KeyGen = 0x0330,
    Rc5Ecb = 0x0331,
    Rc5Cbc = 0x0332,
    Rc5Mac = 0x0333,
    Rc5MacGeneral = 0x0334,
    Rc5CbcPad = 0x0335,
    IdeaKeyGen = 0x0340,
    IdeaEcb = 0x0341,
    IdeaCbc = 0x0342,
    IdeaMac = 0x0343,
    IdeaMacGeneral = 0x0344,
    IdeaCbcPad = 0x0345,
    GenericSecretKeyGen = 0x0350,
    ConcatenateBaseAndKey = 0x0360,
    ConcatenateBaseAndData = 0x0362,
    ConcatenateDataAndBase = 0x0363,
    XorBaseAndData = 0x0364,
    ExtractKeyFromKey = 0x0365,
    Ssl3PreMasterKeyGen = 0x0370,
    Ssl3MasterKeyDerive = 0x0371,
    Ssl3KeyAndMacDerive = 0x0372,
    Ssl3MasterKeyDeriveDh = 0x0373,
    TlsPreMasterKeyGen = 0x0374,
    TlsMasterKeyDerive = 0x0375,
    TlsKeyAndMacDerive = 0x0376,
    TlsMasterKeyDeriveDh = 0x0377,
    TlsPrf = 0x0378,
    Ssl3Md5Mac = 0x0380,
    Ssl3Sha1Mac = 0x0381,
    Md5KeyDerivation = 0x0390,
    Md2KeyDerivation = 0x0391,
    Sha1KeyDerivation = 0x0392,
    Sha256KeyDerivation = 0x0393,
    Sha384KeyDerivation = 0x0394,
    Sha512KeyDerivation = 0x0395,
    Sha224KeyDerivation = 0x0396,
    PbeMd2DesCbc = 0x03a0,
    PbeMd5DesCbc = 0x03a1,
    PbeMd5CastCbc = 0x03a2,
    PbeMd5Cast3Cbc = 0x03a3,
    PbeMd5Cast128Cbc = 0x03a4,
    PbeSha1Cast128Cbc = 0x03a5,
    PbeSha1Rc4128 = 0x03a6,
    PbeSha1Rc440 = 0x03a7,
    PbeSha1Des3EdeCbc = 0x03a8,
    PbeSha1Des2EdeCbc = 0x03a9,
    PbeSha1Rc2128Cbc = 0x03aa,
    PbeSha1Rc240Cbc = 0x03ab,
    Pkcs5Pbkd2 = 0x03b0,
    PbaSha1WithSha1Hmac = 0x03c0,
    WtlsPreMasterKeyGen = 0x03d0,
    WtlsMasterKeyDerive = 0x03d1,
    WtlsMasterKeyDeriveDhEcc = 0x03d2,
    WtlsPrf = 0x03d3,
    WtlsServerKeyAndMacDerive = 0x03d4,
    WtlsClientKeyAndMacDerive = 0x03d5,
    Tls10MacServer = 0x03d6,
    Tls10MacClient = 0x03d7,
    Tls12Mac = 0x03d8,
    Tls12Kdf = 0x03d9,
    Tls12MasterKeyDerive = 0x03e0,
    Tls12KeyAndMacDerive = 0x03e1,
    Tls12MasterKeyDeriveDh = 0x03e2,
    Tls12KeySafeDerive = 0x03e3,
    TlsMac = 0x03e4,
    TlsKdf = 0x03e5,
    KeyWrapLynks = 0x0400,
    KeyWrapSetOaep = 0x0401,
    CmsSig = 0x0500,
    KipDerive = 0x0510,
    KipWrap = 0x0511,
    KipMac = 0x0512,
    CamelliaKeyGen = 0x0550,
    CamelliaEcb = 0x0551,
    CamelliaCbc = 0x0552,
    CamelliaMac = 0x0553,
    CamelliaMacGeneral = 0x0554,
    CamelliaCbcPad = 0x0555,
    CamelliaEcbEncryptData = 0x0556,
    CamelliaCbcEncryptData = 0x0557,
    CamelliaCtr = 0x0558,
    AriaKeyGen = 0x0560,
    AriaEcb = 0x0561,
    AriaCbc = 0x0562,
    AriaMac = 0x0563,
    AriaMacGeneral = 0x0564,
    AriaCbcPad = 0x0565,
    AriaEcbEncryptData = 0x0566,
    AriaCbcEncryptData = 0x0567,
    SeedKeyGen = 0x0650,
    SeedEcb = 0x0651,
    SeedCbc = 0x0652,
    SeedMac = 0x0653,
    SeedMacGeneral = 0x0654,
    SeedCbcPad = 0x0655,
    SeedEcbEncryptData = 0x0656,
    SeedCbcEncryptData = 0x0657,
    SkipjackKeyGen = 0x1000,
    SkipjackEcb64 = 0x1001,
    SkipjackCbc64 = 0x1002,
    SkipjackOfb64 = 0x1003,
    SkipjackCfb64 = 0x1004,
    SkipjackCfb32 = 0x1005,
    SkipjackCfb16 = 0x1006,
    SkipjackCfb8 = 0x1007,
    SkipjackWrap = 0x1008,
    SkipjackPrivateWrap = 0x1009,
    SkipjackRelayx = 0x100a,
    KeaKeyPairGen = 0x1010,
    KeaKeyDerive = 0x1011,
    KeaDerive = 0x1012,
    FortezzaTimestamp = 0x1020,
    BatonKeyGen = 0x1030,
    BatonEcb128 = 0x1031,
    BatonEcb96 = 0x1032,
    BatonCbc128 = 0x1033,
    BatonCounter = 0x1034,
    BatonShuffle = 0x1035,
    BatonWrap = 0x1036,
    EcKeyPairGen = 0x1040,
    Ecdsa = 0x1041,
    EcdsaSha1 = 0x1042,
    EcdsaSha224 = 0x1043,
    EcdsaSha256 = 0x1044,
    EcdsaSha384 = 0x1045,
    EcdsaSha512 = 0x1046,
    Ecdh1Derive = 0x1050,
    Ecdh1CofactorDerive = 0x1051,
    EcmqvDerive = 0x1052,
    EcdhAesKeyWrap = 0x1053,
    RsaAesKeyWrap = 0x1054,
    JuniperKeyGen = 0x1060,
    JuniperEcb128 = 0x1061,
    JuniperCbc128 = 0x1062,
    JuniperCounter = 0x1063,
    JuniperShuffle = 0x1064,
    JuniperWrap = 0x1065,
    Fasthash = 0x1070,
    AesKeyGen = 0x1080,
    AesEcb = 0x1081,
    AesCbc = 0x1082,
    AesMac = 0x1083,
    AesMacGeneral = 0x1084,
    AesCbcPad = 0x1085,
    AesCtr = 0x1086,
    AesGcm = 0x1087,
    AesCcm = 0x1088,
    AesCts = 0x1089,
    AesCmac = 0x108a,
    AesCmacGeneral = 0x108b,
    AesXcbcMac = 0x108c,
    AesXcbcMac96 = 0x108d,
    AesGmac = 0x108e,
    BlowfishKeyGen = 0x1090,
    BlowfishCbc = 0x1091,
    TwofishKeyGen = 0x1092,
    TwofishCbc = 0x1093,
    BlowfishCbcPad = 0x1094,
    TwofishCbcPad = 0x1095,
    DesEcbEncryptData = 0x1100,
    DesCbcEncryptData = 0x1101,
    Des3EcbEncryptData = 0x1102,
    Des3CbcEncryptData = 0x1103,
    AesEcbEncryptData = 0x1104,
    AesCbcEncryptData = 0x1105,
    Gostr3410KeyPairGen = 0x1200,
    Gostr3410 = 0x1201,
    Gostr3410WithGostr3411 = 0x1202,
    Gostr3410KeyWrap = 0x1203,
    Gostr3410Derive = 0x1204,
    Gostr3411 = 0x1210,
    Gostr3411Hmac = 0x1211,
    Gost28147KeyGen = 0x1220,
    Gost28147Ecb = 0x1221,
    Gost28147 = 0x1222,
    Gost28147Mac = 0x1223,
    Gost28147KeyWrap = 0x1224,
    DsaParameterGen = 0x2000,
    DhPkcsParameterGen = 0x2001,
    X942DhParameterGen = 0x2002,
    DsaProbablisticParameterGen = 0x2003,
    DsaShaweTaylorParameterGen = 0x2004,
    AesOfb = 0x2104,
    AesCfb64 = 0x2105,
    AesCfb8 = 0x2106,
    AesCfb128 = 0x2107,
    AesCfb1 = 0x2108,
    AesKeyWrap = 0x2109,
    AesKeyWrapPad = 0x210a,
    RsaPkcsTpm11 = 0x4001,
    RsaPkcsOaepTpm11 = 0x4002,
    VendorDefined = 0x80000000,
}

impl TryFrom<c_ulong> for MechanismType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        MechanismType::from_u64(value.into()).ok_or(())
    }
}

impl TryFrom<MechanismType> for u64 {
    type Error = ();
    fn try_from(value: MechanismType) -> Result<Self, Self::Error> {
        MechanismType::to_u64(&value).ok_or(())
    }
}
