#[allow(dead_code)]
pub mod example {
    #[allow(dead_code)]
    pub mod edwards_ui {
        #[allow(dead_code, clippy::all)]
        pub mod wurbo_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
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
            }
            impl ::core::fmt::Debug for Input {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Input")
                        .field("placeholder", &self.placeholder)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Output {
                /// the resulting value of the total outputs combined
                pub value: Option<_rt::String>,
                /// optional id string: None is intial render, Some for update value
                pub id: Option<_rt::String>,
                /// the output dest for the message changes
                pub message: Option<_rt::String>,
                /// the output dest for the signature changes
                pub signature: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for Output {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Output")
                        .field("value", &self.value)
                        .field("id", &self.id)
                        .field("message", &self.message)
                        .field("signature", &self.signature)
                        .finish()
                }
            }
            /// COntent for the entire page
            #[derive(Clone)]
            pub struct Content {
                pub page: Page,
                pub input: Input,
                pub output: Option<Output>,
            }
            impl ::core::fmt::Debug for Content {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Content")
                        .field("page", &self.page)
                        .field("input", &self.input)
                        .field("output", &self.output)
                        .finish()
                }
            }
            /// Context variants
            #[derive(Clone)]
            pub enum Context {
                AllContent(Content),
                Message(_rt::String),
                Submit(_rt::String),
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
                        Context::Message(e) => {
                            f.debug_tuple("Context::Message").field(e).finish()
                        }
                        Context::Submit(e) => {
                            f.debug_tuple("Context::Submit").field(e).finish()
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod wurbo_in {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type ListenDetails = super::super::super::example::edwards_ui::wurbo_types::ListenDetails;
            #[allow(unused_unsafe, clippy::all)]
            /// Add an event listener to the given element
            pub fn addeventlistener(details: &ListenDetails) {
                unsafe {
                    let super::super::super::example::edwards_ui::wurbo_types::ListenDetails {
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
                    #[link(wasm_import_module = "example:edwards-ui/wurbo-in@0.1.0")]
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
        }
    }
}
#[allow(dead_code)]
pub mod seed_keeper {
    #[allow(dead_code)]
    pub mod edwards_wit {
        #[allow(dead_code, clippy::all)]
        pub mod operations {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn sign(message: &[u8]) -> Result<_rt::Vec<u8>, _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let vec0 = message;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "seed-keeper:edwards-wit/operations@0.1.0"
                    )]
                    extern "C" {
                        #[link_name = "sign"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<*mut u8>();
                                let l4 = *ptr1.add(8).cast::<usize>();
                                let len5 = l4;
                                _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l6 = *ptr1.add(4).cast::<*mut u8>();
                                let l7 = *ptr1.add(8).cast::<usize>();
                                let len8 = l7;
                                let bytes8 = _rt::Vec::from_raw_parts(
                                    l6.cast(),
                                    len8,
                                    len8,
                                );
                                _rt::string_lift(bytes8)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn verify(
                message: &[u8],
                signature: &[u8],
            ) -> Result<bool, _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let vec0 = message;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let vec1 = signature;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "seed-keeper:edwards-wit/operations@0.1.0"
                    )]
                    extern "C" {
                        #[link_name = "verify"]
                        fn wit_import(
                            _: *mut u8,
                            _: usize,
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
                        _: *mut u8,
                    ) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1.cast_mut(), len1, ptr2);
                    let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                    match l3 {
                        0 => {
                            let e = {
                                let l4 = i32::from(*ptr2.add(4).cast::<u8>());
                                _rt::bool_lift(l4 as u8)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l5 = *ptr2.add(4).cast::<*mut u8>();
                                let l6 = *ptr2.add(8).cast::<usize>();
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
    pub mod example {
        #[allow(dead_code)]
        pub mod edwards_ui {
            #[allow(dead_code, clippy::all)]
            pub mod wurbo_out {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Context = super::super::super::super::example::edwards_ui::wurbo_types::Context;
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
                pub unsafe fn _export_render_cabi<T: Guest>(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    use super::super::super::super::example::edwards_ui::wurbo_types::Context as V30;
                    let v30 = match l0 {
                        0 => {
                            let e30 = {
                                let l1 = *arg0.add(4).cast::<*mut u8>();
                                let l2 = *arg0.add(8).cast::<usize>();
                                let len3 = l2;
                                let bytes3 = _rt::Vec::from_raw_parts(
                                    l1.cast(),
                                    len3,
                                    len3,
                                );
                                let l4 = *arg0.add(12).cast::<*mut u8>();
                                let l5 = *arg0.add(16).cast::<usize>();
                                let len6 = l5;
                                let bytes6 = _rt::Vec::from_raw_parts(
                                    l4.cast(),
                                    len6,
                                    len6,
                                );
                                let l7 = i32::from(*arg0.add(20).cast::<u8>());
                                super::super::super::super::example::edwards_ui::wurbo_types::Content {
                                    page: super::super::super::super::example::edwards_ui::wurbo_types::Page {
                                        title: _rt::string_lift(bytes3),
                                    },
                                    input: super::super::super::super::example::edwards_ui::wurbo_types::Input {
                                        placeholder: _rt::string_lift(bytes6),
                                    },
                                    output: match l7 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l8 = i32::from(*arg0.add(24).cast::<u8>());
                                                let l12 = i32::from(*arg0.add(36).cast::<u8>());
                                                let l16 = i32::from(*arg0.add(48).cast::<u8>());
                                                let l20 = i32::from(*arg0.add(60).cast::<u8>());
                                                super::super::super::super::example::edwards_ui::wurbo_types::Output {
                                                    value: match l8 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l9 = *arg0.add(28).cast::<*mut u8>();
                                                                let l10 = *arg0.add(32).cast::<usize>();
                                                                let len11 = l10;
                                                                let bytes11 = _rt::Vec::from_raw_parts(
                                                                    l9.cast(),
                                                                    len11,
                                                                    len11,
                                                                );
                                                                _rt::string_lift(bytes11)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    id: match l12 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l13 = *arg0.add(40).cast::<*mut u8>();
                                                                let l14 = *arg0.add(44).cast::<usize>();
                                                                let len15 = l14;
                                                                let bytes15 = _rt::Vec::from_raw_parts(
                                                                    l13.cast(),
                                                                    len15,
                                                                    len15,
                                                                );
                                                                _rt::string_lift(bytes15)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    message: match l16 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l17 = *arg0.add(52).cast::<*mut u8>();
                                                                let l18 = *arg0.add(56).cast::<usize>();
                                                                let len19 = l18;
                                                                let bytes19 = _rt::Vec::from_raw_parts(
                                                                    l17.cast(),
                                                                    len19,
                                                                    len19,
                                                                );
                                                                _rt::string_lift(bytes19)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    signature: match l20 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l21 = *arg0.add(64).cast::<*mut u8>();
                                                                let l22 = *arg0.add(68).cast::<usize>();
                                                                let len23 = l22;
                                                                let bytes23 = _rt::Vec::from_raw_parts(
                                                                    l21.cast(),
                                                                    len23,
                                                                    len23,
                                                                );
                                                                _rt::string_lift(bytes23)
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
                                }
                            };
                            V30::AllContent(e30)
                        }
                        1 => {
                            let e30 = {
                                let l24 = *arg0.add(4).cast::<*mut u8>();
                                let l25 = *arg0.add(8).cast::<usize>();
                                let len26 = l25;
                                let bytes26 = _rt::Vec::from_raw_parts(
                                    l24.cast(),
                                    len26,
                                    len26,
                                );
                                _rt::string_lift(bytes26)
                            };
                            V30::Message(e30)
                        }
                        n => {
                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                            let e30 = {
                                let l27 = *arg0.add(4).cast::<*mut u8>();
                                let l28 = *arg0.add(8).cast::<usize>();
                                let len29 = l28;
                                let bytes29 = _rt::Vec::from_raw_parts(
                                    l27.cast(),
                                    len29,
                                    len29,
                                );
                                _rt::string_lift(bytes29)
                            };
                            V30::Submit(e30)
                        }
                    };
                    let result31 = T::render(v30);
                    _rt::cabi_dealloc(arg0, 72, 4);
                    let ptr32 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result31 {
                        Ok(e) => {
                            *ptr32.add(0).cast::<u8>() = (0i32) as u8;
                            let vec33 = (e.into_bytes()).into_boxed_slice();
                            let ptr33 = vec33.as_ptr().cast::<u8>();
                            let len33 = vec33.len();
                            ::core::mem::forget(vec33);
                            *ptr32.add(8).cast::<usize>() = len33;
                            *ptr32.add(4).cast::<*mut u8>() = ptr33.cast_mut();
                        }
                        Err(e) => {
                            *ptr32.add(0).cast::<u8>() = (1i32) as u8;
                            let vec34 = (e.into_bytes()).into_boxed_slice();
                            let ptr34 = vec34.as_ptr().cast::<u8>();
                            let len34 = vec34.len();
                            ::core::mem::forget(vec34);
                            *ptr32.add(8).cast::<usize>() = len34;
                            *ptr32.add(4).cast::<*mut u8>() = ptr34.cast_mut();
                        }
                    };
                    ptr32
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
                    /// listen on all or given selectors
                    fn activate(selectors: Option<_rt::Vec<_rt::String>>);
                    /// renders the initial Web component with the given data
                    /// and the target template to use as top level entry point
                    fn render(ctx: Context) -> Result<_rt::String, _rt::String>;
                    /// Optionally customize the configuration of the templates used to render the component
                    fn customize(
                        templates: _rt::Vec<(_rt::String, _rt::String)>,
                    ) -> Result<(), _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_example_edwards_ui_wurbo_out_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "example:edwards-ui/wurbo-out@0.1.0#activate"] unsafe extern "C"
                        fn export_activate(arg0 : i32, arg1 : * mut u8, arg2 : usize,) {
                        $($path_to_types)*:: _export_activate_cabi::<$ty > (arg0, arg1,
                        arg2) } #[export_name =
                        "example:edwards-ui/wurbo-out@0.1.0#render"] unsafe extern "C" fn
                        export_render(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_render_cabi::<$ty > (arg0) }
                        #[export_name =
                        "cabi_post_example:edwards-ui/wurbo-out@0.1.0#render"] unsafe
                        extern "C" fn _post_return_render(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_render::<$ty > (arg0) }
                        #[export_name = "example:edwards-ui/wurbo-out@0.1.0#customize"]
                        unsafe extern "C" fn export_customize(arg0 : * mut u8, arg1 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_customize_cabi::<$ty > (arg0, arg1) } #[export_name =
                        "cabi_post_example:edwards-ui/wurbo-out@0.1.0#customize"] unsafe
                        extern "C" fn _post_return_customize(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_customize::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_example_edwards_ui_wurbo_out_0_1_0_cabi;
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
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
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
        exports::example::edwards_ui::wurbo_out::__export_example_edwards_ui_wurbo_out_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::example::edwards_ui::wurbo_out);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:example:edwards-ui@0.1.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 857] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xdb\x05\x01A\x02\x01\
A\x0a\x01B\x07\x01p}\x01j\x01\0\x01s\x01@\x01\x07message\0\0\x01\x04\0\x04sign\x01\
\x02\x01j\x01\x7f\x01s\x01@\x02\x07message\0\x09signature\0\0\x03\x04\0\x06verif\
y\x01\x04\x03\0(seed-keeper:edwards-wit/operations@0.1.0\x05\0\x01B\x0e\x01r\x02\
\x08selectors\x02tys\x04\0\x0elisten-details\x03\0\0\x01r\x01\x05titles\x04\0\x04\
page\x03\0\x02\x01r\x01\x0bplaceholders\x04\0\x05input\x03\0\x04\x01ks\x01r\x04\x05\
value\x06\x02id\x06\x07message\x06\x09signature\x06\x04\0\x06output\x03\0\x07\x01\
k\x08\x01r\x03\x04page\x03\x05input\x05\x06output\x09\x04\0\x07content\x03\0\x0a\
\x01q\x03\x0ball-content\x01\x0b\0\x07message\x01s\0\x06submit\x01s\0\x04\0\x07c\
ontext\x03\0\x0c\x03\0$example:edwards-ui/wurbo-types@0.1.0\x05\x01\x02\x03\0\x01\
\x0elisten-details\x01B\x04\x02\x03\x02\x01\x02\x04\0\x0elisten-details\x03\0\0\x01\
@\x01\x07details\x01\x01\0\x04\0\x10addeventlistener\x01\x02\x03\0!example:edwar\
ds-ui/wurbo-in@0.1.0\x05\x03\x02\x03\0\x01\x07context\x01B\x0e\x02\x03\x02\x01\x04\
\x04\0\x07context\x03\0\0\x01ps\x01k\x02\x01@\x01\x09selectors\x03\x01\0\x04\0\x08\
activate\x01\x04\x01j\x01s\x01s\x01@\x01\x03ctx\x01\0\x05\x04\0\x06render\x01\x06\
\x01o\x02ss\x01p\x07\x01j\0\x01s\x01@\x01\x09templates\x08\0\x09\x04\0\x09custom\
ize\x01\x0a\x04\0\"example:edwards-ui/wurbo-out@0.1.0\x05\x05\x04\0\x20example:e\
dwards-ui/example@0.1.0\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\
\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
