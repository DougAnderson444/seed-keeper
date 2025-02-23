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
        pub mod wurbo_out {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Context = super::super::super::example::edwards_ui::wurbo_types::Context;
            #[allow(unused_unsafe, clippy::all)]
            /// listen on all or given selectors
            pub fn activate(selectors: Option<&[_rt::String]>) {
                unsafe {
                    let mut cleanup_list = _rt::Vec::new();
                    let (result2_0, result2_1, result2_2) = match selectors {
                        Some(e) => {
                            let vec1 = e;
                            let len1 = vec1.len();
                            let layout1 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec1.len() * 8,
                                4,
                            );
                            let result1 = if layout1.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout1).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout1);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec1.into_iter().enumerate() {
                                let base = result1.add(i * 8);
                                {
                                    let vec0 = e;
                                    let ptr0 = vec0.as_ptr().cast::<u8>();
                                    let len0 = vec0.len();
                                    *base.add(4).cast::<usize>() = len0;
                                    *base.add(0).cast::<*mut u8>() = ptr0.cast_mut();
                                }
                            }
                            cleanup_list.extend_from_slice(&[(result1, layout1)]);
                            (1i32, result1, len1)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "example:edwards-ui/wurbo-out@0.1.0")]
                    extern "C" {
                        #[link_name = "activate"]
                        fn wit_import(_: i32, _: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(result2_0, result2_1, result2_2);
                    for (ptr, layout) in cleanup_list {
                        if layout.size() != 0 {
                            _rt::alloc::dealloc(ptr.cast(), layout);
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// renders the initial Web component with the given data
            /// and the target template to use as top level entry point
            pub fn render(ctx: &Context) -> Result<_rt::String, _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 72]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 72]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    use super::super::super::example::edwards_ui::wurbo_types::Context as V13;
                    match ctx {
                        V13::AllContent(e) => {
                            *ptr0.add(0).cast::<u8>() = (0i32) as u8;
                            let super::super::super::example::edwards_ui::wurbo_types::Content {
                                page: page1,
                                input: input1,
                                output: output1,
                            } = e;
                            let super::super::super::example::edwards_ui::wurbo_types::Page {
                                title: title2,
                            } = page1;
                            let vec3 = title2;
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            *ptr0.add(8).cast::<usize>() = len3;
                            *ptr0.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                            let super::super::super::example::edwards_ui::wurbo_types::Input {
                                placeholder: placeholder4,
                            } = input1;
                            let vec5 = placeholder4;
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            *ptr0.add(16).cast::<usize>() = len5;
                            *ptr0.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                            match output1 {
                                Some(e) => {
                                    *ptr0.add(20).cast::<u8>() = (1i32) as u8;
                                    let super::super::super::example::edwards_ui::wurbo_types::Output {
                                        value: value6,
                                        id: id6,
                                        message: message6,
                                        signature: signature6,
                                    } = e;
                                    match value6 {
                                        Some(e) => {
                                            *ptr0.add(24).cast::<u8>() = (1i32) as u8;
                                            let vec7 = e;
                                            let ptr7 = vec7.as_ptr().cast::<u8>();
                                            let len7 = vec7.len();
                                            *ptr0.add(32).cast::<usize>() = len7;
                                            *ptr0.add(28).cast::<*mut u8>() = ptr7.cast_mut();
                                        }
                                        None => {
                                            *ptr0.add(24).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    match id6 {
                                        Some(e) => {
                                            *ptr0.add(36).cast::<u8>() = (1i32) as u8;
                                            let vec8 = e;
                                            let ptr8 = vec8.as_ptr().cast::<u8>();
                                            let len8 = vec8.len();
                                            *ptr0.add(44).cast::<usize>() = len8;
                                            *ptr0.add(40).cast::<*mut u8>() = ptr8.cast_mut();
                                        }
                                        None => {
                                            *ptr0.add(36).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    match message6 {
                                        Some(e) => {
                                            *ptr0.add(48).cast::<u8>() = (1i32) as u8;
                                            let vec9 = e;
                                            let ptr9 = vec9.as_ptr().cast::<u8>();
                                            let len9 = vec9.len();
                                            *ptr0.add(56).cast::<usize>() = len9;
                                            *ptr0.add(52).cast::<*mut u8>() = ptr9.cast_mut();
                                        }
                                        None => {
                                            *ptr0.add(48).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    match signature6 {
                                        Some(e) => {
                                            *ptr0.add(60).cast::<u8>() = (1i32) as u8;
                                            let vec10 = e;
                                            let ptr10 = vec10.as_ptr().cast::<u8>();
                                            let len10 = vec10.len();
                                            *ptr0.add(68).cast::<usize>() = len10;
                                            *ptr0.add(64).cast::<*mut u8>() = ptr10.cast_mut();
                                        }
                                        None => {
                                            *ptr0.add(60).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                }
                                None => {
                                    *ptr0.add(20).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        V13::Message(e) => {
                            *ptr0.add(0).cast::<u8>() = (1i32) as u8;
                            let vec11 = e;
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            *ptr0.add(8).cast::<usize>() = len11;
                            *ptr0.add(4).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                        V13::Submit(e) => {
                            *ptr0.add(0).cast::<u8>() = (2i32) as u8;
                            let vec12 = e;
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            *ptr0.add(8).cast::<usize>() = len12;
                            *ptr0.add(4).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    }
                    let ptr14 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "example:edwards-ui/wurbo-out@0.1.0")]
                    extern "C" {
                        #[link_name = "render"]
                        fn wit_import(_: *mut u8, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0, ptr14);
                    let l15 = i32::from(*ptr14.add(0).cast::<u8>());
                    match l15 {
                        0 => {
                            let e = {
                                let l16 = *ptr14.add(4).cast::<*mut u8>();
                                let l17 = *ptr14.add(8).cast::<usize>();
                                let len18 = l17;
                                let bytes18 = _rt::Vec::from_raw_parts(
                                    l16.cast(),
                                    len18,
                                    len18,
                                );
                                _rt::string_lift(bytes18)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l19 = *ptr14.add(4).cast::<*mut u8>();
                                let l20 = *ptr14.add(8).cast::<usize>();
                                let len21 = l20;
                                let bytes21 = _rt::Vec::from_raw_parts(
                                    l19.cast(),
                                    len21,
                                    len21,
                                );
                                _rt::string_lift(bytes21)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Optionally customize the configuration of the templates used to render the component
            pub fn customize(
                templates: &[(_rt::String, _rt::String)],
            ) -> Result<(), _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let vec3 = templates;
                    let len3 = vec3.len();
                    let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec3.len() * 16,
                        4,
                    );
                    let result3 = if layout3.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout3);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec3.into_iter().enumerate() {
                        let base = result3.add(i * 16);
                        {
                            let (t0_0, t0_1) = e;
                            let vec1 = t0_0;
                            let ptr1 = vec1.as_ptr().cast::<u8>();
                            let len1 = vec1.len();
                            *base.add(4).cast::<usize>() = len1;
                            *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                            let vec2 = t0_1;
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            *base.add(12).cast::<usize>() = len2;
                            *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    }
                    let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "example:edwards-ui/wurbo-out@0.1.0")]
                    extern "C" {
                        #[link_name = "customize"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result3, len3, ptr4);
                    let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                    if layout3.size() != 0 {
                        _rt::alloc::dealloc(result3.cast(), layout3);
                    }
                    match l5 {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l6 = *ptr4.add(4).cast::<*mut u8>();
                                let l7 = *ptr4.add(8).cast::<usize>();
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
        }
    }
}
#[allow(dead_code)]
pub mod seed_keeper {
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
        pub mod wurbo_out {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Context = super::super::super::seed_keeper::wit_ui::wurbo_types::Context;
            #[allow(unused_unsafe, clippy::all)]
            /// renders the initial Web component with the given data
            /// and the target template to use as top level entry point
            pub fn render(ctx: &Context) -> Result<_rt::String, _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    use super::super::super::seed_keeper::wit_ui::wurbo_types::Context as V14;
                    let (
                        result15_0,
                        result15_1,
                        result15_2,
                        result15_3,
                        result15_4,
                        result15_5,
                        result15_6,
                        result15_7,
                        result15_8,
                        result15_9,
                        result15_10,
                        result15_11,
                        result15_12,
                    ) = match ctx {
                        V14::AllContent(e) => {
                            let super::super::super::seed_keeper::wit_ui::wurbo_types::Content {
                                page: page0,
                                input: input0,
                                load: load0,
                            } = e;
                            let (result3_0, result3_1, result3_2) = match page0 {
                                Some(e) => {
                                    let super::super::super::seed_keeper::wit_ui::wurbo_types::Page {
                                        title: title1,
                                    } = e;
                                    let vec2 = title1;
                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                    let len2 = vec2.len();
                                    (1i32, ptr2.cast_mut(), len2)
                                }
                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                            };
                            let (
                                result8_0,
                                result8_1,
                                result8_2,
                                result8_3,
                                result8_4,
                                result8_5,
                            ) = match input0 {
                                Some(e) => {
                                    let super::super::super::seed_keeper::wit_ui::wurbo_types::Input {
                                        placeholder: placeholder4,
                                        encrypted_seed: encrypted_seed4,
                                    } = e;
                                    let vec5 = placeholder4;
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    let (result7_0, result7_1, result7_2) = match encrypted_seed4 {
                                        Some(e) => {
                                            let vec6 = e;
                                            let ptr6 = vec6.as_ptr().cast::<u8>();
                                            let len6 = vec6.len();
                                            (1i32, ptr6.cast_mut(), len6)
                                        }
                                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                                    };
                                    (
                                        1i32,
                                        ptr5.cast_mut(),
                                        len5,
                                        result7_0,
                                        result7_1,
                                        result7_2,
                                    )
                                }
                                None => {
                                    (
                                        0i32,
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                        ::core::ptr::null_mut(),
                                        0usize,
                                    )
                                }
                            };
                            let (result10_0, result10_1, result10_2) = match load0 {
                                Some(e) => {
                                    let vec9 = e;
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    (1i32, ptr9.cast_mut(), len9)
                                }
                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                            };
                            (
                                0i32,
                                result3_0 as *mut u8,
                                result3_1,
                                result3_2,
                                result8_0,
                                result8_1,
                                result8_2,
                                result8_3,
                                result8_4,
                                result8_5,
                                result10_0,
                                result10_1,
                                result10_2,
                            )
                        }
                        V14::Username(e) => {
                            let vec11 = e;
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            (
                                1i32,
                                ptr11.cast_mut(),
                                len11 as *mut u8,
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                            )
                        }
                        V14::Password(e) => {
                            let vec12 = e;
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            (
                                2i32,
                                ptr12.cast_mut(),
                                len12 as *mut u8,
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                            )
                        }
                        V14::Encrypted(e) => {
                            let vec13 = e;
                            let ptr13 = vec13.as_ptr().cast::<u8>();
                            let len13 = vec13.len();
                            (
                                3i32,
                                ptr13.cast_mut(),
                                len13 as *mut u8,
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                            )
                        }
                        V14::Submit => {
                            (
                                4i32,
                                ::core::ptr::null_mut(),
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                                ::core::ptr::null_mut(),
                                0usize,
                            )
                        }
                    };
                    let ptr16 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wit-ui/wurbo-out@0.1.0")]
                    extern "C" {
                        #[link_name = "render"]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
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
                        _: i32,
                        _: *mut u8,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: i32,
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
                        result15_0,
                        result15_1,
                        result15_2,
                        result15_3,
                        result15_4,
                        result15_5,
                        result15_6,
                        result15_7,
                        result15_8,
                        result15_9,
                        result15_10,
                        result15_11,
                        result15_12,
                        ptr16,
                    );
                    let l17 = i32::from(*ptr16.add(0).cast::<u8>());
                    match l17 {
                        0 => {
                            let e = {
                                let l18 = *ptr16.add(4).cast::<*mut u8>();
                                let l19 = *ptr16.add(8).cast::<usize>();
                                let len20 = l19;
                                let bytes20 = _rt::Vec::from_raw_parts(
                                    l18.cast(),
                                    len20,
                                    len20,
                                );
                                _rt::string_lift(bytes20)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l21 = *ptr16.add(4).cast::<*mut u8>();
                                let l22 = *ptr16.add(8).cast::<usize>();
                                let len23 = l22;
                                let bytes23 = _rt::Vec::from_raw_parts(
                                    l21.cast(),
                                    len23,
                                    len23,
                                );
                                _rt::string_lift(bytes23)
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// listen on all or given selectors
            pub fn activate(selectors: Option<&[_rt::String]>) {
                unsafe {
                    let mut cleanup_list = _rt::Vec::new();
                    let (result2_0, result2_1, result2_2) = match selectors {
                        Some(e) => {
                            let vec1 = e;
                            let len1 = vec1.len();
                            let layout1 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec1.len() * 8,
                                4,
                            );
                            let result1 = if layout1.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout1).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout1);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec1.into_iter().enumerate() {
                                let base = result1.add(i * 8);
                                {
                                    let vec0 = e;
                                    let ptr0 = vec0.as_ptr().cast::<u8>();
                                    let len0 = vec0.len();
                                    *base.add(4).cast::<usize>() = len0;
                                    *base.add(0).cast::<*mut u8>() = ptr0.cast_mut();
                                }
                            }
                            cleanup_list.extend_from_slice(&[(result1, layout1)]);
                            (1i32, result1, len1)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wit-ui/wurbo-out@0.1.0")]
                    extern "C" {
                        #[link_name = "activate"]
                        fn wit_import(_: i32, _: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(result2_0, result2_1, result2_2);
                    for (ptr, layout) in cleanup_list {
                        if layout.size() != 0 {
                            _rt::alloc::dealloc(ptr.cast(), layout);
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Optionally customize the configuration of the templates used to render the component
            pub fn customize(
                templates: &[(_rt::String, _rt::String)],
            ) -> Result<(), _rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let vec3 = templates;
                    let len3 = vec3.len();
                    let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec3.len() * 16,
                        4,
                    );
                    let result3 = if layout3.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout3);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec3.into_iter().enumerate() {
                        let base = result3.add(i * 16);
                        {
                            let (t0_0, t0_1) = e;
                            let vec1 = t0_0;
                            let ptr1 = vec1.as_ptr().cast::<u8>();
                            let len1 = vec1.len();
                            *base.add(4).cast::<usize>() = len1;
                            *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                            let vec2 = t0_1;
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            *base.add(12).cast::<usize>() = len2;
                            *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                    }
                    let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "seed-keeper:wit-ui/wurbo-out@0.1.0")]
                    extern "C" {
                        #[link_name = "customize"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result3, len3, ptr4);
                    let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                    if layout3.size() != 0 {
                        _rt::alloc::dealloc(result3.cast(), layout3);
                    }
                    match l5 {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l6 = *ptr4.add(4).cast::<*mut u8>();
                                let l7 = *ptr4.add(8).cast::<usize>();
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
        }
    }
}
#[allow(dead_code)]
pub mod wallet {
    #[allow(dead_code)]
    pub mod aggregate_wit_ui {
        #[allow(dead_code, clippy::all)]
        pub mod wurbo_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type EdwardsContext = super::super::super::example::edwards_ui::wurbo_out::Context;
            pub type SeedContext = super::super::super::seed_keeper::wit_ui::wurbo_out::Context;
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
            pub struct App {
                pub title: _rt::String,
            }
            impl ::core::fmt::Debug for App {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("App").field("title", &self.title).finish()
                }
            }
            /// Content record for the initial content of the entire app
            #[derive(Clone)]
            pub struct Content {
                /// pass in props like title, etc.
                pub app: App,
                /// optionally pass in an encrypted seed to load
                pub seed_ui: SeedContext,
                /// optionally pass in a message to sign or verify
                pub edwards_ui: EdwardsContext,
            }
            impl ::core::fmt::Debug for Content {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Content")
                        .field("app", &self.app)
                        .field("seed-ui", &self.seed_ui)
                        .field("edwards-ui", &self.edwards_ui)
                        .finish()
                }
            }
            /// Context variants
            #[derive(Clone)]
            pub enum Context {
                AllContent(Content),
                Seed(SeedContext),
                Edwards(EdwardsContext),
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
                        Context::Seed(e) => {
                            f.debug_tuple("Context::Seed").field(e).finish()
                        }
                        Context::Edwards(e) => {
                            f.debug_tuple("Context::Edwards").field(e).finish()
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
            pub type ListenDetails = super::super::super::wallet::aggregate_wit_ui::wurbo_types::ListenDetails;
            #[allow(unused_unsafe, clippy::all)]
            /// Add an event listener to the given element
            pub fn addeventlistener(details: &ListenDetails) {
                unsafe {
                    let super::super::super::wallet::aggregate_wit_ui::wurbo_types::ListenDetails {
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
                    #[link(
                        wasm_import_module = "wallet:aggregate-wit-ui/wurbo-in@0.1.0"
                    )]
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
pub mod exports {
    #[allow(dead_code)]
    pub mod wallet {
        #[allow(dead_code)]
        pub mod aggregate_wit_ui {
            #[allow(dead_code, clippy::all)]
            pub mod wurbo_out {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Context = super::super::super::super::wallet::aggregate_wit_ui::wurbo_types::Context;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_render_cabi<T: Guest>(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    use super::super::super::super::wallet::aggregate_wit_ui::wurbo_types::Context as V120;
                    let v120 = match l0 {
                        0 => {
                            let e120 = {
                                let l1 = *arg0.add(4).cast::<*mut u8>();
                                let l2 = *arg0.add(8).cast::<usize>();
                                let len3 = l2;
                                let bytes3 = _rt::Vec::from_raw_parts(
                                    l1.cast(),
                                    len3,
                                    len3,
                                );
                                let l4 = i32::from(*arg0.add(12).cast::<u8>());
                                use super::super::super::super::seed_keeper::wit_ui::wurbo_types::Context as V30;
                                let v30 = match l4 {
                                    0 => {
                                        let e30 = {
                                            let l5 = i32::from(*arg0.add(16).cast::<u8>());
                                            let l9 = i32::from(*arg0.add(28).cast::<u8>());
                                            let l17 = i32::from(*arg0.add(52).cast::<u8>());
                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Content {
                                                page: match l5 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l6 = *arg0.add(20).cast::<*mut u8>();
                                                            let l7 = *arg0.add(24).cast::<usize>();
                                                            let len8 = l7;
                                                            let bytes8 = _rt::Vec::from_raw_parts(
                                                                l6.cast(),
                                                                len8,
                                                                len8,
                                                            );
                                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Page {
                                                                title: _rt::string_lift(bytes8),
                                                            }
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                input: match l9 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l10 = *arg0.add(32).cast::<*mut u8>();
                                                            let l11 = *arg0.add(36).cast::<usize>();
                                                            let len12 = l11;
                                                            let bytes12 = _rt::Vec::from_raw_parts(
                                                                l10.cast(),
                                                                len12,
                                                                len12,
                                                            );
                                                            let l13 = i32::from(*arg0.add(40).cast::<u8>());
                                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Input {
                                                                placeholder: _rt::string_lift(bytes12),
                                                                encrypted_seed: match l13 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l14 = *arg0.add(44).cast::<*mut u8>();
                                                                            let l15 = *arg0.add(48).cast::<usize>();
                                                                            let len16 = l15;
                                                                            let bytes16 = _rt::Vec::from_raw_parts(
                                                                                l14.cast(),
                                                                                len16,
                                                                                len16,
                                                                            );
                                                                            _rt::string_lift(bytes16)
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
                                                load: match l17 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l18 = *arg0.add(56).cast::<*mut u8>();
                                                            let l19 = *arg0.add(60).cast::<usize>();
                                                            let len20 = l19;
                                                            let bytes20 = _rt::Vec::from_raw_parts(
                                                                l18.cast(),
                                                                len20,
                                                                len20,
                                                            );
                                                            _rt::string_lift(bytes20)
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
                                            let l21 = *arg0.add(16).cast::<*mut u8>();
                                            let l22 = *arg0.add(20).cast::<usize>();
                                            let len23 = l22;
                                            let bytes23 = _rt::Vec::from_raw_parts(
                                                l21.cast(),
                                                len23,
                                                len23,
                                            );
                                            _rt::string_lift(bytes23)
                                        };
                                        V30::Username(e30)
                                    }
                                    2 => {
                                        let e30 = {
                                            let l24 = *arg0.add(16).cast::<*mut u8>();
                                            let l25 = *arg0.add(20).cast::<usize>();
                                            let len26 = l25;
                                            let bytes26 = _rt::Vec::from_raw_parts(
                                                l24.cast(),
                                                len26,
                                                len26,
                                            );
                                            _rt::string_lift(bytes26)
                                        };
                                        V30::Password(e30)
                                    }
                                    3 => {
                                        let e30 = {
                                            let l27 = *arg0.add(16).cast::<*mut u8>();
                                            let l28 = *arg0.add(20).cast::<usize>();
                                            let len29 = l28;
                                            let bytes29 = _rt::Vec::from_raw_parts(
                                                l27.cast(),
                                                len29,
                                                len29,
                                            );
                                            _rt::string_lift(bytes29)
                                        };
                                        V30::Encrypted(e30)
                                    }
                                    n => {
                                        debug_assert_eq!(n, 4, "invalid enum discriminant");
                                        V30::Submit
                                    }
                                };
                                let l31 = i32::from(*arg0.add(64).cast::<u8>());
                                use super::super::super::super::example::edwards_ui::wurbo_types::Context as V61;
                                let v61 = match l31 {
                                    0 => {
                                        let e61 = {
                                            let l32 = *arg0.add(68).cast::<*mut u8>();
                                            let l33 = *arg0.add(72).cast::<usize>();
                                            let len34 = l33;
                                            let bytes34 = _rt::Vec::from_raw_parts(
                                                l32.cast(),
                                                len34,
                                                len34,
                                            );
                                            let l35 = *arg0.add(76).cast::<*mut u8>();
                                            let l36 = *arg0.add(80).cast::<usize>();
                                            let len37 = l36;
                                            let bytes37 = _rt::Vec::from_raw_parts(
                                                l35.cast(),
                                                len37,
                                                len37,
                                            );
                                            let l38 = i32::from(*arg0.add(84).cast::<u8>());
                                            super::super::super::super::example::edwards_ui::wurbo_types::Content {
                                                page: super::super::super::super::example::edwards_ui::wurbo_types::Page {
                                                    title: _rt::string_lift(bytes34),
                                                },
                                                input: super::super::super::super::example::edwards_ui::wurbo_types::Input {
                                                    placeholder: _rt::string_lift(bytes37),
                                                },
                                                output: match l38 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l39 = i32::from(*arg0.add(88).cast::<u8>());
                                                            let l43 = i32::from(*arg0.add(100).cast::<u8>());
                                                            let l47 = i32::from(*arg0.add(112).cast::<u8>());
                                                            let l51 = i32::from(*arg0.add(124).cast::<u8>());
                                                            super::super::super::super::example::edwards_ui::wurbo_types::Output {
                                                                value: match l39 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l40 = *arg0.add(92).cast::<*mut u8>();
                                                                            let l41 = *arg0.add(96).cast::<usize>();
                                                                            let len42 = l41;
                                                                            let bytes42 = _rt::Vec::from_raw_parts(
                                                                                l40.cast(),
                                                                                len42,
                                                                                len42,
                                                                            );
                                                                            _rt::string_lift(bytes42)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                id: match l43 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l44 = *arg0.add(104).cast::<*mut u8>();
                                                                            let l45 = *arg0.add(108).cast::<usize>();
                                                                            let len46 = l45;
                                                                            let bytes46 = _rt::Vec::from_raw_parts(
                                                                                l44.cast(),
                                                                                len46,
                                                                                len46,
                                                                            );
                                                                            _rt::string_lift(bytes46)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                message: match l47 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l48 = *arg0.add(116).cast::<*mut u8>();
                                                                            let l49 = *arg0.add(120).cast::<usize>();
                                                                            let len50 = l49;
                                                                            let bytes50 = _rt::Vec::from_raw_parts(
                                                                                l48.cast(),
                                                                                len50,
                                                                                len50,
                                                                            );
                                                                            _rt::string_lift(bytes50)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                signature: match l51 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l52 = *arg0.add(128).cast::<*mut u8>();
                                                                            let l53 = *arg0.add(132).cast::<usize>();
                                                                            let len54 = l53;
                                                                            let bytes54 = _rt::Vec::from_raw_parts(
                                                                                l52.cast(),
                                                                                len54,
                                                                                len54,
                                                                            );
                                                                            _rt::string_lift(bytes54)
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
                                        V61::AllContent(e61)
                                    }
                                    1 => {
                                        let e61 = {
                                            let l55 = *arg0.add(68).cast::<*mut u8>();
                                            let l56 = *arg0.add(72).cast::<usize>();
                                            let len57 = l56;
                                            let bytes57 = _rt::Vec::from_raw_parts(
                                                l55.cast(),
                                                len57,
                                                len57,
                                            );
                                            _rt::string_lift(bytes57)
                                        };
                                        V61::Message(e61)
                                    }
                                    n => {
                                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                                        let e61 = {
                                            let l58 = *arg0.add(68).cast::<*mut u8>();
                                            let l59 = *arg0.add(72).cast::<usize>();
                                            let len60 = l59;
                                            let bytes60 = _rt::Vec::from_raw_parts(
                                                l58.cast(),
                                                len60,
                                                len60,
                                            );
                                            _rt::string_lift(bytes60)
                                        };
                                        V61::Submit(e61)
                                    }
                                };
                                super::super::super::super::wallet::aggregate_wit_ui::wurbo_types::Content {
                                    app: super::super::super::super::wallet::aggregate_wit_ui::wurbo_types::App {
                                        title: _rt::string_lift(bytes3),
                                    },
                                    seed_ui: v30,
                                    edwards_ui: v61,
                                }
                            };
                            V120::AllContent(e120)
                        }
                        1 => {
                            let e120 = {
                                let l62 = i32::from(*arg0.add(4).cast::<u8>());
                                use super::super::super::super::seed_keeper::wit_ui::wurbo_types::Context as V88;
                                let v88 = match l62 {
                                    0 => {
                                        let e88 = {
                                            let l63 = i32::from(*arg0.add(8).cast::<u8>());
                                            let l67 = i32::from(*arg0.add(20).cast::<u8>());
                                            let l75 = i32::from(*arg0.add(44).cast::<u8>());
                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Content {
                                                page: match l63 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l64 = *arg0.add(12).cast::<*mut u8>();
                                                            let l65 = *arg0.add(16).cast::<usize>();
                                                            let len66 = l65;
                                                            let bytes66 = _rt::Vec::from_raw_parts(
                                                                l64.cast(),
                                                                len66,
                                                                len66,
                                                            );
                                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Page {
                                                                title: _rt::string_lift(bytes66),
                                                            }
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                input: match l67 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l68 = *arg0.add(24).cast::<*mut u8>();
                                                            let l69 = *arg0.add(28).cast::<usize>();
                                                            let len70 = l69;
                                                            let bytes70 = _rt::Vec::from_raw_parts(
                                                                l68.cast(),
                                                                len70,
                                                                len70,
                                                            );
                                                            let l71 = i32::from(*arg0.add(32).cast::<u8>());
                                                            super::super::super::super::seed_keeper::wit_ui::wurbo_types::Input {
                                                                placeholder: _rt::string_lift(bytes70),
                                                                encrypted_seed: match l71 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l72 = *arg0.add(36).cast::<*mut u8>();
                                                                            let l73 = *arg0.add(40).cast::<usize>();
                                                                            let len74 = l73;
                                                                            let bytes74 = _rt::Vec::from_raw_parts(
                                                                                l72.cast(),
                                                                                len74,
                                                                                len74,
                                                                            );
                                                                            _rt::string_lift(bytes74)
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
                                                load: match l75 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l76 = *arg0.add(48).cast::<*mut u8>();
                                                            let l77 = *arg0.add(52).cast::<usize>();
                                                            let len78 = l77;
                                                            let bytes78 = _rt::Vec::from_raw_parts(
                                                                l76.cast(),
                                                                len78,
                                                                len78,
                                                            );
                                                            _rt::string_lift(bytes78)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V88::AllContent(e88)
                                    }
                                    1 => {
                                        let e88 = {
                                            let l79 = *arg0.add(8).cast::<*mut u8>();
                                            let l80 = *arg0.add(12).cast::<usize>();
                                            let len81 = l80;
                                            let bytes81 = _rt::Vec::from_raw_parts(
                                                l79.cast(),
                                                len81,
                                                len81,
                                            );
                                            _rt::string_lift(bytes81)
                                        };
                                        V88::Username(e88)
                                    }
                                    2 => {
                                        let e88 = {
                                            let l82 = *arg0.add(8).cast::<*mut u8>();
                                            let l83 = *arg0.add(12).cast::<usize>();
                                            let len84 = l83;
                                            let bytes84 = _rt::Vec::from_raw_parts(
                                                l82.cast(),
                                                len84,
                                                len84,
                                            );
                                            _rt::string_lift(bytes84)
                                        };
                                        V88::Password(e88)
                                    }
                                    3 => {
                                        let e88 = {
                                            let l85 = *arg0.add(8).cast::<*mut u8>();
                                            let l86 = *arg0.add(12).cast::<usize>();
                                            let len87 = l86;
                                            let bytes87 = _rt::Vec::from_raw_parts(
                                                l85.cast(),
                                                len87,
                                                len87,
                                            );
                                            _rt::string_lift(bytes87)
                                        };
                                        V88::Encrypted(e88)
                                    }
                                    n => {
                                        debug_assert_eq!(n, 4, "invalid enum discriminant");
                                        V88::Submit
                                    }
                                };
                                v88
                            };
                            V120::Seed(e120)
                        }
                        n => {
                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                            let e120 = {
                                let l89 = i32::from(*arg0.add(4).cast::<u8>());
                                use super::super::super::super::example::edwards_ui::wurbo_types::Context as V119;
                                let v119 = match l89 {
                                    0 => {
                                        let e119 = {
                                            let l90 = *arg0.add(8).cast::<*mut u8>();
                                            let l91 = *arg0.add(12).cast::<usize>();
                                            let len92 = l91;
                                            let bytes92 = _rt::Vec::from_raw_parts(
                                                l90.cast(),
                                                len92,
                                                len92,
                                            );
                                            let l93 = *arg0.add(16).cast::<*mut u8>();
                                            let l94 = *arg0.add(20).cast::<usize>();
                                            let len95 = l94;
                                            let bytes95 = _rt::Vec::from_raw_parts(
                                                l93.cast(),
                                                len95,
                                                len95,
                                            );
                                            let l96 = i32::from(*arg0.add(24).cast::<u8>());
                                            super::super::super::super::example::edwards_ui::wurbo_types::Content {
                                                page: super::super::super::super::example::edwards_ui::wurbo_types::Page {
                                                    title: _rt::string_lift(bytes92),
                                                },
                                                input: super::super::super::super::example::edwards_ui::wurbo_types::Input {
                                                    placeholder: _rt::string_lift(bytes95),
                                                },
                                                output: match l96 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l97 = i32::from(*arg0.add(28).cast::<u8>());
                                                            let l101 = i32::from(*arg0.add(40).cast::<u8>());
                                                            let l105 = i32::from(*arg0.add(52).cast::<u8>());
                                                            let l109 = i32::from(*arg0.add(64).cast::<u8>());
                                                            super::super::super::super::example::edwards_ui::wurbo_types::Output {
                                                                value: match l97 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l98 = *arg0.add(32).cast::<*mut u8>();
                                                                            let l99 = *arg0.add(36).cast::<usize>();
                                                                            let len100 = l99;
                                                                            let bytes100 = _rt::Vec::from_raw_parts(
                                                                                l98.cast(),
                                                                                len100,
                                                                                len100,
                                                                            );
                                                                            _rt::string_lift(bytes100)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                id: match l101 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l102 = *arg0.add(44).cast::<*mut u8>();
                                                                            let l103 = *arg0.add(48).cast::<usize>();
                                                                            let len104 = l103;
                                                                            let bytes104 = _rt::Vec::from_raw_parts(
                                                                                l102.cast(),
                                                                                len104,
                                                                                len104,
                                                                            );
                                                                            _rt::string_lift(bytes104)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                message: match l105 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l106 = *arg0.add(56).cast::<*mut u8>();
                                                                            let l107 = *arg0.add(60).cast::<usize>();
                                                                            let len108 = l107;
                                                                            let bytes108 = _rt::Vec::from_raw_parts(
                                                                                l106.cast(),
                                                                                len108,
                                                                                len108,
                                                                            );
                                                                            _rt::string_lift(bytes108)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                signature: match l109 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l110 = *arg0.add(68).cast::<*mut u8>();
                                                                            let l111 = *arg0.add(72).cast::<usize>();
                                                                            let len112 = l111;
                                                                            let bytes112 = _rt::Vec::from_raw_parts(
                                                                                l110.cast(),
                                                                                len112,
                                                                                len112,
                                                                            );
                                                                            _rt::string_lift(bytes112)
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
                                        V119::AllContent(e119)
                                    }
                                    1 => {
                                        let e119 = {
                                            let l113 = *arg0.add(8).cast::<*mut u8>();
                                            let l114 = *arg0.add(12).cast::<usize>();
                                            let len115 = l114;
                                            let bytes115 = _rt::Vec::from_raw_parts(
                                                l113.cast(),
                                                len115,
                                                len115,
                                            );
                                            _rt::string_lift(bytes115)
                                        };
                                        V119::Message(e119)
                                    }
                                    n => {
                                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                                        let e119 = {
                                            let l116 = *arg0.add(8).cast::<*mut u8>();
                                            let l117 = *arg0.add(12).cast::<usize>();
                                            let len118 = l117;
                                            let bytes118 = _rt::Vec::from_raw_parts(
                                                l116.cast(),
                                                len118,
                                                len118,
                                            );
                                            _rt::string_lift(bytes118)
                                        };
                                        V119::Submit(e119)
                                    }
                                };
                                v119
                            };
                            V120::Edwards(e120)
                        }
                    };
                    let result121 = T::render(v120);
                    _rt::cabi_dealloc(arg0, 136, 4);
                    let ptr122 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result121 {
                        Ok(e) => {
                            *ptr122.add(0).cast::<u8>() = (0i32) as u8;
                            let vec123 = (e.into_bytes()).into_boxed_slice();
                            let ptr123 = vec123.as_ptr().cast::<u8>();
                            let len123 = vec123.len();
                            ::core::mem::forget(vec123);
                            *ptr122.add(8).cast::<usize>() = len123;
                            *ptr122.add(4).cast::<*mut u8>() = ptr123.cast_mut();
                        }
                        Err(e) => {
                            *ptr122.add(0).cast::<u8>() = (1i32) as u8;
                            let vec124 = (e.into_bytes()).into_boxed_slice();
                            let ptr124 = vec124.as_ptr().cast::<u8>();
                            let len124 = vec124.len();
                            ::core::mem::forget(vec124);
                            *ptr122.add(8).cast::<usize>() = len124;
                            *ptr122.add(4).cast::<*mut u8>() = ptr124.cast_mut();
                        }
                    };
                    ptr122
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
                pub trait Guest {
                    /// renders the initial Web component with the given data
                    fn render(ctx: Context) -> Result<_rt::String, _rt::String>;
                    /// listen on all or given selectors
                    fn activate(selectors: Option<_rt::Vec<_rt::String>>);
                }
                #[doc(hidden)]
                macro_rules! __export_wallet_aggregate_wit_ui_wurbo_out_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "wallet:aggregate-wit-ui/wurbo-out@0.1.0#render"] unsafe extern
                        "C" fn export_render(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_render_cabi::<$ty > (arg0) }
                        #[export_name =
                        "cabi_post_wallet:aggregate-wit-ui/wurbo-out@0.1.0#render"]
                        unsafe extern "C" fn _post_return_render(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_render::<$ty > (arg0) }
                        #[export_name =
                        "wallet:aggregate-wit-ui/wurbo-out@0.1.0#activate"] unsafe extern
                        "C" fn export_activate(arg0 : i32, arg1 : * mut u8, arg2 :
                        usize,) { $($path_to_types)*:: _export_activate_cabi::<$ty >
                        (arg0, arg1, arg2) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wallet_aggregate_wit_ui_wurbo_out_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod aggregation {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_activates_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::activates(
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
                pub trait Guest {
                    /// calls activate on on the child components
                    fn activates(selectors: Option<_rt::Vec<_rt::String>>);
                }
                #[doc(hidden)]
                macro_rules! __export_wallet_aggregate_wit_ui_aggregation_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "wallet:aggregate-wit-ui/aggregation@0.1.0#activates"] unsafe
                        extern "C" fn export_activates(arg0 : i32, arg1 : * mut u8, arg2
                        : usize,) { $($path_to_types)*:: _export_activates_cabi::<$ty >
                        (arg0, arg1, arg2) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wallet_aggregate_wit_ui_aggregation_0_1_0_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::alloc;
    pub use alloc_crate::vec::Vec;
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
macro_rules! __export_agg_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::wallet::aggregate_wit_ui::wurbo_out::__export_wallet_aggregate_wit_ui_wurbo_out_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::wallet::aggregate_wit_ui::wurbo_out); $($path_to_types_root)*::
        exports::wallet::aggregate_wit_ui::aggregation::__export_wallet_aggregate_wit_ui_aggregation_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::wallet::aggregate_wit_ui::aggregation);
    };
}
#[doc(inline)]
pub(crate) use __export_agg_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:wallet:aggregate-wit-ui@0.1.0:agg:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1690] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa0\x0c\x01A\x02\x01\
A\x16\x01B\x0e\x01r\x02\x08selectors\x02tys\x04\0\x0elisten-details\x03\0\0\x01r\
\x01\x05titles\x04\0\x04page\x03\0\x02\x01r\x01\x0bplaceholders\x04\0\x05input\x03\
\0\x04\x01ks\x01r\x04\x05value\x06\x02id\x06\x07message\x06\x09signature\x06\x04\
\0\x06output\x03\0\x07\x01k\x08\x01r\x03\x04page\x03\x05input\x05\x06output\x09\x04\
\0\x07content\x03\0\x0a\x01q\x03\x0ball-content\x01\x0b\0\x07message\x01s\0\x06s\
ubmit\x01s\0\x04\0\x07context\x03\0\x0c\x03\0$example:edwards-ui/wurbo-types@0.1\
.0\x05\0\x02\x03\0\0\x07context\x01B\x0e\x02\x03\x02\x01\x01\x04\0\x07context\x03\
\0\0\x01ps\x01k\x02\x01@\x01\x09selectors\x03\x01\0\x04\0\x08activate\x01\x04\x01\
j\x01s\x01s\x01@\x01\x03ctx\x01\0\x05\x04\0\x06render\x01\x06\x01o\x02ss\x01p\x07\
\x01j\0\x01s\x01@\x01\x09templates\x08\0\x09\x04\0\x09customize\x01\x0a\x03\0\"e\
xample:edwards-ui/wurbo-out@0.1.0\x05\x02\x01B\x0f\x01s\x04\0\x09encrypted\x03\0\
\0\x01r\x02\x08selectors\x02tys\x04\0\x0elisten-details\x03\0\x02\x01r\x01\x05ti\
tles\x04\0\x04page\x03\0\x04\x01ks\x01r\x02\x0bplaceholders\x0eencrypted-seed\x06\
\x04\0\x05input\x03\0\x07\x01k\x05\x01k\x08\x01r\x03\x04page\x09\x05input\x0a\x04\
load\x06\x04\0\x07content\x03\0\x0b\x01q\x05\x0ball-content\x01\x0c\0\x08usernam\
e\x01s\0\x08password\x01s\0\x09encrypted\x01\x01\0\x06submit\0\0\x04\0\x07contex\
t\x03\0\x0d\x03\0$seed-keeper:wit-ui/wurbo-types@0.1.0\x05\x03\x02\x03\0\x02\x07\
context\x01B\x0e\x02\x03\x02\x01\x04\x04\0\x07context\x03\0\0\x01j\x01s\x01s\x01\
@\x01\x03ctx\x01\0\x02\x04\0\x06render\x01\x03\x01ps\x01k\x04\x01@\x01\x09select\
ors\x05\x01\0\x04\0\x08activate\x01\x06\x01o\x02ss\x01p\x07\x01j\0\x01s\x01@\x01\
\x09templates\x08\0\x09\x04\0\x09customize\x01\x0a\x03\0\"seed-keeper:wit-ui/wur\
bo-out@0.1.0\x05\x05\x02\x03\0\x01\x07context\x02\x03\0\x03\x07context\x01B\x0c\x02\
\x03\x02\x01\x06\x04\0\x0fedwards-context\x03\0\0\x02\x03\x02\x01\x07\x04\0\x0cs\
eed-context\x03\0\x02\x01r\x02\x08selectors\x02tys\x04\0\x0elisten-details\x03\0\
\x04\x01r\x01\x05titles\x04\0\x03app\x03\0\x06\x01r\x03\x03app\x07\x07seed-ui\x03\
\x0aedwards-ui\x01\x04\0\x07content\x03\0\x08\x01q\x03\x0ball-content\x01\x09\0\x04\
seed\x01\x03\0\x07edwards\x01\x01\0\x04\0\x07context\x03\0\x0a\x03\0)wallet:aggr\
egate-wit-ui/wurbo-types@0.1.0\x05\x08\x02\x03\0\x04\x0elisten-details\x01B\x04\x02\
\x03\x02\x01\x09\x04\0\x0elisten-details\x03\0\0\x01@\x01\x07details\x01\x01\0\x04\
\0\x10addeventlistener\x01\x02\x03\0&wallet:aggregate-wit-ui/wurbo-in@0.1.0\x05\x0a\
\x02\x03\0\x04\x07context\x01B\x09\x02\x03\x02\x01\x0b\x04\0\x07context\x03\0\0\x01\
j\x01s\x01s\x01@\x01\x03ctx\x01\0\x02\x04\0\x06render\x01\x03\x01ps\x01k\x04\x01\
@\x01\x09selectors\x05\x01\0\x04\0\x08activate\x01\x06\x04\0'wallet:aggregate-wi\
t-ui/wurbo-out@0.1.0\x05\x0c\x01B\x04\x01ps\x01k\0\x01@\x01\x09selectors\x01\x01\
\0\x04\0\x09activates\x01\x02\x04\0)wallet:aggregate-wit-ui/aggregation@0.1.0\x05\
\x0d\x04\0!wallet:aggregate-wit-ui/agg@0.1.0\x04\0\x0b\x09\x01\0\x03agg\x03\0\0\0\
G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindge\
n-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
