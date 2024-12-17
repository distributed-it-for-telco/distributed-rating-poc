#[allow(dead_code)]
pub mod orange {
    #[allow(dead_code)]
    pub mod commons {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub struct Balance {
                pub count: f32,
                pub unit: _rt::String,
                pub party_id: _rt::String,
            }
            impl ::core::fmt::Debug for Balance {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Balance")
                        .field("count", &self.count)
                        .field("unit", &self.unit)
                        .field("party-id", &self.party_id)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct AgentIdentification {
                pub name: _rt::String,
                pub partner_id: _rt::String,
            }
            impl ::core::fmt::Debug for AgentIdentification {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AgentIdentification")
                        .field("name", &self.name)
                        .field("partner-id", &self.partner_id)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct UsageCharacteristic {
                pub name: _rt::String,
                pub value: _rt::String,
                pub value_type: _rt::String,
            }
            impl ::core::fmt::Debug for UsageCharacteristic {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("UsageCharacteristic")
                        .field("name", &self.name)
                        .field("value", &self.value)
                        .field("value-type", &self.value_type)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct UsageProofRequest {
                pub party_id: _rt::String,
                pub usage_id: _rt::String,
                pub usage_characteristic_list: _rt::Vec<UsageCharacteristic>,
                pub rating: _rt::String,
                pub usage_date: _rt::String,
                pub offer_id: _rt::String,
                pub description: _rt::String,
                pub usage_type: _rt::String,
                pub product_name: _rt::String,
            }
            impl ::core::fmt::Debug for UsageProofRequest {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("UsageProofRequest")
                        .field("party-id", &self.party_id)
                        .field("usage-id", &self.usage_id)
                        .field(
                            "usage-characteristic-list",
                            &self.usage_characteristic_list,
                        )
                        .field("rating", &self.rating)
                        .field("usage-date", &self.usage_date)
                        .field("offer-id", &self.offer_id)
                        .field("description", &self.description)
                        .field("usage-type", &self.usage_type)
                        .field("product-name", &self.product_name)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Usage {
                pub usage_characteristic_list: _rt::Vec<UsageCharacteristic>,
            }
            impl ::core::fmt::Debug for Usage {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Usage")
                        .field(
                            "usage-characteristic-list",
                            &self.usage_characteristic_list,
                        )
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct RatingRecord {
                pub producer: _rt::String,
                pub unit: _rt::String,
                pub price: _rt::String,
            }
            impl ::core::fmt::Debug for RatingRecord {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RatingRecord")
                        .field("producer", &self.producer)
                        .field("unit", &self.unit)
                        .field("price", &self.price)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct RatingRequest {
                pub customer_id: _rt::String,
                pub agent_id: _rt::String,
                pub language: _rt::String,
                pub offer_id: _rt::String,
                pub usage: Usage,
                pub rating_history: _rt::Vec<RatingRecord>,
            }
            impl ::core::fmt::Debug for RatingRequest {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RatingRequest")
                        .field("customer-id", &self.customer_id)
                        .field("agent-id", &self.agent_id)
                        .field("language", &self.language)
                        .field("offer-id", &self.offer_id)
                        .field("usage", &self.usage)
                        .field("rating-history", &self.rating_history)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct BillingInformation {
                pub price: _rt::String,
                pub unit: _rt::String,
                pub messages: _rt::Vec<_rt::String>,
            }
            impl ::core::fmt::Debug for BillingInformation {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("BillingInformation")
                        .field("price", &self.price)
                        .field("unit", &self.unit)
                        .field("messages", &self.messages)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct AuthorizationStatus {
                pub code: u16,
                pub key: _rt::String,
            }
            impl ::core::fmt::Debug for AuthorizationStatus {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AuthorizationStatus")
                        .field("code", &self.code)
                        .field("key", &self.key)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct RatingResponse {
                pub authorization_status: AuthorizationStatus,
                pub billing_information: BillingInformation,
                pub next_agent: AgentIdentification,
            }
            impl ::core::fmt::Debug for RatingResponse {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("RatingResponse")
                        .field("authorization-status", &self.authorization_status)
                        .field("billing-information", &self.billing_information)
                        .field("next-agent", &self.next_agent)
                        .finish()
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod orange {
        #[allow(dead_code)]
        pub mod commons {
            #[allow(dead_code, clippy::all)]
            pub mod mappers {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type RatingRequest = super::super::super::super::orange::commons::types::RatingRequest;
                pub type RatingResponse = super::super::super::super::orange::commons::types::RatingResponse;
                pub type Balance = super::super::super::super::orange::commons::types::Balance;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_rating_request_to_string_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                    arg6: *mut u8,
                    arg7: usize,
                    arg8: *mut u8,
                    arg9: usize,
                    arg10: *mut u8,
                    arg11: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let len2 = arg5;
                    let bytes2 = _rt::Vec::from_raw_parts(arg4.cast(), len2, len2);
                    let len3 = arg7;
                    let bytes3 = _rt::Vec::from_raw_parts(arg6.cast(), len3, len3);
                    let base13 = arg8;
                    let len13 = arg9;
                    let mut result13 = _rt::Vec::with_capacity(len13);
                    for i in 0..len13 {
                        let base = base13.add(i * 24);
                        let e13 = {
                            let l4 = *base.add(0).cast::<*mut u8>();
                            let l5 = *base.add(4).cast::<usize>();
                            let len6 = l5;
                            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
                            let l7 = *base.add(8).cast::<*mut u8>();
                            let l8 = *base.add(12).cast::<usize>();
                            let len9 = l8;
                            let bytes9 = _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                            let l10 = *base.add(16).cast::<*mut u8>();
                            let l11 = *base.add(20).cast::<usize>();
                            let len12 = l11;
                            let bytes12 = _rt::Vec::from_raw_parts(
                                l10.cast(),
                                len12,
                                len12,
                            );
                            super::super::super::super::orange::commons::types::UsageCharacteristic {
                                name: _rt::string_lift(bytes6),
                                value: _rt::string_lift(bytes9),
                                value_type: _rt::string_lift(bytes12),
                            }
                        };
                        result13.push(e13);
                    }
                    _rt::cabi_dealloc(base13, len13 * 24, 4);
                    let base23 = arg10;
                    let len23 = arg11;
                    let mut result23 = _rt::Vec::with_capacity(len23);
                    for i in 0..len23 {
                        let base = base23.add(i * 24);
                        let e23 = {
                            let l14 = *base.add(0).cast::<*mut u8>();
                            let l15 = *base.add(4).cast::<usize>();
                            let len16 = l15;
                            let bytes16 = _rt::Vec::from_raw_parts(
                                l14.cast(),
                                len16,
                                len16,
                            );
                            let l17 = *base.add(8).cast::<*mut u8>();
                            let l18 = *base.add(12).cast::<usize>();
                            let len19 = l18;
                            let bytes19 = _rt::Vec::from_raw_parts(
                                l17.cast(),
                                len19,
                                len19,
                            );
                            let l20 = *base.add(16).cast::<*mut u8>();
                            let l21 = *base.add(20).cast::<usize>();
                            let len22 = l21;
                            let bytes22 = _rt::Vec::from_raw_parts(
                                l20.cast(),
                                len22,
                                len22,
                            );
                            super::super::super::super::orange::commons::types::RatingRecord {
                                producer: _rt::string_lift(bytes16),
                                unit: _rt::string_lift(bytes19),
                                price: _rt::string_lift(bytes22),
                            }
                        };
                        result23.push(e23);
                    }
                    _rt::cabi_dealloc(base23, len23 * 24, 4);
                    let result24 = T::rating_request_to_string(super::super::super::super::orange::commons::types::RatingRequest {
                        customer_id: _rt::string_lift(bytes0),
                        agent_id: _rt::string_lift(bytes1),
                        language: _rt::string_lift(bytes2),
                        offer_id: _rt::string_lift(bytes3),
                        usage: super::super::super::super::orange::commons::types::Usage {
                            usage_characteristic_list: result13,
                        },
                        rating_history: result23,
                    });
                    let ptr25 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec26 = (result24.into_bytes()).into_boxed_slice();
                    let ptr26 = vec26.as_ptr().cast::<u8>();
                    let len26 = vec26.len();
                    ::core::mem::forget(vec26);
                    *ptr25.add(4).cast::<usize>() = len26;
                    *ptr25.add(0).cast::<*mut u8>() = ptr26.cast_mut();
                    ptr25
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_rating_request_to_string<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_string_to_rating_request_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::string_to_rating_request(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::orange::commons::types::RatingRequest {
                        customer_id: customer_id3,
                        agent_id: agent_id3,
                        language: language3,
                        offer_id: offer_id3,
                        usage: usage3,
                        rating_history: rating_history3,
                    } = result1;
                    let vec4 = (customer_id3.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(4).cast::<usize>() = len4;
                    *ptr2.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    let vec5 = (agent_id3.into_bytes()).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr2.add(12).cast::<usize>() = len5;
                    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                    let vec6 = (language3.into_bytes()).into_boxed_slice();
                    let ptr6 = vec6.as_ptr().cast::<u8>();
                    let len6 = vec6.len();
                    ::core::mem::forget(vec6);
                    *ptr2.add(20).cast::<usize>() = len6;
                    *ptr2.add(16).cast::<*mut u8>() = ptr6.cast_mut();
                    let vec7 = (offer_id3.into_bytes()).into_boxed_slice();
                    let ptr7 = vec7.as_ptr().cast::<u8>();
                    let len7 = vec7.len();
                    ::core::mem::forget(vec7);
                    *ptr2.add(28).cast::<usize>() = len7;
                    *ptr2.add(24).cast::<*mut u8>() = ptr7.cast_mut();
                    let super::super::super::super::orange::commons::types::Usage {
                        usage_characteristic_list: usage_characteristic_list8,
                    } = usage3;
                    let vec13 = usage_characteristic_list8;
                    let len13 = vec13.len();
                    let layout13 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec13.len() * 24,
                        4,
                    );
                    let result13 = if layout13.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout13).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout13);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec13.into_iter().enumerate() {
                        let base = result13.add(i * 24);
                        {
                            let super::super::super::super::orange::commons::types::UsageCharacteristic {
                                name: name9,
                                value: value9,
                                value_type: value_type9,
                            } = e;
                            let vec10 = (name9.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *base.add(4).cast::<usize>() = len10;
                            *base.add(0).cast::<*mut u8>() = ptr10.cast_mut();
                            let vec11 = (value9.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *base.add(12).cast::<usize>() = len11;
                            *base.add(8).cast::<*mut u8>() = ptr11.cast_mut();
                            let vec12 = (value_type9.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *base.add(20).cast::<usize>() = len12;
                            *base.add(16).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    }
                    *ptr2.add(36).cast::<usize>() = len13;
                    *ptr2.add(32).cast::<*mut u8>() = result13;
                    let vec18 = rating_history3;
                    let len18 = vec18.len();
                    let layout18 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec18.len() * 24,
                        4,
                    );
                    let result18 = if layout18.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout18).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout18);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec18.into_iter().enumerate() {
                        let base = result18.add(i * 24);
                        {
                            let super::super::super::super::orange::commons::types::RatingRecord {
                                producer: producer14,
                                unit: unit14,
                                price: price14,
                            } = e;
                            let vec15 = (producer14.into_bytes()).into_boxed_slice();
                            let ptr15 = vec15.as_ptr().cast::<u8>();
                            let len15 = vec15.len();
                            ::core::mem::forget(vec15);
                            *base.add(4).cast::<usize>() = len15;
                            *base.add(0).cast::<*mut u8>() = ptr15.cast_mut();
                            let vec16 = (unit14.into_bytes()).into_boxed_slice();
                            let ptr16 = vec16.as_ptr().cast::<u8>();
                            let len16 = vec16.len();
                            ::core::mem::forget(vec16);
                            *base.add(12).cast::<usize>() = len16;
                            *base.add(8).cast::<*mut u8>() = ptr16.cast_mut();
                            let vec17 = (price14.into_bytes()).into_boxed_slice();
                            let ptr17 = vec17.as_ptr().cast::<u8>();
                            let len17 = vec17.len();
                            ::core::mem::forget(vec17);
                            *base.add(20).cast::<usize>() = len17;
                            *base.add(16).cast::<*mut u8>() = ptr17.cast_mut();
                        }
                    }
                    *ptr2.add(44).cast::<usize>() = len18;
                    *ptr2.add(40).cast::<*mut u8>() = result18;
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_string_to_rating_request<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                    let l4 = *arg0.add(16).cast::<*mut u8>();
                    let l5 = *arg0.add(20).cast::<usize>();
                    _rt::cabi_dealloc(l4, l5, 1);
                    let l6 = *arg0.add(24).cast::<*mut u8>();
                    let l7 = *arg0.add(28).cast::<usize>();
                    _rt::cabi_dealloc(l6, l7, 1);
                    let l8 = *arg0.add(32).cast::<*mut u8>();
                    let l9 = *arg0.add(36).cast::<usize>();
                    let base16 = l8;
                    let len16 = l9;
                    for i in 0..len16 {
                        let base = base16.add(i * 24);
                        {
                            let l10 = *base.add(0).cast::<*mut u8>();
                            let l11 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l10, l11, 1);
                            let l12 = *base.add(8).cast::<*mut u8>();
                            let l13 = *base.add(12).cast::<usize>();
                            _rt::cabi_dealloc(l12, l13, 1);
                            let l14 = *base.add(16).cast::<*mut u8>();
                            let l15 = *base.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l14, l15, 1);
                        }
                    }
                    _rt::cabi_dealloc(base16, len16 * 24, 4);
                    let l17 = *arg0.add(40).cast::<*mut u8>();
                    let l18 = *arg0.add(44).cast::<usize>();
                    let base25 = l17;
                    let len25 = l18;
                    for i in 0..len25 {
                        let base = base25.add(i * 24);
                        {
                            let l19 = *base.add(0).cast::<*mut u8>();
                            let l20 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l19, l20, 1);
                            let l21 = *base.add(8).cast::<*mut u8>();
                            let l22 = *base.add(12).cast::<usize>();
                            _rt::cabi_dealloc(l21, l22, 1);
                            let l23 = *base.add(16).cast::<*mut u8>();
                            let l24 = *base.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l23, l24, 1);
                        }
                    }
                    _rt::cabi_dealloc(base25, len25 * 24, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_rating_response_to_string_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: *mut u8,
                    arg6: usize,
                    arg7: *mut u8,
                    arg8: usize,
                    arg9: *mut u8,
                    arg10: usize,
                    arg11: *mut u8,
                    arg12: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let len2 = arg6;
                    let bytes2 = _rt::Vec::from_raw_parts(arg5.cast(), len2, len2);
                    let base6 = arg7;
                    let len6 = arg8;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 8);
                        let e6 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            _rt::string_lift(bytes5)
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 8, 4);
                    let len7 = arg10;
                    let bytes7 = _rt::Vec::from_raw_parts(arg9.cast(), len7, len7);
                    let len8 = arg12;
                    let bytes8 = _rt::Vec::from_raw_parts(arg11.cast(), len8, len8);
                    let result9 = T::rating_response_to_string(super::super::super::super::orange::commons::types::RatingResponse {
                        authorization_status: super::super::super::super::orange::commons::types::AuthorizationStatus {
                            code: arg0 as u16,
                            key: _rt::string_lift(bytes0),
                        },
                        billing_information: super::super::super::super::orange::commons::types::BillingInformation {
                            price: _rt::string_lift(bytes1),
                            unit: _rt::string_lift(bytes2),
                            messages: result6,
                        },
                        next_agent: super::super::super::super::orange::commons::types::AgentIdentification {
                            name: _rt::string_lift(bytes7),
                            partner_id: _rt::string_lift(bytes8),
                        },
                    });
                    let ptr10 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec11 = (result9.into_bytes()).into_boxed_slice();
                    let ptr11 = vec11.as_ptr().cast::<u8>();
                    let len11 = vec11.len();
                    ::core::mem::forget(vec11);
                    *ptr10.add(4).cast::<usize>() = len11;
                    *ptr10.add(0).cast::<*mut u8>() = ptr11.cast_mut();
                    ptr10
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_rating_response_to_string<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_string_to_rating_response_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::string_to_rating_response(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::orange::commons::types::RatingResponse {
                        authorization_status: authorization_status3,
                        billing_information: billing_information3,
                        next_agent: next_agent3,
                    } = result1;
                    let super::super::super::super::orange::commons::types::AuthorizationStatus {
                        code: code4,
                        key: key4,
                    } = authorization_status3;
                    *ptr2.add(0).cast::<u16>() = (_rt::as_i32(code4)) as u16;
                    let vec5 = (key4.into_bytes()).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr2.add(8).cast::<usize>() = len5;
                    *ptr2.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                    let super::super::super::super::orange::commons::types::BillingInformation {
                        price: price6,
                        unit: unit6,
                        messages: messages6,
                    } = billing_information3;
                    let vec7 = (price6.into_bytes()).into_boxed_slice();
                    let ptr7 = vec7.as_ptr().cast::<u8>();
                    let len7 = vec7.len();
                    ::core::mem::forget(vec7);
                    *ptr2.add(16).cast::<usize>() = len7;
                    *ptr2.add(12).cast::<*mut u8>() = ptr7.cast_mut();
                    let vec8 = (unit6.into_bytes()).into_boxed_slice();
                    let ptr8 = vec8.as_ptr().cast::<u8>();
                    let len8 = vec8.len();
                    ::core::mem::forget(vec8);
                    *ptr2.add(24).cast::<usize>() = len8;
                    *ptr2.add(20).cast::<*mut u8>() = ptr8.cast_mut();
                    let vec10 = messages6;
                    let len10 = vec10.len();
                    let layout10 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec10.len() * 8,
                        4,
                    );
                    let result10 = if layout10.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout10).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout10);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec10.into_iter().enumerate() {
                        let base = result10.add(i * 8);
                        {
                            let vec9 = (e.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *base.add(4).cast::<usize>() = len9;
                            *base.add(0).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    }
                    *ptr2.add(32).cast::<usize>() = len10;
                    *ptr2.add(28).cast::<*mut u8>() = result10;
                    let super::super::super::super::orange::commons::types::AgentIdentification {
                        name: name11,
                        partner_id: partner_id11,
                    } = next_agent3;
                    let vec12 = (name11.into_bytes()).into_boxed_slice();
                    let ptr12 = vec12.as_ptr().cast::<u8>();
                    let len12 = vec12.len();
                    ::core::mem::forget(vec12);
                    *ptr2.add(40).cast::<usize>() = len12;
                    *ptr2.add(36).cast::<*mut u8>() = ptr12.cast_mut();
                    let vec13 = (partner_id11.into_bytes()).into_boxed_slice();
                    let ptr13 = vec13.as_ptr().cast::<u8>();
                    let len13 = vec13.len();
                    ::core::mem::forget(vec13);
                    *ptr2.add(48).cast::<usize>() = len13;
                    *ptr2.add(44).cast::<*mut u8>() = ptr13.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_string_to_rating_response<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(4).cast::<*mut u8>();
                    let l1 = *arg0.add(8).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(12).cast::<*mut u8>();
                    let l3 = *arg0.add(16).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                    let l4 = *arg0.add(20).cast::<*mut u8>();
                    let l5 = *arg0.add(24).cast::<usize>();
                    _rt::cabi_dealloc(l4, l5, 1);
                    let l6 = *arg0.add(28).cast::<*mut u8>();
                    let l7 = *arg0.add(32).cast::<usize>();
                    let base10 = l6;
                    let len10 = l7;
                    for i in 0..len10 {
                        let base = base10.add(i * 8);
                        {
                            let l8 = *base.add(0).cast::<*mut u8>();
                            let l9 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l8, l9, 1);
                        }
                    }
                    _rt::cabi_dealloc(base10, len10 * 8, 4);
                    let l11 = *arg0.add(36).cast::<*mut u8>();
                    let l12 = *arg0.add(40).cast::<usize>();
                    _rt::cabi_dealloc(l11, l12, 1);
                    let l13 = *arg0.add(44).cast::<*mut u8>();
                    let l14 = *arg0.add(48).cast::<usize>();
                    _rt::cabi_dealloc(l13, l14, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_balance_to_string_cabi<T: Guest>(
                    arg0: f32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::balance_to_string(super::super::super::super::orange::commons::types::Balance {
                        count: arg0,
                        unit: _rt::string_lift(bytes0),
                        party_id: _rt::string_lift(bytes1),
                    });
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec4 = (result2.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr3.add(4).cast::<usize>() = len4;
                    *ptr3.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_balance_to_string<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_string_to_balance_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::string_to_balance(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::orange::commons::types::Balance {
                        count: count3,
                        unit: unit3,
                        party_id: party_id3,
                    } = result1;
                    *ptr2.add(0).cast::<f32>() = _rt::as_f32(count3);
                    let vec4 = (unit3.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(8).cast::<usize>() = len4;
                    *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                    let vec5 = (party_id3.into_bytes()).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr2.add(16).cast::<usize>() = len5;
                    *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_string_to_balance<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(4).cast::<*mut u8>();
                    let l1 = *arg0.add(8).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(12).cast::<*mut u8>();
                    let l3 = *arg0.add(16).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_balance_to_stringified_array_cabi<T: Guest>(
                    arg0: f32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::balance_to_stringified_array(super::super::super::super::orange::commons::types::Balance {
                        count: arg0,
                        unit: _rt::string_lift(bytes0),
                        party_id: _rt::string_lift(bytes1),
                    });
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec4 = (result2).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr3.add(4).cast::<usize>() = len4;
                    *ptr3.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_balance_to_stringified_array<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                pub trait Guest {
                    fn rating_request_to_string(request: RatingRequest) -> _rt::String;
                    fn string_to_rating_request(value: _rt::String) -> RatingRequest;
                    fn rating_response_to_string(
                        response: RatingResponse,
                    ) -> _rt::String;
                    fn string_to_rating_response(value: _rt::String) -> RatingResponse;
                    fn balance_to_string(response: Balance) -> _rt::String;
                    fn string_to_balance(value: _rt::String) -> Balance;
                    fn balance_to_stringified_array(response: Balance) -> _rt::Vec<u8>;
                }
                #[doc(hidden)]
                macro_rules! __export_orange_commons_mappers_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "orange:commons/mappers#rating-request-to-string"] unsafe extern
                        "C" fn export_rating_request_to_string(arg0 : * mut u8, arg1 :
                        usize, arg2 : * mut u8, arg3 : usize, arg4 : * mut u8, arg5 :
                        usize, arg6 : * mut u8, arg7 : usize, arg8 : * mut u8, arg9 :
                        usize, arg10 : * mut u8, arg11 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_rating_request_to_string_cabi::<$ty
                        > (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9,
                        arg10, arg11) } #[export_name =
                        "cabi_post_orange:commons/mappers#rating-request-to-string"]
                        unsafe extern "C" fn _post_return_rating_request_to_string(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_rating_request_to_string::<$ty > (arg0) }
                        #[export_name =
                        "orange:commons/mappers#string-to-rating-request"] unsafe extern
                        "C" fn export_string_to_rating_request(arg0 : * mut u8, arg1 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_string_to_rating_request_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_orange:commons/mappers#string-to-rating-request"]
                        unsafe extern "C" fn _post_return_string_to_rating_request(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_string_to_rating_request::<$ty > (arg0) }
                        #[export_name =
                        "orange:commons/mappers#rating-response-to-string"] unsafe extern
                        "C" fn export_rating_response_to_string(arg0 : i32, arg1 : * mut
                        u8, arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : * mut u8,
                        arg6 : usize, arg7 : * mut u8, arg8 : usize, arg9 : * mut u8,
                        arg10 : usize, arg11 : * mut u8, arg12 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_rating_response_to_string_cabi::<$ty
                        > (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9,
                        arg10, arg11, arg12) } #[export_name =
                        "cabi_post_orange:commons/mappers#rating-response-to-string"]
                        unsafe extern "C" fn _post_return_rating_response_to_string(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_rating_response_to_string::<$ty > (arg0) }
                        #[export_name =
                        "orange:commons/mappers#string-to-rating-response"] unsafe extern
                        "C" fn export_string_to_rating_response(arg0 : * mut u8, arg1 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_string_to_rating_response_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_orange:commons/mappers#string-to-rating-response"]
                        unsafe extern "C" fn _post_return_string_to_rating_response(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_string_to_rating_response::<$ty > (arg0) }
                        #[export_name = "orange:commons/mappers#balance-to-string"]
                        unsafe extern "C" fn export_balance_to_string(arg0 : f32, arg1 :
                        * mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize,) -> * mut
                        u8 { $($path_to_types)*:: _export_balance_to_string_cabi::<$ty >
                        (arg0, arg1, arg2, arg3, arg4) } #[export_name =
                        "cabi_post_orange:commons/mappers#balance-to-string"] unsafe
                        extern "C" fn _post_return_balance_to_string(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_balance_to_string::<$ty >
                        (arg0) } #[export_name =
                        "orange:commons/mappers#string-to-balance"] unsafe extern "C" fn
                        export_string_to_balance(arg0 : * mut u8, arg1 : usize,) -> * mut
                        u8 { $($path_to_types)*:: _export_string_to_balance_cabi::<$ty >
                        (arg0, arg1) } #[export_name =
                        "cabi_post_orange:commons/mappers#string-to-balance"] unsafe
                        extern "C" fn _post_return_string_to_balance(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_string_to_balance::<$ty >
                        (arg0) } #[export_name =
                        "orange:commons/mappers#balance-to-stringified-array"] unsafe
                        extern "C" fn export_balance_to_stringified_array(arg0 : f32,
                        arg1 : * mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize,) ->
                        * mut u8 { $($path_to_types)*::
                        _export_balance_to_stringified_array_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4) } #[export_name =
                        "cabi_post_orange:commons/mappers#balance-to-stringified-array"]
                        unsafe extern "C" fn
                        _post_return_balance_to_stringified_array(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_balance_to_stringified_array::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_orange_commons_mappers_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 52]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 52],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod rating_response_builder {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type RatingResponse = super::super::super::super::orange::commons::types::RatingResponse;
                pub type AgentIdentification = super::super::super::super::orange::commons::types::AgentIdentification;
                pub type BuilderHandle = u32;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_create_builder_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::create_builder();
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_unit_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::unit(arg0 as u32, _rt::string_lift(bytes0));
                    _rt::as_i32(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_price_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::price(arg0 as u32, _rt::string_lift(bytes0));
                    _rt::as_i32(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_message_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::message(arg0 as u32, _rt::string_lift(bytes0));
                    _rt::as_i32(result1)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_authorized_cabi<T: Guest>(arg0: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::authorized(arg0 as u32);
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_not_authorized_cabi<T: Guest>(arg0: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::not_authorized(arg0 as u32);
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_next_agent_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::next_agent(
                        arg0 as u32,
                        super::super::super::super::orange::commons::types::AgentIdentification {
                            name: _rt::string_lift(bytes0),
                            partner_id: _rt::string_lift(bytes1),
                        },
                    );
                    _rt::as_i32(result2)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_build_cabi<T: Guest>(arg0: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::build(arg0 as u32);
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::orange::commons::types::RatingResponse {
                        authorization_status: authorization_status2,
                        billing_information: billing_information2,
                        next_agent: next_agent2,
                    } = result0;
                    let super::super::super::super::orange::commons::types::AuthorizationStatus {
                        code: code3,
                        key: key3,
                    } = authorization_status2;
                    *ptr1.add(0).cast::<u16>() = (_rt::as_i32(code3)) as u16;
                    let vec4 = (key3.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr1.add(8).cast::<usize>() = len4;
                    *ptr1.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                    let super::super::super::super::orange::commons::types::BillingInformation {
                        price: price5,
                        unit: unit5,
                        messages: messages5,
                    } = billing_information2;
                    let vec6 = (price5.into_bytes()).into_boxed_slice();
                    let ptr6 = vec6.as_ptr().cast::<u8>();
                    let len6 = vec6.len();
                    ::core::mem::forget(vec6);
                    *ptr1.add(16).cast::<usize>() = len6;
                    *ptr1.add(12).cast::<*mut u8>() = ptr6.cast_mut();
                    let vec7 = (unit5.into_bytes()).into_boxed_slice();
                    let ptr7 = vec7.as_ptr().cast::<u8>();
                    let len7 = vec7.len();
                    ::core::mem::forget(vec7);
                    *ptr1.add(24).cast::<usize>() = len7;
                    *ptr1.add(20).cast::<*mut u8>() = ptr7.cast_mut();
                    let vec9 = messages5;
                    let len9 = vec9.len();
                    let layout9 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec9.len() * 8,
                        4,
                    );
                    let result9 = if layout9.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout9).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout9);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec9.into_iter().enumerate() {
                        let base = result9.add(i * 8);
                        {
                            let vec8 = (e.into_bytes()).into_boxed_slice();
                            let ptr8 = vec8.as_ptr().cast::<u8>();
                            let len8 = vec8.len();
                            ::core::mem::forget(vec8);
                            *base.add(4).cast::<usize>() = len8;
                            *base.add(0).cast::<*mut u8>() = ptr8.cast_mut();
                        }
                    }
                    *ptr1.add(32).cast::<usize>() = len9;
                    *ptr1.add(28).cast::<*mut u8>() = result9;
                    let super::super::super::super::orange::commons::types::AgentIdentification {
                        name: name10,
                        partner_id: partner_id10,
                    } = next_agent2;
                    let vec11 = (name10.into_bytes()).into_boxed_slice();
                    let ptr11 = vec11.as_ptr().cast::<u8>();
                    let len11 = vec11.len();
                    ::core::mem::forget(vec11);
                    *ptr1.add(40).cast::<usize>() = len11;
                    *ptr1.add(36).cast::<*mut u8>() = ptr11.cast_mut();
                    let vec12 = (partner_id10.into_bytes()).into_boxed_slice();
                    let ptr12 = vec12.as_ptr().cast::<u8>();
                    let len12 = vec12.len();
                    ::core::mem::forget(vec12);
                    *ptr1.add(48).cast::<usize>() = len12;
                    *ptr1.add(44).cast::<*mut u8>() = ptr12.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_build<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(4).cast::<*mut u8>();
                    let l1 = *arg0.add(8).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(12).cast::<*mut u8>();
                    let l3 = *arg0.add(16).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                    let l4 = *arg0.add(20).cast::<*mut u8>();
                    let l5 = *arg0.add(24).cast::<usize>();
                    _rt::cabi_dealloc(l4, l5, 1);
                    let l6 = *arg0.add(28).cast::<*mut u8>();
                    let l7 = *arg0.add(32).cast::<usize>();
                    let base10 = l6;
                    let len10 = l7;
                    for i in 0..len10 {
                        let base = base10.add(i * 8);
                        {
                            let l8 = *base.add(0).cast::<*mut u8>();
                            let l9 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l8, l9, 1);
                        }
                    }
                    _rt::cabi_dealloc(base10, len10 * 8, 4);
                    let l11 = *arg0.add(36).cast::<*mut u8>();
                    let l12 = *arg0.add(40).cast::<usize>();
                    _rt::cabi_dealloc(l11, l12, 1);
                    let l13 = *arg0.add(44).cast::<*mut u8>();
                    let l14 = *arg0.add(48).cast::<usize>();
                    _rt::cabi_dealloc(l13, l14, 1);
                }
                pub trait Guest {
                    fn create_builder() -> BuilderHandle;
                    fn unit(handle: BuilderHandle, value: _rt::String) -> BuilderHandle;
                    fn price(handle: BuilderHandle, value: _rt::String) -> BuilderHandle;
                    fn message(
                        handle: BuilderHandle,
                        value: _rt::String,
                    ) -> BuilderHandle;
                    fn authorized(handle: BuilderHandle) -> BuilderHandle;
                    fn not_authorized(handle: BuilderHandle) -> BuilderHandle;
                    fn next_agent(
                        handle: BuilderHandle,
                        value: AgentIdentification,
                    ) -> BuilderHandle;
                    fn build(handle: BuilderHandle) -> RatingResponse;
                }
                #[doc(hidden)]
                macro_rules! __export_orange_commons_rating_response_builder_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "orange:commons/rating-response-builder#create-builder"] unsafe
                        extern "C" fn export_create_builder() -> i32 {
                        $($path_to_types)*:: _export_create_builder_cabi::<$ty > () }
                        #[export_name = "orange:commons/rating-response-builder#unit"]
                        unsafe extern "C" fn export_unit(arg0 : i32, arg1 : * mut u8,
                        arg2 : usize,) -> i32 { $($path_to_types)*::
                        _export_unit_cabi::<$ty > (arg0, arg1, arg2) } #[export_name =
                        "orange:commons/rating-response-builder#price"] unsafe extern "C"
                        fn export_price(arg0 : i32, arg1 : * mut u8, arg2 : usize,) ->
                        i32 { $($path_to_types)*:: _export_price_cabi::<$ty > (arg0,
                        arg1, arg2) } #[export_name =
                        "orange:commons/rating-response-builder#message"] unsafe extern
                        "C" fn export_message(arg0 : i32, arg1 : * mut u8, arg2 : usize,)
                        -> i32 { $($path_to_types)*:: _export_message_cabi::<$ty > (arg0,
                        arg1, arg2) } #[export_name =
                        "orange:commons/rating-response-builder#authorized"] unsafe
                        extern "C" fn export_authorized(arg0 : i32,) -> i32 {
                        $($path_to_types)*:: _export_authorized_cabi::<$ty > (arg0) }
                        #[export_name =
                        "orange:commons/rating-response-builder#not-authorized"] unsafe
                        extern "C" fn export_not_authorized(arg0 : i32,) -> i32 {
                        $($path_to_types)*:: _export_not_authorized_cabi::<$ty > (arg0) }
                        #[export_name =
                        "orange:commons/rating-response-builder#next-agent"] unsafe
                        extern "C" fn export_next_agent(arg0 : i32, arg1 : * mut u8, arg2
                        : usize, arg3 : * mut u8, arg4 : usize,) -> i32 {
                        $($path_to_types)*:: _export_next_agent_cabi::<$ty > (arg0, arg1,
                        arg2, arg3, arg4) } #[export_name =
                        "orange:commons/rating-response-builder#build"] unsafe extern "C"
                        fn export_build(arg0 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_build_cabi::<$ty > (arg0) } #[export_name =
                        "cabi_post_orange:commons/rating-response-builder#build"] unsafe
                        extern "C" fn _post_return_build(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_build::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_orange_commons_rating_response_builder_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 52]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 52],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod some_builder {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct SomeItem {
                    pub item: _rt::String,
                }
                impl ::core::fmt::Debug for SomeItem {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("SomeItem").field("item", &self.item).finish()
                    }
                }
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct Builder {
                    handle: _rt::Resource<Builder>,
                }
                type _BuilderRep<T> = Option<T>;
                impl Builder {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Builder`.
                    pub fn new<T: GuestBuilder>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _BuilderRep<T> = Some(val);
                        let ptr: *mut _BuilderRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestBuilder>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestBuilder>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestBuilder>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _BuilderRep<T>);
                    }
                    fn as_ptr<T: GuestBuilder>(&self) -> *mut _BuilderRep<T> {
                        Builder::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Builder`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct BuilderBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Builder>,
                }
                impl<'a> BuilderBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestBuilder>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _BuilderRep<T> {
                        Builder::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Builder {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]orange:commons/some-builder"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]builder"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_builder_cabi<T: GuestBuilder>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = Builder::new(
                        T::new(_rt::Vec::from_raw_parts(arg0.cast(), len0, len0)),
                    );
                    (result1).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_builder_some_value_cabi<T: GuestBuilder>(
                    arg0: *mut u8,
                    arg1: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::some_value(
                        BuilderBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Some(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        None => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_builder_generate_cabi<T: GuestBuilder>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::generate(
                        BuilderBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let SomeItem { item: item2 } = result0;
                    let vec3 = (item2.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(4).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_builder_generate<T: GuestBuilder>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    type Builder: GuestBuilder;
                }
                pub trait GuestBuilder: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]orange:commons/some-builder"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]builder"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]orange:commons/some-builder"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]builder"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new(init: _rt::Vec<u8>) -> Self;
                    fn some_value(&self, value: u32) -> Option<Builder>;
                    fn generate(&self) -> SomeItem;
                }
                #[doc(hidden)]
                macro_rules! __export_orange_commons_some_builder_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "orange:commons/some-builder#[constructor]builder"] unsafe extern
                        "C" fn export_constructor_builder(arg0 : * mut u8, arg1 : usize,)
                        -> i32 { $($path_to_types)*::
                        _export_constructor_builder_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Builder > (arg0, arg1) } #[export_name =
                        "orange:commons/some-builder#[method]builder.some-value"] unsafe
                        extern "C" fn export_method_builder_some_value(arg0 : * mut u8,
                        arg1 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_method_builder_some_value_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Builder > (arg0, arg1) }
                        #[export_name =
                        "orange:commons/some-builder#[method]builder.generate"] unsafe
                        extern "C" fn export_method_builder_generate(arg0 : * mut u8,) ->
                        * mut u8 { $($path_to_types)*::
                        _export_method_builder_generate_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Builder > (arg0) } #[export_name =
                        "cabi_post_orange:commons/some-builder#[method]builder.generate"]
                        unsafe extern "C" fn _post_return_method_builder_generate(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_builder_generate::<<$ty as
                        $($path_to_types)*:: Guest >::Builder > (arg0) } const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "orange:commons/some-builder#[dtor]builder"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Builder::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Builder > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_orange_commons_some_builder_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod commons {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Balance = super::super::super::super::orange::commons::types::Balance;
                pub type UsageProofRequest = super::super::super::super::orange::commons::types::UsageProofRequest;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_create_balance_cabi<T: Guest>(
                    arg0: f32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::create_balance(
                        arg0,
                        _rt::string_lift(bytes0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let super::super::super::super::orange::commons::types::Balance {
                        count: count4,
                        unit: unit4,
                        party_id: party_id4,
                    } = result2;
                    *ptr3.add(0).cast::<f32>() = _rt::as_f32(count4);
                    let vec5 = (unit4.into_bytes()).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr3.add(8).cast::<usize>() = len5;
                    *ptr3.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                    let vec6 = (party_id4.into_bytes()).into_boxed_slice();
                    let ptr6 = vec6.as_ptr().cast::<u8>();
                    let len6 = vec6.len();
                    ::core::mem::forget(vec6);
                    *ptr3.add(16).cast::<usize>() = len6;
                    *ptr3.add(12).cast::<*mut u8>() = ptr6.cast_mut();
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_create_balance<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(4).cast::<*mut u8>();
                    let l1 = *arg0.add(8).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(12).cast::<*mut u8>();
                    let l3 = *arg0.add(16).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_generate_rating_proof_cabi<T: Guest>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let len2 = l1;
                    let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
                    let l3 = *arg0.add(8).cast::<*mut u8>();
                    let l4 = *arg0.add(12).cast::<usize>();
                    let len5 = l4;
                    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                    let l6 = *arg0.add(16).cast::<*mut u8>();
                    let l7 = *arg0.add(20).cast::<usize>();
                    let base17 = l6;
                    let len17 = l7;
                    let mut result17 = _rt::Vec::with_capacity(len17);
                    for i in 0..len17 {
                        let base = base17.add(i * 24);
                        let e17 = {
                            let l8 = *base.add(0).cast::<*mut u8>();
                            let l9 = *base.add(4).cast::<usize>();
                            let len10 = l9;
                            let bytes10 = _rt::Vec::from_raw_parts(
                                l8.cast(),
                                len10,
                                len10,
                            );
                            let l11 = *base.add(8).cast::<*mut u8>();
                            let l12 = *base.add(12).cast::<usize>();
                            let len13 = l12;
                            let bytes13 = _rt::Vec::from_raw_parts(
                                l11.cast(),
                                len13,
                                len13,
                            );
                            let l14 = *base.add(16).cast::<*mut u8>();
                            let l15 = *base.add(20).cast::<usize>();
                            let len16 = l15;
                            let bytes16 = _rt::Vec::from_raw_parts(
                                l14.cast(),
                                len16,
                                len16,
                            );
                            super::super::super::super::orange::commons::types::UsageCharacteristic {
                                name: _rt::string_lift(bytes10),
                                value: _rt::string_lift(bytes13),
                                value_type: _rt::string_lift(bytes16),
                            }
                        };
                        result17.push(e17);
                    }
                    _rt::cabi_dealloc(base17, len17 * 24, 4);
                    let l18 = *arg0.add(24).cast::<*mut u8>();
                    let l19 = *arg0.add(28).cast::<usize>();
                    let len20 = l19;
                    let bytes20 = _rt::Vec::from_raw_parts(l18.cast(), len20, len20);
                    let l21 = *arg0.add(32).cast::<*mut u8>();
                    let l22 = *arg0.add(36).cast::<usize>();
                    let len23 = l22;
                    let bytes23 = _rt::Vec::from_raw_parts(l21.cast(), len23, len23);
                    let l24 = *arg0.add(40).cast::<*mut u8>();
                    let l25 = *arg0.add(44).cast::<usize>();
                    let len26 = l25;
                    let bytes26 = _rt::Vec::from_raw_parts(l24.cast(), len26, len26);
                    let l27 = *arg0.add(48).cast::<*mut u8>();
                    let l28 = *arg0.add(52).cast::<usize>();
                    let len29 = l28;
                    let bytes29 = _rt::Vec::from_raw_parts(l27.cast(), len29, len29);
                    let l30 = *arg0.add(56).cast::<*mut u8>();
                    let l31 = *arg0.add(60).cast::<usize>();
                    let len32 = l31;
                    let bytes32 = _rt::Vec::from_raw_parts(l30.cast(), len32, len32);
                    let l33 = *arg0.add(64).cast::<*mut u8>();
                    let l34 = *arg0.add(68).cast::<usize>();
                    let len35 = l34;
                    let bytes35 = _rt::Vec::from_raw_parts(l33.cast(), len35, len35);
                    let result36 = T::generate_rating_proof(super::super::super::super::orange::commons::types::UsageProofRequest {
                        party_id: _rt::string_lift(bytes2),
                        usage_id: _rt::string_lift(bytes5),
                        usage_characteristic_list: result17,
                        rating: _rt::string_lift(bytes20),
                        usage_date: _rt::string_lift(bytes23),
                        offer_id: _rt::string_lift(bytes26),
                        description: _rt::string_lift(bytes29),
                        usage_type: _rt::string_lift(bytes32),
                        product_name: _rt::string_lift(bytes35),
                    });
                    _rt::cabi_dealloc(arg0, 72, 4);
                    let ptr37 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec38 = (result36.into_bytes()).into_boxed_slice();
                    let ptr38 = vec38.as_ptr().cast::<u8>();
                    let len38 = vec38.len();
                    ::core::mem::forget(vec38);
                    *ptr37.add(4).cast::<usize>() = len38;
                    *ptr37.add(0).cast::<*mut u8>() = ptr38.cast_mut();
                    ptr37
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_generate_rating_proof<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn create_balance(
                        count: f32,
                        unit: _rt::String,
                        party_id: _rt::String,
                    ) -> Balance;
                    fn generate_rating_proof(request: UsageProofRequest) -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_orange_commons_commons_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "orange:commons/commons#create-balance"] unsafe extern "C" fn
                        export_create_balance(arg0 : f32, arg1 : * mut u8, arg2 : usize,
                        arg3 : * mut u8, arg4 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_create_balance_cabi::<$ty > (arg0,
                        arg1, arg2, arg3, arg4) } #[export_name =
                        "cabi_post_orange:commons/commons#create-balance"] unsafe extern
                        "C" fn _post_return_create_balance(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_create_balance::<$ty > (arg0)
                        } #[export_name = "orange:commons/commons#generate-rating-proof"]
                        unsafe extern "C" fn export_generate_rating_proof(arg0 : * mut
                        u8,) -> * mut u8 { $($path_to_types)*::
                        _export_generate_rating_proof_cabi::<$ty > (arg0) } #[export_name
                        = "cabi_post_orange:commons/commons#generate-rating-proof"]
                        unsafe extern "C" fn _post_return_generate_rating_proof(arg0 : *
                        mut u8,) { $($path_to_types)*::
                        __post_return_generate_rating_proof::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_orange_commons_commons_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 20],
                );
            }
            #[allow(dead_code, clippy::all)]
            pub mod error_types {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                #[doc(hidden)]
                macro_rules! __export_orange_commons_error_types_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_orange_commons_error_types_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::alloc;
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
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::boxed::Box;
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
macro_rules! __export_commons_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::orange::commons::mappers::__export_orange_commons_mappers_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::orange::commons::mappers);
        $($path_to_types_root)*::
        exports::orange::commons::rating_response_builder::__export_orange_commons_rating_response_builder_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::orange::commons::rating_response_builder); $($path_to_types_root)*::
        exports::orange::commons::some_builder::__export_orange_commons_some_builder_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::orange::commons::some_builder);
        $($path_to_types_root)*::
        exports::orange::commons::commons::__export_orange_commons_commons_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::orange::commons::commons);
        $($path_to_types_root)*::
        exports::orange::commons::error_types::__export_orange_commons_error_types_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::orange::commons::error_types);
    };
}
#[doc(inline)]
pub(crate) use __export_commons_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:orange:commons:commons-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2412] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe8\x11\x01A\x02\x01\
A\x11\x01B\"\x01r\x03\x05countv\x04units\x08party-ids\x04\0\x07balance\x03\0\0\x01\
r\x01\x05valid\x7f\x04\0\x13validation-response\x03\0\x02\x01r\x02\x04names\x0ap\
artner-ids\x04\0\x14agent-identification\x03\0\x04\x01r\x03\x04names\x05values\x0a\
value-types\x04\0\x14usage-characteristic\x03\0\x06\x01p\x07\x01r\x09\x08party-i\
ds\x08usage-ids\x19usage-characteristic-list\x08\x06ratings\x0ausage-dates\x08of\
fer-ids\x0bdescriptions\x0ausage-types\x0cproduct-names\x04\0\x13usage-proof-req\
uest\x03\0\x09\x01r\x01\x19usage-characteristic-list\x08\x04\0\x05usage\x03\0\x0b\
\x01r\x02\x05usage\x0c\x0fatomic-offer-ids\x04\0\x14get-children-request\x03\0\x0d\
\x01r\x02\x0eidentification\x05\x05usage\x0c\x04\0\x05agent\x03\0\x0f\x01p\x10\x01\
r\x01\x06agents\x11\x04\0\x0aagent-list\x03\0\x12\x01r\x03\x08producers\x04units\
\x05prices\x04\0\x0drating-record\x03\0\x14\x01p\x15\x01r\x06\x0bcustomer-ids\x08\
agent-ids\x08languages\x08offer-ids\x05usage\x0c\x0erating-history\x16\x04\0\x0e\
rating-request\x03\0\x17\x01r\x03\x0erating-request\x18\x09client-ips\x0eclient-\
countrys\x04\0\x12validation-request\x03\0\x19\x01ps\x01r\x03\x05prices\x04units\
\x08messages\x1b\x04\0\x13billing-information\x03\0\x1c\x01r\x02\x04code{\x03key\
s\x04\0\x14authorization-status\x03\0\x1e\x01r\x03\x14authorization-status\x1f\x13\
billing-information\x1d\x0anext-agent\x05\x04\0\x0frating-response\x03\0\x20\x03\
\0\x14orange:commons/types\x05\0\x02\x03\0\0\x0erating-request\x02\x03\0\0\x0fra\
ting-response\x02\x03\0\0\x07balance\x01B\x15\x02\x03\x02\x01\x01\x04\0\x0eratin\
g-request\x03\0\0\x02\x03\x02\x01\x02\x04\0\x0frating-response\x03\0\x02\x02\x03\
\x02\x01\x03\x04\0\x07balance\x03\0\x04\x01@\x01\x07request\x01\0s\x04\0\x18rati\
ng-request-to-string\x01\x06\x01@\x01\x05values\0\x01\x04\0\x18string-to-rating-\
request\x01\x07\x01@\x01\x08response\x03\0s\x04\0\x19rating-response-to-string\x01\
\x08\x01@\x01\x05values\0\x03\x04\0\x19string-to-rating-response\x01\x09\x01@\x01\
\x08response\x05\0s\x04\0\x11balance-to-string\x01\x0a\x01@\x01\x05values\0\x05\x04\
\0\x11string-to-balance\x01\x0b\x01p}\x01@\x01\x08response\x05\0\x0c\x04\0\x1cba\
lance-to-stringified-array\x01\x0d\x04\0\x16orange:commons/mappers\x05\x04\x02\x03\
\0\0\x14agent-identification\x01B\x13\x02\x03\x02\x01\x02\x04\0\x0frating-respon\
se\x03\0\0\x02\x03\x02\x01\x05\x04\0\x14agent-identification\x03\0\x02\x01y\x04\0\
\x0ebuilder-handle\x03\0\x04\x01@\0\0\x05\x04\0\x0ecreate-builder\x01\x06\x01@\x02\
\x06handle\x05\x05values\0\x05\x04\0\x04unit\x01\x07\x04\0\x05price\x01\x07\x04\0\
\x07message\x01\x07\x01@\x01\x06handle\x05\0\x05\x04\0\x0aauthorized\x01\x08\x04\
\0\x0enot-authorized\x01\x08\x01@\x02\x06handle\x05\x05value\x03\0\x05\x04\0\x0a\
next-agent\x01\x09\x01@\x01\x06handle\x05\0\x01\x04\0\x05build\x01\x0a\x04\0&ora\
nge:commons/rating-response-builder\x05\x06\x01B\x0d\x01r\x01\x04items\x04\0\x09\
some-item\x03\0\0\x04\0\x07builder\x03\x01\x01p}\x01i\x02\x01@\x01\x04init\x03\0\
\x04\x04\0\x14[constructor]builder\x01\x05\x01h\x02\x01k\x04\x01@\x02\x04self\x06\
\x05valuey\0\x07\x04\0\x1a[method]builder.some-value\x01\x08\x01@\x01\x04self\x06\
\0\x01\x04\0\x18[method]builder.generate\x01\x09\x04\0\x1borange:commons/some-bu\
ilder\x05\x07\x02\x03\0\0\x13usage-proof-request\x01B\x08\x02\x03\x02\x01\x03\x04\
\0\x07balance\x03\0\0\x02\x03\x02\x01\x08\x04\0\x13usage-proof-request\x03\0\x02\
\x01@\x03\x05countv\x04units\x08party-ids\0\x01\x04\0\x0ecreate-balance\x01\x04\x01\
@\x01\x07request\x03\0s\x04\0\x15generate-rating-proof\x01\x05\x04\0\x16orange:c\
ommons/commons\x05\x09\x01B\x08\x01r\x01\x07messages\x04\0\x10validation-error\x03\
\0\0\x01r\x01\x07messages\x04\0\x0busage-error\x03\0\x02\x01r\x01\x07messages\x04\
\0\x0bother-error\x03\0\x04\x01q\x03\x0avalidation\x01\x01\0\x05usage\x01\x03\0\x05\
other\x01\x05\0\x04\0\x0dgeneric-error\x03\0\x06\x04\0\x1aorange:commons/error-t\
ypes\x05\x0a\x04\0\x1corange:commons/commons-world\x04\0\x0b\x13\x01\0\x0dcommon\
s-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.22\
0.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
