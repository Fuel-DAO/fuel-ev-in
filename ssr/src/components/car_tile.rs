use convert_case::{Case, Casing};
use leptos::*;
use leptos_router::use_navigate;
use server_fn::codec::Cbor;

use crate::{
    canister::backend::{Car, CarDetails, CarStatus},
    components::{spinners::Spinner, PopupOverlay},
    state::{
        canisters::{authenticated_canisters, CanistersAuthWire},
        checkout_state::CheckoutState,
    },
};

#[component]
pub fn SearchResult() -> impl IntoView {
    view! { <SearchResultInner /> }
}

#[server(input=Cbor)]
async fn fetch_all_cars(cans: CanistersAuthWire) -> Result<Vec<Car>, ServerFnError> {
    let backend = cans.canisters().unwrap();
    let backend = backend.backend().await;

    let cars = backend.list_all_cars().await;
    match cars {
        Ok(cars) => Ok(cars),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Some error occuered while fetching cars {:?}",
            e
        ))),
    }
}

#[component]
fn SearchResultInner() -> impl IntoView {
    let cans_res = authenticated_canisters();

    let search_resource = create_resource(
        || {},
        move |_| {
            let cans_res = cans_res.clone();

            async move {
                let cars = fetch_all_cars(cans_res.wait_untracked().await.unwrap()).await;
                match cars {
                    Ok(cars) => Ok(cars),
                    Err(e) => Err(format!("Some error occuered while fetching cars {:?}", e)),
                }
            }
        },
    );

    view! {
        <Suspense fallback=move || view! { <Spinner /> }>
            <div>
                {move || {
                    search_resource
                        .get()
                        .map(|res| {
                            match res {
                                Ok(cars) => {
                                    view! {
                                        <div>
                                            <ShowSearchResult cars />
                                        </div>
                                    }
                                }
                                Err(e) => {
                                    view! {
                                        <div>
                                            <span>{e}</span>
                                        </div>
                                    }
                                }
                            }
                        })
                }}
            </div>
        </Suspense>
    }
}

#[component]
fn ShowSearchResult(cars: Vec<Car>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 gap-4 p-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
            {cars.into_iter().map(|car| view! { <CarCard car /> }).collect_view()}
        </div>
    }
}

#[component]
pub fn CarCard(car: Car) -> impl IntoView {
    // let car_principal = car.principal;
    let car = car.details;
    let details = car.clone();
    let checkout = car.clone();

    let car_type = format!("{:?}", car.car_type);
    // let default_image_url = format!("{:?}", car.default_image_url);
    let transmission_type = format!("{:?}", car.transmission_type);
    let car_status = format!("{:?}", &car.status).to_case(Case::Title);
    let status = car.status.clone();
    let show_details_popup = create_rw_signal(false);
    let click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        show_details_popup.set(true);
    };

    view! {
        <div
            class="p-4 bg-white rounded-xl shadow-md"
            class:opacity-50=move || {
                format!("{:?}", status.clone()) != format!("{:?}", CarStatus::Available)
                    && format!("{:?}", status.clone()) != format!("{:?}", CarStatus::ComingSoon)
            }
        >
            <button on:click=click>
                // Car Name and Type
                <div class="mb-4 text-left">
                    <h1 class="text-xl font-bold text-gray-800">{car.model}</h1>
                    <p class="text-sm text-gray-400">{car_type}</p>
                </div>

                // Car Image
                <div class="mb-4">
                    <img src=car.default_image_url alt="Koenigsegg" class="w-full h-auto" />
                </div>

            </button>
            <PopupOverlay show=show_details_popup>
                <CarDetailsPopup car=details.clone() show_popup=show_details_popup />
            </PopupOverlay>

            // Car Details (Fuel, Transmission, Seating)
            <div class="flex justify-between items-center mb-4">
                <Show when=move || car.mileage.is_some()>
                    <div class="flex items-center">
                        <img src="/icons/mileage.svg" />
                        <span class="ml-1 text-sm text-gray-600">{car.mileage.unwrap()}Km</span>
                    </div>
                </Show>

                <div class="flex items-center">
                    <img src="/icons/car_transmission.svg" />
                    <span class="ml-1 text-sm text-gray-600">{transmission_type}</span>
                </div>

                <div class="flex items-center">
                    <img src="/icons/capacity.svg" />
                    <span class="ml-1 text-sm text-gray-600">{car.capacity}</span>
                </div>
            </div>

            // Pricing and Availability
            <div class="flex justify-between items-center">
                <div class="flex flex-col">
                    <div class="flex-1 text-lg font-bold text-gray-800">
                        "₹"{car.price_per_day} <span class="text-sm text-gray-400">"/ day"</span>
                    </div>
                    <Show when=move || car.price_per_day != car.current_price_per_day>
                        <div class="flex-1 text-sm text-gray-400 line-through">
                            "₹"{car.price_per_day}/day
                        </div>
                    </Show>
                </div>
                {match car.status {
                    CarStatus::Available => {
                        view! {
                            <div>
                                <button on:click=move |_| navigate_to_checkout(checkout.clone())>
                                    // <a href="/checkout">
                                    <div
                                        class="py-1 px-3 text-sm text-white rounded fill-green-500"
                                        style="background-color:#03B74B"
                                    >
                                        Rent Now
                                    </div>
                                // </a>
                                </button>
                            </div>
                        }
                    }
                    CarStatus::ComingSoon => {
                        view! {
                            <div class="py-1 px-3 text-sm text-green-500 rounded border border-green-500">
                                {car_status}
                            </div>
                        }
                    }
                    _ => {
                        view! {
                            <div
                                class="py-1 px-3 text-sm text-white rounded"
                                style="background-color:gray"
                            >
                                Not Available
                            </div>
                        }
                    }
                }}

            </div>
        </div>
    }
}

#[component]
fn CarDetailsPopup(#[prop(into)] show_popup: SignalSetter<bool>, car: CarDetails) -> impl IntoView {
    let details = car.clone();

    let car_type = format!("{:?}", car.car_type);
    let transmission_type = format!("{:?}", car.transmission_type);
    let car_status = format!("{:?}", &car.status).to_case(Case::Title);
    let fuel_type = format!("{:?}", car.fuel_type);
    view! {
        <div class="container mx-auto bg-gray-100">
            <div class="flex bg-gray-100 flex-cols-2">
                <div class="flex-1"></div>
                <button
                    class="px-4 text-gray-400 hover:text-gray-600 items-right"
                    on:click=move |_| {
                        show_popup.set(false);
                    }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="w-6 h-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>
            </div>
            <div class="grid grid-cols-1 gap-4 p-4 bg-gray-100 lg:grid-cols-2">

                // Left Section: Main car image and gallery
                <div class="flex overflow-hidden flex-col items-center p-6 max-w-full bg-white rounded-lg shadow-lg">
                    // Car promotion with large image and text
                    <div
                        class="relative mb-4 w-full h-64 bg-center bg-cover rounded-lg md:h-72"
                        style=format!("background-image: url('{}');", car.default_image_url)
                    >

                        // Text overlay on top of the image
                        <div class="flex absolute inset-0 flex-col justify-start items-start p-6 rounded-lg md:items-start">
                            <h2 class="mb-2 text-xl font-bold text-left text-white md:text-2xl">
                                "Sports car with the best design and acceleration"
                            </h2>
                            <p class="mb-4 text-sm text-left text-white md:text-base">
                                "Safety and comfort while driving a futuristic and elegant sports car"
                            </p>
                        </div>
                    </div>

                    // Gallery with additional car images
                    <div class="flex justify-center space-x-2 md:justify-start">
                        <img
                            src="/img/car-side.svg"
                            alt="Car thumbnail"
                            class="object-cover w-20 h-20 rounded-lg shadow-sm md:w-24 md:h-24"
                        />
                        <img
                            src="/img/interior.svg"
                            alt="Car interior"
                            class="object-cover w-20 h-20 rounded-lg shadow-sm md:w-24 md:h-24"
                        />
                        <img
                            src="/img/interior.svg"
                            alt="Car seats"
                            class="object-cover w-20 h-20 rounded-lg shadow-sm md:w-24 md:h-24"
                        />
                    </div>
                </div>

                // Right Section: Car details, price, and "Rent Now" button
                <div class="overflow-hidden p-6 max-w-full bg-white rounded-lg shadow-lg">
                    <div class="flex justify-between items-start mb-4">
                        <div class="w-full">
                            // Car name and rating
                            <h2 class="text-xl font-bold text-gray-800 md:text-2xl">
                                {format!("{} {}", car.make, car.model)}
                            </h2>
                            <div class="flex items-center">
                                <span class="mr-2 text-lg text-yellow-400">"★★★★★"</span>
                                <span class="text-sm text-gray-500">"440+ Reviewer"</span>
                            </div>
                        </div>
                    // Close button (optional, depending on the design)
                    </div>

                    // Car description
                    <p class="mb-4 text-sm text-gray-600">
                        "NISMO has become the embodiment of Nissan's outstanding performance, inspired by the most unforgiving proving ground, the 'race track'."
                    </p>

                    // Car specifications
                    <div class="grid grid-cols-2 gap-4 mb-6">
                        <div>
                            <span class="block text-gray-400">"Type Car"</span>
                            <span class="block text-gray-800">{car_type}</span>
                        </div>
                        <div>
                            <span class="block text-gray-400">"Capacity"</span>
                            <span class="block text-gray-800">{car.capacity}</span>
                        </div>
                        <div>
                            <span class="block text-gray-400">"Steering"</span>
                            <span class="block text-gray-800">{transmission_type}</span>
                        </div>
                        <div>
                            <span class="block text-gray-400">{fuel_type}</span>
                            <span class="block text-gray-800">{car.mileage}</span>
                        </div>
                    </div>

                    // Pricing and "Rent Now" button
                    <div class="flex justify-between items-center">
                        <div>
                            <span class="text-2xl font-bold text-gray-800">$80.00</span>
                            <span class="ml-2 text-sm text-gray-400 line-through">$100.00</span>
                            <span class="block text-sm text-gray-400">"/ days"</span>
                        </div>
                        <button class="py-2 px-4 rounded-lg">
                            {match car.status {
                                CarStatus::Available => {
                                    view! {
                                        <div>
                                            <button
                                                class="hover:bg-green-500"
                                                on:click=move |_| navigate_to_checkout(details.clone())
                                            >
                                                // <a href="/checkout">
                                                <div
                                                    class="py-1 px-3 w-24 text-sm text-white rounded fill-green-500"
                                                    style="background-color:#03B74B"
                                                >
                                                    Rent Now
                                                </div>
                                            // </a>
                                            </button>
                                        </div>
                                    }
                                }
                                CarStatus::ComingSoon => {
                                    view! {
                                        <div class="py-1 px-3 w-24 text-sm text-green-500 rounded border border-green-500">
                                            {car_status}
                                        </div>
                                    }
                                }
                                _ => {
                                    view! {
                                        <div
                                            class="py-1 px-3 w-24 text-sm text-white rounded"
                                            style="background-color:gray"
                                        >
                                            Not Available
                                        </div>
                                    }
                                }
                            }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn navigate_to_checkout(car: CarDetails) {
    CheckoutState::set(car);
    let nav = use_navigate();
    nav("/checkout", Default::default());
}
