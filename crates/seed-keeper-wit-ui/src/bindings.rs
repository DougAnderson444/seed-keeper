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
    #[allow(dead_code)]
    pub mod wit_ui {
        #[allow(dead_code, clippy::all)]
        pub mod wurbo_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Encrypted = _rt::String;
            /// Details required in order to add an event listener to an element
            #[derive(Clone)]
            pub struct ListenDetails {
                pub selector: _rt::String,
                pub ty: _rt::String,
            }
            impl ::core::fmt::Debug for ListenDetails {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ListenDetails")
                        .field("selector", &self.selector)
                        .field("ty", &self.ty)
                        .finish()
                }
            }
            /// Context for the minijinja rendering
            #[derive(Clone)]
            pub struct Page {
                pub title: _rt::String,
            }
            impl ::core::fmt::Debug for Page {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Page").field("title", &self.title).finish()
                }
            }
            #[derive(Clone)]
            pub struct Input {
                pub placeholder: _rt::String,
                pub encrypted_seed: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for Input {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Input")
                        .field("placeholder", &self.placeholder)
                        .field("encrypted-seed", &self.encrypted_seed)
                        .finish()
                }
            }
            /// Content for the entire page
            #[derive(Clone)]
            pub struct Content {
                pub page: Option<Page>,
                pub input: Option<Input>,
                pub load: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for Content {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Content")
                        .field("page", &self.page)
                        .field("input", &self.input)
                        .field("load", &self.load)
                        .finish()
                }
            }
            /// Context variants
            #[derive(Clone)]
            pub enum Context {
                AllContent(Content),
                Username(_rt::String),
                Password(_rt::String),
                Encrypted(Encrypted),
                Submit,
            }
            impl ::core::fmt::Debug for Context {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Context::AllContent(e) => {
                            f.debug_tuple("Context::AllContent").field(e).finish()
                        }
                        Context::Username(e) => {
                            f.debug_tuple("Context::Username").field(e).finish()
                        }
                        Context::Password(e) => {
                            f.debug_tuple("Context::Password").field(e).finish()
                        }
                        Context::Encrypted(e) => {
                            f.debug_tuple("Context::Encrypted").field(e).finish()
                        }
                        Context::Submit => f.debug_tuple("Context::Submit").finish(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod wurbo_in {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type ListenDetails = super::super::super::seed_keeper::wit_ui::wurbo_types::ListenDetails;
            #[allow(unused_unsafe, clippy::all)]
            /// Add an event listener to the given element
            pub fn addeventlistener(details: &ListenDetails) {
                unsafe {
                    let super::super::super::seed_keeper::wit_ui::wurbo_types::ListenDetails {
                        selector: selector0,
                        ty: ty0,
                    } = details;
                    let vec1 = selector0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let vec2 = ty0;
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wit-ui/wurbo-in@0.1.0")]
                    extern "C" {
                        #[link_name = "addeventlistener"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr1.cast_mut(), len1, ptr2.cast_mut(), len2);
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Emit events from this component. Messages should be serialized JSON strings of Event type.
            pub fn emit(message: &str) {
                unsafe {
                    let vec0 = message;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wit-ui/wurbo-in@0.1.0")]
                    extern "C" {
                        #[link_name = "emit"]
                        fn wit_import(_: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0);
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
        pub mod wit_ui {
            #[allow(dead_code, clippy::all)]
            pub mod wurbo_out {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Context = super::super::super::super::seed_keeper::wit_ui::wurbo_types::Context;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_render_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: i32,
                    arg5: *mut u8,
                    arg6: usize,
                    arg7: i32,
                    arg8: *mut u8,
                    arg9: usize,
                    arg10: i32,
                    arg11: *mut u8,
                    arg12: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    use super::super::super::super::seed_keeper::wit_ui::wurbo_types::Context as V7;
                    let v7 = match arg0 {
                        0 => {
                            let e7 = super::super::super::super::seed_keeper::wit_ui::wurbo_types::Content {
                                page: match arg1 as i32 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let len0 = arg3;
                                            let bytes0 = _rt::Vec::from_raw_parts(
                                                arg2.cast(),
                                                len0,
                                                len0,
                                            );
                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Page {
                                                title: _rt::string_lift(bytes0),
                                            }
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                },
                                input: match arg4 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let len1 = arg6;
                                            let bytes1 = _rt::Vec::from_raw_parts(
                                                arg5.cast(),
                                                len1,
                                                len1,
                                            );
                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Input {
                                                placeholder: _rt::string_lift(bytes1),
                                                encrypted_seed: match arg7 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let len2 = arg9;
                                                            let bytes2 = _rt::Vec::from_raw_parts(
                                                                arg8.cast(),
                                                                len2,
                                                                len2,
                                                            );
                                                            _rt::string_lift(bytes2)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                },
                                load: match arg10 {
                                    0 => None,
                                    1 => {
                                        let e = {
                                            let len3 = arg12;
                                            let bytes3 = _rt::Vec::from_raw_parts(
                                                arg11.cast(),
                                                len3,
                                                len3,
                                            );
                                            _rt::string_lift(bytes3)
                                        };
                                        Some(e)
                                    }
                                    _ => _rt::invalid_enum_discriminant(),
                                },
                            };
                            V7::AllContent(e7)
                        }
                        1 => {
                            let e7 = {
                                let len4 = arg2 as usize;
                                let bytes4 = _rt::Vec::from_raw_parts(
                                    arg1.cast(),
                                    len4,
                                    len4,
                                );
                                _rt::string_lift(bytes4)
                            };
                            V7::Username(e7)
                        }
                        2 => {
                            let e7 = {
                                let len5 = arg2 as usize;
                                let bytes5 = _rt::Vec::from_raw_parts(
                                    arg1.cast(),
                                    len5,
                                    len5,
                                );
                                _rt::string_lift(bytes5)
                            };
                            V7::Password(e7)
                        }
                        3 => {
                            let e7 = {
                                let len6 = arg2 as usize;
                                let bytes6 = _rt::Vec::from_raw_parts(
                                    arg1.cast(),
                                    len6,
                                    len6,
                                );
                                _rt::string_lift(bytes6)
                            };
                            V7::Encrypted(e7)
                        }
                        n => {
                            debug_assert_eq!(n, 4, "invalid enum discriminant");
                            V7::Submit
                        }
                    };
                    let result8 = T::render(v7);
                    let ptr9 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result8 {
                        Ok(e) => {
                            *ptr9.add(0).cast::<u8>() = (0i32) as u8;
                            let vec10 = (e.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr9.add(8).cast::<usize>() = len10;
                            *ptr9.add(4).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                        Err(e) => {
                            *ptr9.add(0).cast::<u8>() = (1i32) as u8;
                            let vec11 = (e.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *ptr9.add(8).cast::<usize>() = len11;
                            *ptr9.add(4).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                    };
                    ptr9
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_render<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(4).cast::<*mut u8>();
                            let l4 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_activate_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::activate(
                        match arg0 {
                            0 => None,
                            1 => {
                                let e = {
                                    let base3 = arg1;
                                    let len3 = arg2;
                                    let mut result3 = _rt::Vec::with_capacity(len3);
                                    for i in 0..len3 {
                                        let base = base3.add(i * 8);
                                        let e3 = {
                                            let l0 = *base.add(0).cast::<*mut u8>();
                                            let l1 = *base.add(4).cast::<usize>();
                                            let len2 = l1;
                                            let bytes2 = _rt::Vec::from_raw_parts(
                                                l0.cast(),
                                                len2,
                                                len2,
                                            );
                                            _rt::string_lift(bytes2)
                                        };
                                        result3.push(e3);
                                    }
                                    _rt::cabi_dealloc(base3, len3 * 8, 4);
                                    result3
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_customize_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base6 = arg0;
                    let len6 = arg1;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 16);
                        let e6 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
                            let l3 = *base.add(8).cast::<*mut u8>();
                            let l4 = *base.add(12).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            (_rt::string_lift(bytes2), _rt::string_lift(bytes5))
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 16, 4);
                    let result7 = T::customize(result6);
                    let ptr8 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result7 {
                        Ok(_) => {
                            *ptr8.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr8.add(0).cast::<u8>() = (1i32) as u8;
                            let vec9 = (e.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr8.add(8).cast::<usize>() = len9;
                            *ptr8.add(4).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    };
                    ptr8
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_customize<T: Guest>(arg0: *mut u8) {
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
                    /// renders the initial Web component with the given data
                    /// and the target template to use as top level entry point
                    fn render(ctx: Context) -> Result<_rt::String, _rt::String>;
                    /// listen on all or given selectors
                    fn activate(selectors: Option<_rt::Vec<_rt::String>>);
                    /// Optionally customize the configuration of the templates used to render the component
                    fn customize(
                        templates: _rt::Vec<(_rt::String, _rt::String)>,
                    ) -> Result<(), _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_seed_keeper_wit_ui_wurbo_out_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "seed-keeper:wit-ui/wurbo-out@0.1.0#render"] unsafe extern "C" fn
                        export_render(arg0 : i32, arg1 : * mut u8, arg2 : * mut u8, arg3
                        : usize, arg4 : i32, arg5 : * mut u8, arg6 : usize, arg7 : i32,
                        arg8 : * mut u8, arg9 : usize, arg10 : i32, arg11 : * mut u8,
                        arg12 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_render_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5,
                        arg6, arg7, arg8, arg9, arg10, arg11, arg12) } #[export_name =
                        "cabi_post_seed-keeper:wit-ui/wurbo-out@0.1.0#render"] unsafe
                        extern "C" fn _post_return_render(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_render::<$ty > (arg0) }
                        #[export_name = "seed-keeper:wit-ui/wurbo-out@0.1.0#activate"]
                        unsafe extern "C" fn export_activate(arg0 : i32, arg1 : * mut u8,
                        arg2 : usize,) { $($path_to_types)*:: _export_activate_cabi::<$ty
                        > (arg0, arg1, arg2) } #[export_name =
                        "seed-keeper:wit-ui/wurbo-out@0.1.0#customize"] unsafe extern "C"
                        fn export_customize(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_customize_cabi::<$ty > (arg0, arg1)
                        } #[export_name =
                        "cabi_post_seed-keeper:wit-ui/wurbo-out@0.1.0#customize"] unsafe
                        extern "C" fn _post_return_customize(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_customize::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_seed_keeper_wit_ui_wurbo_out_0_1_0_cabi;
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
macro_rules! __export_seedworld_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::seed_keeper::wit_ui::wurbo_out::__export_seed_keeper_wit_ui_wurbo_out_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::seed_keeper::wit_ui::wurbo_out);
    };
}
#[doc(inline)]
pub(crate) use __export_seedworld_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:seed-keeper:wit-ui@0.1.0:seedworld:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1025] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x81\x07\x01A\x02\x01\
A\x0d\x01B\x04\x01p}\x01k\0\x01r\x03\x08username\0\x08password\0\x09encrypted\x01\
\x04\0\x0bcredentials\x03\0\x02\x03\0\x1eseed-keeper:wallet/types@0.1.0\x05\0\x02\
\x03\0\0\x0bcredentials\x01B\x0a\x02\x03\x02\x01\x01\x04\0\x0bcredentials\x03\0\0\
\x01j\0\x01s\x01@\x01\x06config\x01\0\x02\x04\0\x0aset-config\x01\x03\x01p}\x01j\
\x01\x04\x01s\x01@\0\0\x05\x04\0\x0dget-encrypted\x01\x06\x04\0\x08get-seed\x01\x06\
\x03\0\x1fseed-keeper:wallet/config@0.1.0\x05\x02\x01B\x0f\x01s\x04\0\x09encrypt\
ed\x03\0\0\x01r\x02\x08selectors\x02tys\x04\0\x0elisten-details\x03\0\x02\x01r\x01\
\x05titles\x04\0\x04page\x03\0\x04\x01ks\x01r\x02\x0bplaceholders\x0eencrypted-s\
eed\x06\x04\0\x05input\x03\0\x07\x01k\x05\x01k\x08\x01r\x03\x04page\x09\x05input\
\x0a\x04load\x06\x04\0\x07content\x03\0\x0b\x01q\x05\x0ball-content\x01\x0c\0\x08\
username\x01s\0\x08password\x01s\0\x09encrypted\x01\x01\0\x06submit\0\0\x04\0\x07\
context\x03\0\x0d\x03\0$seed-keeper:wit-ui/wurbo-types@0.1.0\x05\x03\x02\x03\0\x02\
\x0elisten-details\x01B\x06\x02\x03\x02\x01\x04\x04\0\x0elisten-details\x03\0\0\x01\
@\x01\x07details\x01\x01\0\x04\0\x10addeventlistener\x01\x02\x01@\x01\x07message\
s\x01\0\x04\0\x04emit\x01\x03\x03\0!seed-keeper:wit-ui/wurbo-in@0.1.0\x05\x05\x02\
\x03\0\x02\x07context\x01B\x0e\x02\x03\x02\x01\x06\x04\0\x07context\x03\0\0\x01j\
\x01s\x01s\x01@\x01\x03ctx\x01\0\x02\x04\0\x06render\x01\x03\x01ps\x01k\x04\x01@\
\x01\x09selectors\x05\x01\0\x04\0\x08activate\x01\x06\x01o\x02ss\x01p\x07\x01j\0\
\x01s\x01@\x01\x09templates\x08\0\x09\x04\0\x09customize\x01\x0a\x04\0\"seed-kee\
per:wit-ui/wurbo-out@0.1.0\x05\x07\x04\0\"seed-keeper:wit-ui/seedworld@0.1.0\x04\
\0\x0b\x0f\x01\0\x09seedworld\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0d\
wit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
