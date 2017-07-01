/// Re-export everything from std
pub use std::*;

/// Re-export everything from std
pub mod libraries {
    pub use std::*;

    pub use std::{
        fmt,
        string,
        iter,
        ops,
        cmp,
        ffi,
    };

    pub use fmt::{
        Display,
        Debug,
    };

    pub use string::{
        String,
    };

    pub use iter::{
        Iterator,
        IntoIterator,
    };

    pub use ops::{
        Add as OperatorAdd,
        Sub as OperatorSubtract,
        Mul as OperatorMultiply,
        Div as OperatorDivide,
    };

    pub use cmp::{
        Eq,
        PartialEq,
        Ord,
        PartialOrd,
    };

    pub use ffi::{
        CStr,
        CString,
        OsStr,
        OsString,
        FromBytesWithNulError,
        IntoStringError,
        NulError,
    };
}

/// Re-export built-in
pub mod built_in {
    /// Re-export types
    pub mod types {
        /// Re-export numbers
        pub mod numbers {
            /// Re-export signed integers
            pub mod signed_intergers {
                pub use i8 as int8;
                pub use i16 as int16;
                pub use i32 as int32;
                pub use i64 as int64;
            }

            /// Re-export unsigned integers
            pub mod unsigned_integers {
                pub use u8 as uint8;
                pub use u16 as uint16;
                pub use u32 as uint32;
                pub use u64 as uint64;
            }

            /// Re-export floating-point types
            pub mod floating_points {
                pub use f32 as float32;
                pub use f64 as float64;
            }
        }

        /// Re-export strings
        pub mod strings {
            pub use std::string;
            pub use std::string::*;
            pub use str;
            pub use string::String;
        }
    }
}

mod operator_overloading {
    pub use super::libraries::{ops, cmp};
    pub use self::ops::*;
    pub use self::cmp::*;
}
