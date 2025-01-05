#[allow(dead_code)]
pub mod seed_keeper {
    #[allow(dead_code)]
    pub mod wallet {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// The confuration of the seed keeper
            #[derive(Clone)]
            pub struct Credentials {
                /// The username to use for the seed keeper
                pub username: _rt::Vec<u8>,
                /// The password to use for the seed keeper
                pub password: _rt::Vec<u8>,
                /// Optional prevously generated encrypted seed to use for the seed keeper
                pub encrypted: Option<_rt::Vec<u8>>,
            }
            impl ::core::fmt::Debug for Credentials {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Credentials")
                        .field("username", &self.username)
                        .field("password", &self.password)
                        .field("encrypted", &self.encrypted)
                        .finish()
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod config {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Credentials = super::super::super::seed_keeper::wallet::types::Credentials;
            #[allow(unused_unsafe, clippy::all)]
            /// Load into the component from an encrypted seed, password, and salt (username)
            /// Returns the encrypted seed or an error
            pub fn set_config(config: &Credentials) -> Result<(), _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let super::super::super::seed_keeper::wallet::types::Credentials {
                        username: username0,
                        password: password0,
                        encrypted: encrypted0,
                    } = config;
                    let vec1 = username0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = password0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    let (result4_0, result4_1, result4_2) = match encrypted0 {
                        Some(e) => {
                            let vec3 = e;
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            (1i32, ptr3.cast_mut(), len3)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    let ptr5 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wallet/config@0.1.0")]
                    extern "C" {
                        #[link_name = "set-config"]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        ptr1.cast_mut(),
                        len1,
                        ptr2.cast_mut(),
                        len2,
                        result4_0,
                        result4_1,
                        result4_2,
                        ptr5,
                    );
                    let l6 = i32::from(*ptr5.add(0).cast::<u8>());
                    match l6 {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l7 = *ptr5.add(4).cast::<*mut u8>();
                                let l8 = *ptr5.add(8).cast::<usize>();
                                let len9 = l8;
                                let bytes9 = _rt::Vec::from_raw_parts(
                                    l7.cast(),
                                    len9,
                                    len9,
                                );
                                _rt::string_lift(bytes9)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the encrypted seed or None if it doesn't exist
            pub fn get_encrypted() -> Result<_rt::Vec<u8>, _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wallet/config@0.1.0")]
                    extern "C" {
                        #[link_name = "get-encrypted"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<*mut u8>();
                                let l3 = *ptr0.add(8).cast::<usize>();
                                let len4 = l3;
                                _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l5 = *ptr0.add(4).cast::<*mut u8>();
                                let l6 = *ptr0.add(8).cast::<usize>();
                                let len7 = l6;
                                let bytes7 = _rt::Vec::from_raw_parts(
                                    l5.cast(),
                                    len7,
                                    len7,
                                );
                                _rt::string_lift(bytes7)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the plaintext seed
            pub fn get_seed() -> Result<_rt::Vec<u8>, _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wallet/config@0.1.0")]
                    extern "C" {
                        #[link_name = "get-seed"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<*mut u8>();
                                let l3 = *ptr0.add(8).cast::<usize>();
                                let len4 = l3;
                                _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l5 = *ptr0.add(4).cast::<*mut u8>();
                                let l6 = *ptr0.add(8).cast::<usize>();
                                let len7 = l6;
                                let bytes7 = _rt::Vec::from_raw_parts(
                                    l5.cast(),
                                    len7,
                                    len7,
                                );
                                _rt::string_lift(bytes7)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod seed_keeper {
        #[allow(dead_code)]
        pub mod edwards_wit {
            #[allow(dead_code, clippy::all)]
            pub mod operations {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_sign_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::sign(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(8).cast::<usize>() = len4;
                            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_sign<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(4).cast::<*mut u8>();
                            let l5 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_verify_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let len1 = arg3;
                    let result2 = T::verify(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                        _rt::Vec::from_raw_parts(arg2.cast(), len1, len1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr3.add(4).cast::<u8>() = (match e {
                                true => 1,
                                false => 0,
                            }) as u8;
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr3.add(8).cast::<usize>() = len4;
                            *ptr3.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_verify<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn sign(message: _rt::Vec<u8>) -> Result<_rt::Vec<u8>, _rt::String>;
                    fn verify(
                        message: _rt::Vec<u8>,
                        signature: _rt::Vec<u8>,
                    ) -> Result<bool, _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_seed_keeper_edwards_wit_operations_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "seed-keeper:edwards-wit/operations@0.1.0#sign"] unsafe extern
                        "C" fn export_sign(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_sign_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_seed-keeper:edwards-wit/operations@0.1.0#sign"] unsafe
                        extern "C" fn _post_return_sign(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_sign::<$ty > (arg0) }
                        #[export_name =
                        "seed-keeper:edwards-wit/operations@0.1.0#verify"] unsafe extern
                        "C" fn export_verify(arg0 : * mut u8, arg1 : usize, arg2 : * mut
                        u8, arg3 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_verify_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "cabi_post_seed-keeper:edwards-wit/operations@0.1.0#verify"]
                        unsafe extern "C" fn _post_return_verify(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_verify::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_seed_keeper_edwards_wit_operations_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::string::String;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
macro_rules! __export_example_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::seed_keeper::edwards_wit::operations::__export_seed_keeper_edwards_wit_operations_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::seed_keeper::edwards_wit::operations);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:seed-keeper:edwards-wit@0.1.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 546] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa4\x03\x01A\x02\x01\
A\x07\x01B\x04\x01p}\x01k\0\x01r\x03\x08username\0\x08password\0\x09encrypted\x01\
\x04\0\x0bcredentials\x03\0\x02\x03\0\x1eseed-keeper:wallet/types@0.1.0\x05\0\x02\
\x03\0\0\x0bcredentials\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x0bcredentials\x03\0\0\
\x01j\0\x01s\x01@\x01\x06config\x01\0\x02\x04\0\x0aset-config\x01\x03\x01p}\x01j\
\x01\x04\x01s\x01@\0\0\x05\x04\0\x0dget-encrypted\x01\x06\x04\0\x08get-seed\x01\x06\
\x03\0\x1fseed-keeper:wallet/config@0.1.0\x05\x02\x01B\x07\x01p}\x01j\x01\0\x01s\
\x01@\x01\x07message\0\0\x01\x04\0\x04sign\x01\x02\x01j\x01\x7f\x01s\x01@\x02\x07\
message\0\x09signature\0\0\x03\x04\0\x06verify\x01\x04\x04\0(seed-keeper:edwards\
-wit/operations@0.1.0\x05\x03\x04\0%seed-keeper:edwards-wit/example@0.1.0\x04\0\x0b\
\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-com\
ponent\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
