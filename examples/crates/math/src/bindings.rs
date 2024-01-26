// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod component {
    pub mod math {
      
      #[allow(clippy::all)]
      pub mod addition {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "component:math/addition@0.1.0#add"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add(arg0: i32,arg1: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::add(arg0, arg1);
            wit_bindgen::rt::as_i32(result0)
          }
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn add(x: i32,y: i32,) -> i32;
        }
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:example"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 218] = [3, 0, 7, 101, 120, 97, 109, 112, 108, 101, 0, 97, 115, 109, 13, 0, 1, 0, 7, 93, 1, 65, 2, 1, 65, 2, 1, 66, 2, 1, 64, 2, 1, 120, 122, 1, 121, 122, 0, 122, 4, 0, 3, 97, 100, 100, 1, 0, 4, 1, 29, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 109, 97, 116, 104, 47, 97, 100, 100, 105, 116, 105, 111, 110, 64, 48, 46, 49, 46, 48, 5, 0, 4, 1, 26, 101, 120, 97, 109, 112, 108, 101, 58, 109, 97, 116, 104, 47, 101, 120, 97, 109, 112, 108, 101, 64, 48, 46, 49, 46, 48, 4, 0, 11, 13, 1, 0, 7, 101, 120, 97, 109, 112, 108, 101, 3, 0, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}