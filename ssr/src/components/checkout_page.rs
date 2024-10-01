use leptos::*;

use crate::{
    state::{
        canisters::authenticated_canisters,
        checkout_state::{CheckoutState, CheckoutUser},
    },
    utils::{
        input::{non_empty_string_validator, InputBox},
        time::get_day_month_time,
    },
};

#[component]
pub fn CheckoutPage() -> impl IntoView {
    let checkout = CheckoutState::get();

    match checkout.selected_car.get() {
        Some(car) => {
            let user: CheckoutUser = expect_context();
            let checkout_state: CheckoutState = expect_context();
            let cans_res = authenticated_canisters();

            let fetch_car_details = create_resource(
                move || (checkout.start_time.get(), checkout.end_time.get()),
                move |_| {
                    let cans_res = cans_res.clone();
                    async move {
                        let cans = cans_res.wait_untracked().await.unwrap();
                        let cans = cans.canisters().unwrap();
                        let car_details = cans
                            .backend()
                            .await
                            .get_car_details(
                                car.id,
                                checkout_state.start_time.get_untracked().unwrap(),
                                checkout.end_time.get_untracked().unwrap(),
                            )
                            .await;

                        match car_details {
                            Ok(cars) if cars.is_some() => Ok(cars.unwrap()),
                            _ => Err(format!("Some error occuered while fetching car details")),
                        }
                    }
                },
            );

            let update_name = move |value| user.user.update(|f| f.name = Some(value));
            let update_age = move |value: String| {
                user.user
                    .update(|f| f.age = Some(value.parse::<u8>().unwrap().into()))
            };
            let update_email = move |value| user.user.update(|f| f.email = Some(value));
            let update_code = move |value| user.user.update(|f| f.country_code = Some(value));
            let update_mobile = move |value| user.user.update(|f| f.mobile_number = Some(value));

            // let detals = car.clone();
            view! {
                <div class="container mx-auto py-8 px-4 lg:px-8">
                    <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                    <h1 class="text-2xl lg:text-3xl font-bold mb-6">{format!("{} {}",car.make, car.model )}</h1>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                        // <!-- Left Section - Billing Info, Payment, and Confirmation -->
                        <div class="lg:col-span-2">
                            // <!-- Billing Info -->
                            <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-bold">"Billing Info"</h2>
                                    <span class="text-sm text-gray-500">"Step 1 of 3"</span>
                                </div>
                                <p class="text-sm text-gray-400 mb-4">"Please enter your billing info"</p>

                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                    // <input type="text" placeholder="Full Name" class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().name.map_or("".into(), |f| f ) heading="Full Name"  placeholder="Full Name" updater=update_name validator=move|val|non_empty_string_validator(val) />
                                    // <input type="text" placeholder="Age" class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().age.map_or("".into(), |f| f.to_string() ) heading="Age" placeholder="Age" updater=update_age validator=move|val|non_empty_string_validator(val) input_type="number" />
                                    // <input type="email" placeholder="Email ID" class="border rounded-lg px-4 py-2"/>
                                    <InputBox initial_value=user.user.get_untracked().email.map_or("".into(), |f| f.to_string() ) heading="Email" placeholder="Email" updater=update_email validator=move|val|non_empty_string_validator(val) input_type="email" />

                                    // <input type="text" placeholder="Pan Card No." class="border rounded-lg px-4 py-2"/>
                                    // <input type="text" placeholder="Adhar Card No." class="border rounded-lg px-4 py-2"/>
                                </div>
                                <div class="grid grid-cols-2 space-x-2">
                                    // <input type="text" placeholder="Country Code" class="border rounded-lg px-4 py-2 w-1/3"/>
                                    <InputBox initial_value=user.user.get_untracked().country_code.map_or("".into(), |f| f.to_string() ) heading="Country Code" placeholder="Country Code" updater=update_code validator=move|val|non_empty_string_validator(val) input_type="text" style="col-span-1".into() />
                                    // <input type="text" placeholder="Mobile Number" class="border rounded-lg px-4 py-2 w-2/3"/>
                                    <InputBox initial_value=user.user.get_untracked().mobile_number.map_or("".into(), |f| f.to_string() ) heading="Mobile Number" placeholder="Mobile Number" updater=update_mobile validator=move|val|non_empty_string_validator(val) input_type="number" style="col-span-2".into() />
                                </div>
                            </div>

                            // <!-- Payment Method -->
                            // <PaymentMethod />

                            // <!-- Confirmation -->
                            <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-bold">"Confirmation"</h2>
                                    <span class="text-sm text-gray-500">"Step 2 of 3"</span>
                                </div>
                                <p class="text-sm text-gray-400 mb-4">"We are getting to the end. Just a few clicks and your rental is ready!"</p>

                                <div class="flex items-center mb-6">
                                    <input type="checkbox" id="terms" class="mr-2"/>
                                    <label for="terms" class="text-gray-600 text-sm">"I agree with our terms and conditions and privacy policy."</label>
                                </div>

                                <button class="w-full bg-green-500 text-white py-3 rounded-lg font-bold hover:bg-green-600">
                                    "Rent Now"
                                </button>
                            </div>

                            // <!-- Security Info -->
                            <div class="flex items-center text-gray-600 text-sm">
                                <img src="/icons/security.svg" class="h-6 w-6 mr-2 text-blue-500"  />

                                <span>"All your data is safe. We are using the most advanced security to provide you the best experience ever."</span>
                            </div>
                        </div>

                        // <!-- Right Section - Rental Summary -->
                        <Suspense>
                        {
                            move || {
                                fetch_car_details.get().map(|res| {
                                    match res {
                                        Ok(avail) => {
                                            let car = avail.details;
                                            let available = avail.available;
                                            view! {
                                                <div class="bg-white rounded-lg shadow-lg p-6">
                            <h2 class="text-xl font-bold mb-4">"Rental Summary"</h2>

                            <div class="flex justify-between items-center mb-4">
                                <img src=car.default_image_url alt="Car image" class="w-20 h-16 rounded-lg"/>
                                <div>
                                    <h3 class="text-lg font-bold">{format!("{} {}", car.make, car.model.clone())}</h3>
                                    <p class="text-sm text-gray-400">"440+ Reviewer"</p>
                                </div>
                            </div>

                            <div class="text-sm text-gray-600 mb-4">
                                {
                                    match car.dropoff_location {
                                        Some(location) =>{ view! { <div class="flex justify-between items-center mb-2">
                                                                        <span>"Pick up Location"</span>
                                                                        <span>{ location.address}</span>
                                                                    </div> } }
                                    None => {view! { <div> </div> }}
                                    }
                                }

                                <div class="flex justify-between items-center mb-2">
                                    <span>"Date & Time"</span>
                                    <span>{get_day_month_time(checkout.start_time.get().unwrap())}</span>
                                </div>
                                <div class="flex justify-between items-center mb-2">
                                    <span>"Drop off Location"</span>
                                    <span>"Same Location"</span>
                                </div>
                                <div class="flex justify-between items-center mb-2">
                                    <span>"Date & Time"</span>
                                    <span>{get_day_month_time(checkout.end_time.get().unwrap())}</span>
                                </div>
                            </div>

                            {
                                match available {
                                    Some(a) => {
                                        view! {
                                        <div>
                                            // {a.customer_principal_id.to_text()}
                                            <div class="flex justify-between items-center mb-4">
                                            <span>"Subtotal"</span>
                                            <span>{a.total_amount}</span>
                                        </div>
                                        <div class="flex justify-between items-center mb-4">
                                            <span>"Tax"</span>
                                            <span>"$0.00"</span>
                                        </div>

                                        <div class="flex items-center mb-4">
                                            <input type="text" placeholder="Promo Code" class="border rounded-lg px-4 py-2 flex-1"/>
                                            <button class="bg-gray-200 text-gray-600 py-2 px-4 rounded-lg ml-2">"Apply"</button>
                                        </div>

                                        <div class="flex justify-between items-center mb-4 font-bold text-lg">
                                            <span>"Total Rental Price"</span>
                                            <span>{a.total_amount}</span>
                                        </div>

                                        <button class="w-full bg-green-500 text-white py-3 rounded-lg font-bold hover:bg-green-600">
                                            "Apply Now"
                                        </button>
                                            </div>
                                        }

                                    },
                                    None => view! {<div> </div>}
                                }
                            }


                        </div>
                                            }
                                        },
                                        Err(_) => {
                                            view! {
                                                <div></div>
                                            }
                                        } ,
                                    }


                                })
                            }
                        }

                        </Suspense>
                    </div>
                </div>
            }
        }
        None => {
            view! {
                <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                <h1 class="text-2xl lg:text-3xl font-bold mb-6">Select a car to proceed</h1>
                </div>
            }
        }
    }
}

#[component]
fn PaymentMethod() -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-bold">"Payment Method"</h2>
                                    <span class="text-sm text-gray-500">"Step 3 of 4"</span>
                                </div>

                                <div class="mb-4">
                                    <input type="radio" id="creditCard" name="payment" checked class="mr-2"/>
                                    <label for="creditCard" class="text-lg font-medium">"Credit Card"</label>
                                </div>
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
                                    <input type="text" placeholder="Card Number" class="border rounded-lg px-4 py-2"/>
                                    <input type="text" placeholder="Expiration Date (MM/YY)" class="border rounded-lg px-4 py-2"/>
                                    <input type="text" placeholder="Card Holder" class="border rounded-lg px-4 py-2"/>
                                    <input type="text" placeholder="CVC" class="border rounded-lg px-4 py-2"/>
                                </div>

                                <div class="mb-4">
                                    <input type="radio" id="paypal" name="payment" class="mr-2"/>
                                    <label for="paypal" class="text-lg font-medium">"PayPal"</label>
                                </div>

                                <div>
                                    <input type="radio" id="bitcoin" name="payment" class="mr-2"/>
                                    <label for="bitcoin" class="text-lg font-medium">"Bitcoin"</label>
                                </div>
        </div>
    }
}
