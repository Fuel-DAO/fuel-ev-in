// use leptos::*;
// use leptos_router::use_query_map;

// use crate::models::{RazorpayPayment, ReserveCarAPIRequest};

// #[component]
// pub fn Payment() -> impl IntoView {

//     // Extract razorpay_payment_id, razorpay_payment_link_id, razorpay_payment_link_reference_id, razorpay_payment_link_status from the URL using use_query
//     let query = use_query_map();
//     let razorpay_payment_id = query.get_untracked().get("razorpay_payment_id").cloned();
//     let razorpay_payment_link_id = query.get_untracked().get("razorpay_payment_link_id").cloned();
//     let razorpay_payment_link_reference_id = query.get_untracked().get("razorpay_payment_link_reference_id").cloned();
//     let razorpay_payment_link_status = query.get_untracked().get("razorpay_payment_link_status").cloned();

//     let create_resource = create_resource(||(), move |_| {
//         let razorpay_payment_id = razorpay_payment_id.clone();
//         let razorpay_payment_link_id = razorpay_payment_link_id.clone();
//         let razorpay_payment_link_reference_id = razorpay_payment_link_reference_id.clone();
//         let razorpay_payment_link_status = razorpay_payment_link_status.clone();
//         async move {
//             let razorpay_payment_id = razorpay_payment_id.ok_or( format!("Invalid Razorpay Payment id"))?;
//             let razorpay_payment_link_id = razorpay_payment_link_id.ok_or(  format!("Invalid Razorpay Payment link id"))?;
//             let razorpay_payment_link_reference_id = razorpay_payment_link_reference_id.ok_or( format!("Invalid Razorpay Payment link reference id"))?;
//             let razorpay_payment_link_status = razorpay_payment_link_status.ok_or( format!("Invalid Razorpay Payment link status"))?;

//             if razorpay_payment_link_status != "paid" {
//                 return Err("Payment not successful".to_string());
//             }

//             let payment = RazorpayPayment {
//                 payment_id: razorpay_payment_id.to_string(),
//                 payment_link_id: Some(razorpay_payment_link_id.to_string()),
//                 ref_id: razorpay_payment_link_reference_id.to_string(),
//             };

//             let reserve_car_api_request = ReserveCarAPIRequest {
//                 booking_id: razorpay_payment_link_reference_id.parse::<u64>().map_err(|f| format!("Invalid Ref key {:?}", f))?,
//                 payment,
//             };

//              reserve_car_api_request.reserve_car().await.map_err(|f| format!("Failed to reserve car {:?}", f))
//         }
//     });

    
//     view! {
//         <Suspense fallback=||view! {<div>Waiting for Payment confirmation</div>} >
//             {move || {
//                 create_resource
//                     .get()
//                     .map(|res| {
//                         match res {
//                             Ok(booking_id) => {
//                                 view! {
//                                     <div>
//                                         <div class="flex items-center justify-center  bg-gray-100">
//                                             <div class="bg-white p-2 rounded-lg shadow-lg text-center max-w-md w-full">
//                                                 // Success Icon
//                                                 <div class="flex justify-center">
//                                                     <svg
//                                                         xmlns="http://www.w3.org/2000/svg"
//                                                         width="72"
//                                                         height="72"
//                                                         fill="green"
//                                                         viewBox="0 0 24 24"
//                                                     >
//                                                         <path d="M12 0C5.383 0 0 5.383 0 12c0 6.616 5.383 12 12 12 6.616 0 12-5.384 12-12 0-6.617-5.384-12-12-12zm-1.015 17.828l-4.813-4.828 1.427-1.413 3.4 3.413 7.399-7.428 1.414 1.399-8.827 8.857z" />
//                                                     </svg>
//                                                 </div>

//                                                 // Success Message
//                                                 <h2 class="text-2xl font-semibold text-gray-800 mb-2">
//                                                     "We've sent you a booking confirmation on your email."
//                                                 </h2>
//                                                 <p class="text-gray-600 mb-2">
//                                                     "We'll communicate further instructions on the same email"
//                                                 </p>

//                                                 // Booking Reference
//                                                 <div class="bg-gray-200 py-3 px-4 rounded-lg inline-block text-lg font-medium text-gray-700">
//                                                     Booking Ref. {booking_id}
//                                                 </div>
//                                             </div>
//                                         </div>
//                                         <div class="flex justify-center p-2">
//                                             <a
//                                                 href="/search"
//                                                 class="w-full  text-center bg-green-500 text-white px-3 py-3 rounded-lg font-bold disabled:text-neutral-500 disabled:bg-primary-500/30"
//                                             >
//                                                 Back to Search
//                                             </a>
//                                         </div>
//                                     </div>
//                                 }
//                             }
//                             Err(e) => {
//                                 view! {
//                                     <div>
//                                         <h1>Payment Failed</h1>
//                                         <p>{e}</p>
//                                     </div>
//                                 }
//                             }
//                         }
//                     })
//             }}

//         </Suspense>
//     }
// }


use leptos::*;
use leptos_router::use_query_map;

use crate::models::{RazorpayPayment, ReserveCarAPIRequest};

#[component]
pub fn Payment() -> impl IntoView {
    // Extract query parameters from the URL
    let query = use_query_map();
    let razorpay_payment_id = query.get_untracked().get("razorpay_payment_id").cloned();
    let razorpay_payment_link_id = query.get_untracked().get("razorpay_payment_link_id").cloned();
    let razorpay_payment_link_reference_id = query.get_untracked().get("razorpay_payment_link_reference_id").cloned();
    let razorpay_payment_link_status = query.get_untracked().get("razorpay_payment_link_status").cloned();

    // Create resource to handle the payment process
    let create_resource = create_resource(|| (), move |_| {
        let razorpay_payment_id = razorpay_payment_id.clone();
        let razorpay_payment_link_id = razorpay_payment_link_id.clone();
        let razorpay_payment_link_reference_id = razorpay_payment_link_reference_id.clone();
        let razorpay_payment_link_status = razorpay_payment_link_status.clone();
        async move {
            let razorpay_payment_id = razorpay_payment_id.ok_or("Invalid Razorpay Payment id".to_string())?;
            let razorpay_payment_link_id = razorpay_payment_link_id.ok_or("Invalid Razorpay Payment link id".to_string())?;
            let razorpay_payment_link_reference_id = razorpay_payment_link_reference_id.ok_or("Invalid Razorpay Payment link reference id".to_string())?;
            let razorpay_payment_link_status = razorpay_payment_link_status.ok_or("Invalid Razorpay Payment link status".to_string())?;

            if razorpay_payment_link_status != "paid" {
                return Err("Payment not successful".to_string());
            }

            let payment = RazorpayPayment {
                payment_id: razorpay_payment_id,
                payment_link_id: Some(razorpay_payment_link_id),
                ref_id: razorpay_payment_link_reference_id.clone(),
            };

            let reserve_car_api_request = ReserveCarAPIRequest {
                booking_id: razorpay_payment_link_reference_id.parse::<u64>().map_err(|_| "Invalid Ref key".to_string())?,
                payment,
            };

            reserve_car_api_request.reserve_car().await.map_err(|f| format!("Failed to reserve car: {:?}", f))
        }
    });

    // Render the UI
    view! {
        <Suspense fallback=||view! {
            <div class="flex items-center justify-center h-screen">
                <div class="text-gray-500 text-center">
                    <div class="loader mb-4"></div>
                    "Waiting for Payment Confirmation..."
                </div>
            </div>
        }>
            {move || {
                create_resource.get().map(|res| match res {
                    Ok(booking_id) => {
                        view! {
                            <div class="flex items-center justify-center h-screen bg-gray-100">
                                <div class="bg-white p-6 rounded-lg shadow-lg text-center max-w-md w-full">
                                    <div class="flex justify-center mb-4">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="72" height="72" fill="green" viewBox="0 0 24 24">
                                            <path d="M12 0C5.383 0 0 5.383 0 12c0 6.616 5.383 12 12 12 6.616 0 12-5.384 12-12 0-6.617-5.384-12-12-12zm-1.015 17.828l-4.813-4.828 1.427-1.413 3.4 3.413 7.399-7.428 1.414 1.399-8.827 8.857z" />
                                        </svg>
                                    </div>
                                    <h2 class="text-2xl font-semibold text-gray-800 mb-4">
                                        "Payment Successful!"
                                    </h2>
                                    <p class="text-gray-600 mb-4">
                                        "We've sent you a booking confirmation to your email."
                                    </p>
                                    <div class="bg-gray-200 py-2 px-4 rounded-lg inline-block text-lg font-medium text-gray-700 mb-4">
                                        Booking Ref: {booking_id}
                                    </div>
                                    <a
                                        href="/search"
                                        class="block bg-green-500 text-white px-4 py-2 rounded-lg font-bold hover:bg-green-600 transition"
                                    >
                                        "Back to Search"
                                    </a>
                                </div>
                            </div>
                        }
                    }
                    Err(e) => {
                        view! {
                            <div class="flex items-center justify-center h-screen bg-gray-100">
                                <div class="bg-white p-6 rounded-lg shadow-lg text-center max-w-md w-full">
                                    <h2 class="text-2xl font-semibold text-red-600 mb-4">
                                        "Payment Failed"
                                    </h2>
                                    <p class="text-gray-600 mb-4">{e}</p>
                                    <a
                                        href="/search"
                                        class="block bg-gray-500 text-white px-4 py-2 rounded-lg font-bold hover:bg-gray-600 transition"
                                    >
                                        "Back to Search"
                                    </a>
                                </div>
                            </div>
                        }
                    }
                })
            }}
        </Suspense>
    }
}
