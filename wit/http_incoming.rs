// Generated by `wit-bindgen` 0.6.0. DO NOT EDIT!

#[allow(clippy::all)]
pub mod http_incoming {
    #[used]
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    static __FORCE_SECTION_REF: fn() = super::__link_section;

    pub type HttpUri = wit_bindgen::rt::string::String;
    pub type HttpStatusCode = u16;
    pub type HttpMethod = wit_bindgen::rt::string::String;
    pub type HttpHeaders = wit_bindgen::rt::vec::Vec<(
        wit_bindgen::rt::string::String,
        wit_bindgen::rt::string::String,
    )>;
    pub type HttpBodyHandle = u32;
    #[derive(Clone)]
    pub struct Response {
        pub status: HttpStatusCode,
        pub headers: HttpHeaders,
        pub body: Option<HttpBodyHandle>,
    }
    impl core::fmt::Debug for Response {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Response")
                .field("status", &self.status)
                .field("headers", &self.headers)
                .field("body", &self.body)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct Request {
        pub method: HttpMethod,
        pub uri: HttpUri,
        pub headers: HttpHeaders,
        pub body: Option<HttpBodyHandle>,
    }
    impl core::fmt::Debug for Request {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Request")
                .field("method", &self.method)
                .field("uri", &self.uri)
                .field("headers", &self.headers)
                .field("body", &self.body)
                .finish()
        }
    }
    pub trait HttpIncoming {
        fn handle_request(req: Request) -> Response;
    }

    #[doc(hidden)]
    pub unsafe fn call_handle_request<T: HttpIncoming>(
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
    ) -> i32 {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};

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
        #[cfg(target_arch = "wasm32")]
        wit_bindgen::rt::run_ctors_once();

        let len0 = arg1 as usize;
        let len1 = arg3 as usize;
        let base4 = arg4;
        let len4 = arg5;
        let mut result4 = Vec::with_capacity(len4 as usize);
        for i in 0..len4 {
            let base = base4 + i * 16;
            result4.push({
                let len2 = *((base + 4) as *const i32) as usize;
                let len3 = *((base + 12) as *const i32) as usize;

                (
                    {
                        #[cfg(not(debug_assertions))]
                        {
                            String::from_utf8_unchecked(Vec::from_raw_parts(
                                *((base + 0) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                        }
                        #[cfg(debug_assertions)]
                        {
                            String::from_utf8(Vec::from_raw_parts(
                                *((base + 0) as *const i32) as *mut _,
                                len2,
                                len2,
                            ))
                            .unwrap()
                        }
                    },
                    {
                        #[cfg(not(debug_assertions))]
                        {
                            String::from_utf8_unchecked(Vec::from_raw_parts(
                                *((base + 8) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                        }
                        #[cfg(debug_assertions)]
                        {
                            String::from_utf8(Vec::from_raw_parts(
                                *((base + 8) as *const i32) as *mut _,
                                len3,
                                len3,
                            ))
                            .unwrap()
                        }
                    },
                )
            });
        }
        wit_bindgen::rt::dealloc(base4, (len4 as usize) * 16, 4);
        let result5 = T::handle_request(Request {
            method: {
                #[cfg(not(debug_assertions))]
                {
                    String::from_utf8_unchecked(Vec::from_raw_parts(arg0 as *mut _, len0, len0))
                }
                #[cfg(debug_assertions)]
                {
                    String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap()
                }
            },
            uri: {
                #[cfg(not(debug_assertions))]
                {
                    String::from_utf8_unchecked(Vec::from_raw_parts(arg2 as *mut _, len1, len1))
                }
                #[cfg(debug_assertions)]
                {
                    String::from_utf8(Vec::from_raw_parts(arg2 as *mut _, len1, len1)).unwrap()
                }
            },
            headers: result4,
            body: match arg6 {
                0 => None,
                1 => Some(arg7 as u32),
                #[cfg(not(debug_assertions))]
                _ => core::hint::unreachable_unchecked(),
                #[cfg(debug_assertions)]
                _ => panic!("invalid enum discriminant"),
            },
        });
        let ptr6 = _RET_AREA.0.as_mut_ptr() as i32;
        let Response {
            status: status7,
            headers: headers7,
            body: body7,
        } = result5;
        *((ptr6 + 0) as *mut u16) = (wit_bindgen::rt::as_i32(status7)) as u16;
        let vec11 = headers7;
        let len11 = vec11.len() as i32;
        let layout11 = alloc::Layout::from_size_align_unchecked(vec11.len() * 16, 4);
        let result11 = if layout11.size() != 0 {
            let ptr = alloc::alloc(layout11);
            if ptr.is_null() {
                alloc::handle_alloc_error(layout11);
            }
            ptr
        } else {
            core::ptr::null_mut()
        };
        for (i, e) in vec11.into_iter().enumerate() {
            let base = result11 as i32 + (i as i32) * 16;
            {
                let (t8_0, t8_1) = e;
                let vec9 = (t8_0.into_bytes()).into_boxed_slice();
                let ptr9 = vec9.as_ptr() as i32;
                let len9 = vec9.len() as i32;
                core::mem::forget(vec9);
                *((base + 4) as *mut i32) = len9;
                *((base + 0) as *mut i32) = ptr9;
                let vec10 = (t8_1.into_bytes()).into_boxed_slice();
                let ptr10 = vec10.as_ptr() as i32;
                let len10 = vec10.len() as i32;
                core::mem::forget(vec10);
                *((base + 12) as *mut i32) = len10;
                *((base + 8) as *mut i32) = ptr10;
            }
        }
        *((ptr6 + 8) as *mut i32) = len11;
        *((ptr6 + 4) as *mut i32) = result11 as i32;
        match body7 {
            Some(e) => {
                *((ptr6 + 12) as *mut u8) = (1i32) as u8;
                *((ptr6 + 16) as *mut i32) = wit_bindgen::rt::as_i32(e);
            }
            None => {
                *((ptr6 + 12) as *mut u8) = (0i32) as u8;
            }
        };
        ptr6
    }

    #[doc(hidden)]
    pub unsafe fn post_return_handle_request<T: HttpIncoming>(arg0: i32) {
        let base0 = *((arg0 + 4) as *const i32);
        let len0 = *((arg0 + 8) as *const i32);
        for i in 0..len0 {
            let base = base0 + i * 16;
            {
                wit_bindgen::rt::dealloc(
                    *((base + 0) as *const i32),
                    (*((base + 4) as *const i32)) as usize,
                    1,
                );
                wit_bindgen::rt::dealloc(
                    *((base + 8) as *const i32),
                    (*((base + 12) as *const i32)) as usize,
                    1,
                );
            }
        }
        wit_bindgen::rt::dealloc(base0, (len0 as usize) * 16, 4);
    }

    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};

    #[repr(align(4))]
    struct _RetArea([u8; 20]);
    static mut _RET_AREA: _RetArea = _RetArea([0; 20]);
}

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_http_incoming(($t:ident) => {
    const _: () = {

      #[doc(hidden)]
      #[export_name = "http-incoming#handle-request"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __export_http_incoming_handle_request(arg0: i32,arg1: i32,arg2: i32,arg3: i32,arg4: i32,arg5: i32,arg6: i32,arg7: i32,) -> i32 {
        http_incoming::call_handle_request::<$t>(arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,)
      }

      #[doc(hidden)]
      #[export_name = "cabi_post_http-incoming#handle-request"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn __post_return_http_incoming_handle_request(arg0: i32,) {
        http_incoming::post_return_handle_request::<$t>(arg0,)
      }

    };

    #[used]
    #[doc(hidden)]
    #[cfg(target_arch = "wasm32")]
    static __FORCE_SECTION_REF: fn() = __link_section;
  });

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:http-incoming"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2712] = [
    2, 0, 3, 119, 105, 116, 13, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110, 103, 13,
    104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110, 103, 0, 97, 115, 109, 12, 0, 1, 0, 7,
    190, 8, 1, 65, 4, 1, 66, 27, 1, 109, 6, 13, 110, 101, 116, 119, 111, 114, 107, 45, 101, 114,
    114, 111, 114, 7, 116, 105, 109, 101, 111, 117, 116, 11, 105, 110, 118, 97, 108, 105, 100, 45,
    117, 114, 108, 23, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 45, 110, 111, 116, 45,
    97, 108, 108, 111, 119, 101, 100, 17, 116, 111, 111, 45, 109, 97, 110, 121, 45, 114, 101, 113,
    117, 101, 115, 116, 115, 15, 105, 110, 118, 97, 108, 105, 100, 45, 114, 101, 113, 117, 101,
    115, 116, 4, 13, 114, 101, 113, 117, 101, 115, 116, 45, 101, 114, 114, 111, 114, 0, 3, 0, 0, 1,
    109, 3, 6, 102, 111, 108, 108, 111, 119, 5, 101, 114, 114, 111, 114, 6, 109, 97, 110, 117, 97,
    108, 4, 15, 114, 101, 100, 105, 114, 101, 99, 116, 45, 112, 111, 108, 105, 99, 121, 0, 3, 0, 2,
    1, 114, 2, 7, 116, 105, 109, 101, 111, 117, 116, 121, 8, 114, 101, 100, 105, 114, 101, 99, 116,
    3, 4, 15, 114, 101, 113, 117, 101, 115, 116, 45, 111, 112, 116, 105, 111, 110, 115, 0, 3, 0, 4,
    1, 115, 4, 8, 104, 116, 116, 112, 45, 117, 114, 105, 0, 3, 0, 6, 1, 123, 4, 16, 104, 116, 116,
    112, 45, 115, 116, 97, 116, 117, 115, 45, 99, 111, 100, 101, 0, 3, 0, 8, 1, 115, 4, 11, 104,
    116, 116, 112, 45, 109, 101, 116, 104, 111, 100, 0, 3, 0, 10, 1, 111, 2, 115, 115, 1, 112, 12,
    4, 12, 104, 116, 116, 112, 45, 104, 101, 97, 100, 101, 114, 115, 0, 3, 0, 13, 1, 121, 4, 16,
    104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 104, 97, 110, 100, 108, 101, 0, 3, 0, 15, 1,
    107, 16, 1, 114, 3, 6, 115, 116, 97, 116, 117, 115, 9, 7, 104, 101, 97, 100, 101, 114, 115, 14,
    4, 98, 111, 100, 121, 17, 4, 8, 114, 101, 115, 112, 111, 110, 115, 101, 0, 3, 0, 18, 1, 114, 4,
    6, 109, 101, 116, 104, 111, 100, 11, 3, 117, 114, 105, 7, 7, 104, 101, 97, 100, 101, 114, 115,
    14, 4, 98, 111, 100, 121, 17, 4, 7, 114, 101, 113, 117, 101, 115, 116, 0, 3, 0, 20, 1, 112,
    125, 4, 9, 104, 116, 116, 112, 45, 98, 111, 100, 121, 0, 3, 0, 22, 1, 106, 1, 19, 1, 1, 1, 64,
    2, 3, 114, 101, 113, 21, 7, 111, 112, 116, 105, 111, 110, 115, 5, 0, 24, 4, 5, 102, 101, 116,
    99, 104, 0, 1, 25, 4, 21, 104, 116, 116, 112, 45, 111, 117, 116, 103, 111, 105, 110, 103, 45,
    105, 109, 112, 111, 114, 116, 115, 40, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 111, 117,
    116, 103, 111, 105, 110, 103, 47, 104, 116, 116, 112, 45, 111, 117, 116, 103, 111, 105, 110,
    103, 45, 105, 109, 112, 111, 114, 116, 115, 5, 0, 1, 65, 2, 1, 66, 27, 1, 109, 6, 13, 110, 101,
    116, 119, 111, 114, 107, 45, 101, 114, 114, 111, 114, 7, 116, 105, 109, 101, 111, 117, 116, 11,
    105, 110, 118, 97, 108, 105, 100, 45, 117, 114, 108, 23, 100, 101, 115, 116, 105, 110, 97, 116,
    105, 111, 110, 45, 110, 111, 116, 45, 97, 108, 108, 111, 119, 101, 100, 17, 116, 111, 111, 45,
    109, 97, 110, 121, 45, 114, 101, 113, 117, 101, 115, 116, 115, 15, 105, 110, 118, 97, 108, 105,
    100, 45, 114, 101, 113, 117, 101, 115, 116, 4, 13, 114, 101, 113, 117, 101, 115, 116, 45, 101,
    114, 114, 111, 114, 0, 3, 0, 0, 1, 109, 3, 6, 102, 111, 108, 108, 111, 119, 5, 101, 114, 114,
    111, 114, 6, 109, 97, 110, 117, 97, 108, 4, 15, 114, 101, 100, 105, 114, 101, 99, 116, 45, 112,
    111, 108, 105, 99, 121, 0, 3, 0, 2, 1, 114, 2, 7, 116, 105, 109, 101, 111, 117, 116, 121, 8,
    114, 101, 100, 105, 114, 101, 99, 116, 3, 4, 15, 114, 101, 113, 117, 101, 115, 116, 45, 111,
    112, 116, 105, 111, 110, 115, 0, 3, 0, 4, 1, 115, 4, 8, 104, 116, 116, 112, 45, 117, 114, 105,
    0, 3, 0, 6, 1, 123, 4, 16, 104, 116, 116, 112, 45, 115, 116, 97, 116, 117, 115, 45, 99, 111,
    100, 101, 0, 3, 0, 8, 1, 115, 4, 11, 104, 116, 116, 112, 45, 109, 101, 116, 104, 111, 100, 0,
    3, 0, 10, 1, 111, 2, 115, 115, 1, 112, 12, 4, 12, 104, 116, 116, 112, 45, 104, 101, 97, 100,
    101, 114, 115, 0, 3, 0, 13, 1, 121, 4, 16, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 104,
    97, 110, 100, 108, 101, 0, 3, 0, 15, 1, 107, 16, 1, 114, 3, 6, 115, 116, 97, 116, 117, 115, 9,
    7, 104, 101, 97, 100, 101, 114, 115, 14, 4, 98, 111, 100, 121, 17, 4, 8, 114, 101, 115, 112,
    111, 110, 115, 101, 0, 3, 0, 18, 1, 114, 4, 6, 109, 101, 116, 104, 111, 100, 11, 3, 117, 114,
    105, 7, 7, 104, 101, 97, 100, 101, 114, 115, 14, 4, 98, 111, 100, 121, 17, 4, 7, 114, 101, 113,
    117, 101, 115, 116, 0, 3, 0, 20, 1, 112, 125, 4, 9, 104, 116, 116, 112, 45, 98, 111, 100, 121,
    0, 3, 0, 22, 1, 106, 1, 19, 1, 1, 1, 64, 2, 3, 114, 101, 113, 21, 7, 111, 112, 116, 105, 111,
    110, 115, 5, 0, 24, 4, 5, 102, 101, 116, 99, 104, 0, 1, 25, 3, 13, 104, 116, 116, 112, 45, 111,
    117, 116, 103, 111, 105, 110, 103, 40, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 111, 117,
    116, 103, 111, 105, 110, 103, 47, 104, 116, 116, 112, 45, 111, 117, 116, 103, 111, 105, 110,
    103, 45, 105, 109, 112, 111, 114, 116, 115, 5, 0, 4, 13, 104, 116, 116, 112, 45, 111, 117, 116,
    103, 111, 105, 110, 103, 32, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 111, 117, 116, 103,
    111, 105, 110, 103, 47, 104, 116, 116, 112, 45, 111, 117, 116, 103, 111, 105, 110, 103, 4, 1,
    11, 37, 1, 13, 104, 116, 116, 112, 45, 111, 117, 116, 103, 111, 105, 110, 103, 18, 112, 107,
    103, 58, 47, 104, 116, 116, 112, 45, 111, 117, 116, 103, 111, 105, 110, 103, 3, 0, 0, 7, 160,
    5, 1, 65, 4, 1, 66, 20, 1, 115, 4, 8, 104, 116, 116, 112, 45, 117, 114, 105, 0, 3, 0, 0, 1,
    123, 4, 16, 104, 116, 116, 112, 45, 115, 116, 97, 116, 117, 115, 45, 99, 111, 100, 101, 0, 3,
    0, 2, 1, 115, 4, 11, 104, 116, 116, 112, 45, 109, 101, 116, 104, 111, 100, 0, 3, 0, 4, 1, 111,
    2, 115, 115, 1, 112, 6, 4, 12, 104, 116, 116, 112, 45, 104, 101, 97, 100, 101, 114, 115, 0, 3,
    0, 7, 1, 121, 4, 16, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 104, 97, 110, 100, 108,
    101, 0, 3, 0, 9, 1, 107, 10, 1, 114, 3, 6, 115, 116, 97, 116, 117, 115, 3, 7, 104, 101, 97,
    100, 101, 114, 115, 8, 4, 98, 111, 100, 121, 11, 4, 8, 114, 101, 115, 112, 111, 110, 115, 101,
    0, 3, 0, 12, 1, 114, 4, 6, 109, 101, 116, 104, 111, 100, 5, 3, 117, 114, 105, 1, 7, 104, 101,
    97, 100, 101, 114, 115, 8, 4, 98, 111, 100, 121, 11, 4, 7, 114, 101, 113, 117, 101, 115, 116,
    0, 3, 0, 14, 1, 112, 125, 4, 9, 104, 116, 116, 112, 45, 98, 111, 100, 121, 0, 3, 0, 16, 1, 64,
    1, 3, 114, 101, 113, 15, 0, 13, 4, 14, 104, 97, 110, 100, 108, 101, 45, 114, 101, 113, 117,
    101, 115, 116, 0, 1, 18, 4, 21, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110, 103,
    45, 101, 120, 112, 111, 114, 116, 115, 40, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 105,
    110, 99, 111, 109, 105, 110, 103, 47, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110,
    103, 45, 101, 120, 112, 111, 114, 116, 115, 5, 0, 1, 65, 2, 1, 66, 20, 1, 115, 4, 8, 104, 116,
    116, 112, 45, 117, 114, 105, 0, 3, 0, 0, 1, 123, 4, 16, 104, 116, 116, 112, 45, 115, 116, 97,
    116, 117, 115, 45, 99, 111, 100, 101, 0, 3, 0, 2, 1, 115, 4, 11, 104, 116, 116, 112, 45, 109,
    101, 116, 104, 111, 100, 0, 3, 0, 4, 1, 111, 2, 115, 115, 1, 112, 6, 4, 12, 104, 116, 116, 112,
    45, 104, 101, 97, 100, 101, 114, 115, 0, 3, 0, 7, 1, 121, 4, 16, 104, 116, 116, 112, 45, 98,
    111, 100, 121, 45, 104, 97, 110, 100, 108, 101, 0, 3, 0, 9, 1, 107, 10, 1, 114, 3, 6, 115, 116,
    97, 116, 117, 115, 3, 7, 104, 101, 97, 100, 101, 114, 115, 8, 4, 98, 111, 100, 121, 11, 4, 8,
    114, 101, 115, 112, 111, 110, 115, 101, 0, 3, 0, 12, 1, 114, 4, 6, 109, 101, 116, 104, 111,
    100, 5, 3, 117, 114, 105, 1, 7, 104, 101, 97, 100, 101, 114, 115, 8, 4, 98, 111, 100, 121, 11,
    4, 7, 114, 101, 113, 117, 101, 115, 116, 0, 3, 0, 14, 1, 112, 125, 4, 9, 104, 116, 116, 112,
    45, 98, 111, 100, 121, 0, 3, 0, 16, 1, 64, 1, 3, 114, 101, 113, 15, 0, 13, 4, 14, 104, 97, 110,
    100, 108, 101, 45, 114, 101, 113, 117, 101, 115, 116, 0, 1, 18, 4, 13, 104, 116, 116, 112, 45,
    105, 110, 99, 111, 109, 105, 110, 103, 40, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 105,
    110, 99, 111, 109, 105, 110, 103, 47, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110,
    103, 45, 101, 120, 112, 111, 114, 116, 115, 5, 0, 4, 13, 104, 116, 116, 112, 45, 105, 110, 99,
    111, 109, 105, 110, 103, 32, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 105, 110, 99, 111,
    109, 105, 110, 103, 47, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110, 103, 4, 1,
    11, 37, 1, 13, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110, 103, 18, 112, 107,
    103, 58, 47, 104, 116, 116, 112, 45, 105, 110, 99, 111, 109, 105, 110, 103, 3, 2, 0, 7, 212, 5,
    1, 65, 4, 1, 66, 19, 1, 121, 4, 16, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 104, 97,
    110, 100, 108, 101, 0, 3, 0, 0, 1, 113, 4, 14, 105, 110, 118, 97, 108, 105, 100, 45, 104, 97,
    110, 100, 108, 101, 0, 0, 9, 114, 101, 97, 100, 45, 111, 110, 108, 121, 0, 0, 11, 114, 101, 97,
    100, 45, 102, 97, 105, 108, 101, 100, 1, 115, 0, 12, 119, 114, 105, 116, 101, 45, 102, 97, 105,
    108, 101, 100, 1, 115, 0, 4, 10, 98, 111, 100, 121, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1,
    112, 125, 1, 111, 2, 4, 127, 1, 106, 1, 5, 1, 3, 1, 64, 1, 6, 104, 97, 110, 100, 108, 101, 1,
    0, 6, 4, 14, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 114, 101, 97, 100, 0, 1, 7, 1, 106,
    1, 4, 1, 3, 1, 64, 1, 6, 104, 97, 110, 100, 108, 101, 1, 0, 8, 4, 18, 104, 116, 116, 112, 45,
    98, 111, 100, 121, 45, 114, 101, 97, 100, 45, 97, 108, 108, 0, 1, 9, 1, 106, 1, 119, 1, 3, 1,
    64, 2, 6, 104, 97, 110, 100, 108, 101, 1, 4, 100, 97, 116, 97, 4, 0, 10, 4, 15, 104, 116, 116,
    112, 45, 98, 111, 100, 121, 45, 119, 114, 105, 116, 101, 0, 1, 11, 1, 106, 1, 1, 1, 3, 1, 64,
    0, 0, 12, 4, 13, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 110, 101, 119, 0, 1, 13, 4, 20,
    104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 110, 101, 119, 45, 115, 116, 114, 101, 97, 109,
    0, 1, 13, 4, 15, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 105, 102, 97, 99, 101, 30, 112,
    107, 103, 58, 47, 104, 116, 116, 112, 45, 98, 111, 100, 121, 47, 104, 116, 116, 112, 45, 98,
    111, 100, 121, 45, 105, 102, 97, 99, 101, 5, 0, 1, 65, 2, 1, 66, 19, 1, 121, 4, 16, 104, 116,
    116, 112, 45, 98, 111, 100, 121, 45, 104, 97, 110, 100, 108, 101, 0, 3, 0, 0, 1, 113, 4, 14,
    105, 110, 118, 97, 108, 105, 100, 45, 104, 97, 110, 100, 108, 101, 0, 0, 9, 114, 101, 97, 100,
    45, 111, 110, 108, 121, 0, 0, 11, 114, 101, 97, 100, 45, 102, 97, 105, 108, 101, 100, 1, 115,
    0, 12, 119, 114, 105, 116, 101, 45, 102, 97, 105, 108, 101, 100, 1, 115, 0, 4, 10, 98, 111,
    100, 121, 45, 101, 114, 114, 111, 114, 0, 3, 0, 2, 1, 112, 125, 1, 111, 2, 4, 127, 1, 106, 1,
    5, 1, 3, 1, 64, 1, 6, 104, 97, 110, 100, 108, 101, 1, 0, 6, 4, 14, 104, 116, 116, 112, 45, 98,
    111, 100, 121, 45, 114, 101, 97, 100, 0, 1, 7, 1, 106, 1, 4, 1, 3, 1, 64, 1, 6, 104, 97, 110,
    100, 108, 101, 1, 0, 8, 4, 18, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 114, 101, 97,
    100, 45, 97, 108, 108, 0, 1, 9, 1, 106, 1, 119, 1, 3, 1, 64, 2, 6, 104, 97, 110, 100, 108, 101,
    1, 4, 100, 97, 116, 97, 4, 0, 10, 4, 15, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 119,
    114, 105, 116, 101, 0, 1, 11, 1, 106, 1, 1, 1, 3, 1, 64, 0, 0, 12, 4, 13, 104, 116, 116, 112,
    45, 98, 111, 100, 121, 45, 110, 101, 119, 0, 1, 13, 4, 20, 104, 116, 116, 112, 45, 98, 111,
    100, 121, 45, 110, 101, 119, 45, 115, 116, 114, 101, 97, 109, 0, 1, 13, 3, 9, 104, 116, 116,
    112, 45, 98, 111, 100, 121, 30, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 98, 111, 100,
    121, 47, 104, 116, 116, 112, 45, 98, 111, 100, 121, 45, 105, 102, 97, 99, 101, 5, 0, 4, 9, 104,
    116, 116, 112, 45, 98, 111, 100, 121, 24, 112, 107, 103, 58, 47, 104, 116, 116, 112, 45, 98,
    111, 100, 121, 47, 104, 116, 116, 112, 45, 98, 111, 100, 121, 4, 1, 0, 68, 9, 112, 114, 111,
    100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2,
    13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46, 56, 46, 50, 16,
    119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 5, 48, 46, 54, 46,
    48, 11, 29, 1, 9, 104, 116, 116, 112, 45, 98, 111, 100, 121, 14, 112, 107, 103, 58, 47, 104,
    116, 116, 112, 45, 98, 111, 100, 121, 3, 4, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
