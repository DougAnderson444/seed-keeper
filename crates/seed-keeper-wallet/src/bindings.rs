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
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod seed_keeper {
        #[allow(dead_code)]
        pub mod wallet {
            #[allow(dead_code, clippy::all)]
            pub mod config {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Credentials = super::super::super::super::seed_keeper::wallet::types::Credentials;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_set_config_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: i32,
                    arg5: *mut u8,
                    arg6: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let len1 = arg3;
                    let result3 = T::set_config(super::super::super::super::seed_keeper::wallet::types::Credentials {
                        username: _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                        password: _rt::Vec::from_raw_parts(arg2.cast(), len1, len1),
                        encrypted: match arg4 {
                            0 => None,
                            1 => {
                                let e = {
                                    let len2 = arg6;
                                    _rt::Vec::from_raw_parts(arg5.cast(), len2, len2)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    });
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result3 {
                        Ok(_) => {
                            *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                            let vec5 = (e.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr4.add(8).cast::<usize>() = len5;
                            *ptr4.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr4
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_set_config<T: Guest>(arg0: *mut u8) {
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
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_encrypted_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_encrypted();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(8).cast::<usize>() = len3;
                            *ptr1.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_encrypted<T: Guest>(arg0: *mut u8) {
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
                pub unsafe fn _export_get_seed_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_seed();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec2 = (e).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(8).cast::<usize>() = len3;
                            *ptr1.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_seed<T: Guest>(arg0: *mut u8) {
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
                pub trait Guest {
                    /// Load into the component from an encrypted seed, password, and salt (username)
                    /// Returns the encrypted seed or an error
                    fn set_config(config: Credentials) -> Result<(), _rt::String>;
                    /// Returns the encrypted seed or None if it doesn't exist
                    fn get_encrypted() -> Result<_rt::Vec<u8>, _rt::String>;
                    /// Get the plaintext seed
                    fn get_seed() -> Result<_rt::Vec<u8>, _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_seed_keeper_wallet_config_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "seed-keeper:wallet/config@0.1.0#set-config"] unsafe extern "C"
                        fn export_set_config(arg0 : * mut u8, arg1 : usize, arg2 : * mut
                        u8, arg3 : usize, arg4 : i32, arg5 : * mut u8, arg6 : usize,) ->
                        * mut u8 { $($path_to_types)*:: _export_set_config_cabi::<$ty >
                        (arg0, arg1, arg2, arg3, arg4, arg5, arg6) } #[export_name =
                        "cabi_post_seed-keeper:wallet/config@0.1.0#set-config"] unsafe
                        extern "C" fn _post_return_set_config(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_set_config::<$ty > (arg0) }
                        #[export_name = "seed-keeper:wallet/config@0.1.0#get-encrypted"]
                        unsafe extern "C" fn export_get_encrypted() -> * mut u8 {
                        $($path_to_types)*:: _export_get_encrypted_cabi::<$ty > () }
                        #[export_name =
                        "cabi_post_seed-keeper:wallet/config@0.1.0#get-encrypted"] unsafe
                        extern "C" fn _post_return_get_encrypted(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_encrypted::<$ty > (arg0) }
                        #[export_name = "seed-keeper:wallet/config@0.1.0#get-seed"]
                        unsafe extern "C" fn export_get_seed() -> * mut u8 {
                        $($path_to_types)*:: _export_get_seed_cabi::<$ty > () }
                        #[export_name =
                        "cabi_post_seed-keeper:wallet/config@0.1.0#get-seed"] unsafe
                        extern "C" fn _post_return_get_seed(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_seed::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_seed_keeper_wallet_config_0_1_0_cabi;
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
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
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
macro_rules! __export_keeper_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::seed_keeper::wallet::config::__export_seed_keeper_wallet_config_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::seed_keeper::wallet::config);
    };
}
#[doc(inline)]
pub(crate) use __export_keeper_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:seed-keeper:wallet@0.1.0:keeper:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 417] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa4\x02\x01A\x02\x01\
A\x05\x01B\x04\x01p}\x01k\0\x01r\x03\x08username\0\x08password\0\x09encrypted\x01\
\x04\0\x0bcredentials\x03\0\x02\x03\0\x1eseed-keeper:wallet/types@0.1.0\x05\0\x02\
\x03\0\0\x0bcredentials\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x0bcredentials\x03\0\0\
\x01j\0\x01s\x01@\x01\x06config\x01\0\x02\x04\0\x0aset-config\x01\x03\x01p}\x01j\
\x01\x04\x01s\x01@\0\0\x05\x04\0\x0dget-encrypted\x01\x06\x04\0\x08get-seed\x01\x06\
\x04\0\x1fseed-keeper:wallet/config@0.1.0\x05\x02\x04\0\x1fseed-keeper:wallet/ke\
eper@0.1.0\x04\0\x0b\x0c\x01\0\x06keeper\x03\0\0\0G\x09producers\x01\x0cprocesse\
d-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
