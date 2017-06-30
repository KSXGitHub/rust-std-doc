/// Re-export everything from std
pub use std::*;

/// Re-export everything from std
pub mod my_std {
    /// Re-export everything from std
    pub use std::*;

    /// Re-export fmt
    pub mod my_fmt {
        pub use std::fmt;
        pub use std::fmt::*;

        pub use std::fmt::Display as MyDisplay;
        pub use std::fmt::Debug as MyDebug;
    }

    /// Re-export string
    pub mod my_string {
        pub use std::string;
        pub use std::string::*;

        pub use string::String as MyString;
    }

    /// Re-export iter
    pub mod my_iter {
        pub use std::iter;
        pub use std::iter::*;

        pub use std::iter::Iterator as MyIterator;
    }

    /// Re-export ffi
    pub mod my_ffi {
        pub use std::ffi;
        pub use std::ffi::*;

        pub use std::ffi::OsString as MyOsString;
    }
}

/// Re-export built-in
pub mod my_built_in {
    /// Re-export types
    pub mod my_types {
        /// Re-export numbers
        pub mod my_numbers {
            /// Re-export signed integers
            pub mod my_signed_intergers {
                pub use i8 as int8;
                pub use i16 as int16;
                pub use i32 as int32;
                pub use i64 as int64;
            }

            /// Re-export unsigned integers
            pub mod my_unsigned_integers {
                pub use u8 as uint8;
                pub use u16 as uint16;
                pub use u32 as uint32;
                pub use u64 as uint64;
            }

            /// Re-export floating-point types
            pub mod my_floating_points {
                pub use f32 as float32;
                pub use f64 as float64;
            }
        }

        /// Re-export strings
        pub mod my_strings {
            pub use std::string;
            pub use std::string::*;
            pub use str as my_str;
            pub use string::String as MyString;
        }
    }
}
