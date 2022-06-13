use core::fmt;

pub mod signer;
pub mod constants;
pub mod fitsigner;
pub mod signatures;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HIPError {
    /// An operation is not permitted in the current state.
    /// i.e. an invalid HIP state was reached.
    InvalidState,
    /// An operation cannot proceed because a buffer is empty or full.
    Exhausted,
    /// HIP has variable length parameters i.e. contents field length + padding
    /// may be computed at runtime. Variant to indicate if the allocated buffer
    /// is too short.
    Bufferistooshort,
    /// If the length field of a HIP parameter packet is not equal to size of
    /// its contents
    IncorrectHeaderLength,
    /// A HIP parameter packet size has to be a mutiple of 8
    LengthNotMultiple8,
    /// The value of a field in a param packet was already set. Ex: header field
    /// was not set.
    FieldisAlreadySet,
    /// The value of a field in a param packet was not set
    FieldNotSet,
    /// Error while performing an EC Crypto operation
    ECCError,
    /// Invalid encoding
    InvalidEncoding,
    /// Signature Error
    SignatureError,
    /// Invalid buffer length
    IncorrectLength,
    /// Key, Value Insertion Failed
    MapInsertionOpFailed,
    /// Unrecongnized is blanket error type for anything that we dont recognize
    /// in HIPv2 standard
    Unrecognized,
    /// A timer expired. Could be a `HIP BEX timeout or data timeout or an impl-specific timeout`
    TimeOut,

    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustbootError {
    /// An operation is not permitted in the current state or an invalid state was reached.
    InvalidState,
    /// Firmware authentication failed
    FwAuthFailed,
    /// Image integrity verification failed.
    IntegrityCheckFailed,
    /// The val of the size field in an image header is not valid
    InvalidFirmwareSize,
    /// Type, length, value triple does not exist i.e. tried to parse the header
    /// for a given a `field_type` but we reached the `end of header`.
    TLVNotFound,
    /// The hash output or length is invalid .
    BadHashValue,
    /// The value of a field in a param packet was not set
    FieldNotSet,
    /// Error while performing an `EC Crypto operation`
    ECCError,
    /// The image in a given partition is malformed. Ex:`magic` field or `trailer magic`
    /// has an invalid value.
    InvalidImage,
    /// Something's wrong with the signature stored in the header.
    BadSignature,
    /// The value associated with the requested TLV is too large i.e. invalid.
    InvalidHdrFieldLength,
    /// Suppose to be unreachable
    Unreachable,
    /// Null value
    NullValue,
    /// The requested header field has an invalid value.
    InvalidValue,
    /// Attempt to reinitialize a global mutable static.  
    StaticReinit,
    /// The sector flag value is invalid
    InvalidSectFlag,

    #[doc(hidden)]
    __Nonexhaustive,
}

/// The result type for HIP.
pub type Result<T> = core::result::Result<T, HIPError>;


#[rustfmt::skip]
impl fmt::Display for HIPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HIPError::InvalidState             => write!(f, "Invalid State, operation not permitted"),
            &HIPError::Exhausted                => write!(f, "Buffer is empty or full"),
            &HIPError::Bufferistooshort         => write!(f, "Buffer size is insufficent - too small"),
            &HIPError::IncorrectHeaderLength    => write!(f, "Malformed packet"),
            &HIPError::LengthNotMultiple8       => write!(f, "Length has to be multiple of 8"),
            &HIPError::FieldisAlreadySet        => write!(f, "The field was already set"),
            &HIPError::FieldNotSet              => write!(f, "The field is not set"),
            &HIPError::ECCError                 => write!(f, "EC Crypto operation failed"),
            &HIPError::InvalidEncoding          => write!(f, "Invalid encoding"),
            &HIPError::SignatureError           => write!(f, "Signature Error"),
            &HIPError::IncorrectLength          => write!(f, "The length of a buffer is invalid"),
            &HIPError::MapInsertionOpFailed     => write!(f, "New key, value insertion failed"),
            &HIPError::Unrecognized             => write!(f, "Unrecognized item"),
            &HIPError::TimeOut                  => write!(f, "Timeout Error"),
            &HIPError::__Nonexhaustive          => unreachable!(),
        }
    }
}