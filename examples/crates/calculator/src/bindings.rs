#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod math {
        #[allow(dead_code, clippy::all)]
        pub mod addition {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn add(x: i32, y: i32) -> i32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:math/addition@0.1.0")]
                    extern "C" {
                        #[link_name = "add"]
                        fn wit_import(_: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&x), _rt::as_i32(&y));
                    ret
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod example {
        #[allow(dead_code)]
        pub mod calculator {
            #[allow(dead_code, clippy::all)]
            pub mod calculate {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_evaluate_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::evaluate(_rt::string_lift(bytes0));
                    _rt::as_i32(result1)
                }
                pub trait Guest {
                    fn evaluate(expr: _rt::String) -> i32;
                }
                #[doc(hidden)]
                macro_rules! __export_example_calculator_calculate_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "example:calculator/calculate#evaluate"] unsafe extern "C" fn
                        export_evaluate(arg0 : * mut u8, arg1 : usize,) -> i32 {
                        $($path_to_types)*:: _export_evaluate_cabi::<$ty > (arg0, arg1) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_example_calculator_calculate_cabi;
            }
        }
    }
}
mod _rt {
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_calculator_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::example::calculator::calculate::__export_example_calculator_calculate_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::example::calculator::calculate);
    };
}
#[doc(inline)]
pub(crate) use __export_calculator_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:example:calculator:calculator:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 285] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9c\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x02\x01xz\x01yz\0z\x04\0\x03add\x01\0\x03\0\x1dcomponent:mat\
h/addition@0.1.0\x05\0\x01B\x02\x01@\x01\x04exprs\0z\x04\0\x08evaluate\x01\0\x04\
\0\x1cexample:calculator/calculate\x05\x01\x04\0\x1dexample:calculator/calculato\
r\x04\0\x0b\x10\x01\0\x0acalculator\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
