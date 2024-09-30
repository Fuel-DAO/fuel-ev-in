

use convert_case::{Case, Casing};
use leptos::*;
use server_fn::codec::Cbor;

use crate::{canister::backend::{Car, CarDetails, CarStatus}, components::{spinners::Spinner, PopupOverlay}, state::canisters::{authenticated_canisters, CanistersAuthWire}};



#[component]
pub fn SearchResult( ) -> impl IntoView {
    view! { 
            <SearchResultInner />
        
    }
}

#[server(input=Cbor)]
async fn fetch_all_cars(cans: CanistersAuthWire) -> Result<Vec<Car>, ServerFnError> {
    let backend =   cans.canisters().unwrap();
    let backend = backend.backend().await;
    
        let cars =  backend.list_all_cars().await;
        match cars {
            Ok(cars) => Ok(cars)
            , 
            Err(e) => Err(ServerFnError::ServerError(format!("Some error occuered while fetching cars {:?}", e)) )
            
        }

}

#[component]
fn SearchResultInner() -> impl IntoView {
    let cans_res = authenticated_canisters();

    let search_resource = create_resource(|| {}, move |_| { 
    let cans_res = cans_res.clone();

        async move {
        let cars =  fetch_all_cars(cans_res.wait_untracked().await.unwrap()).await;
        match cars {
            Ok(cars) => Ok(cars)
            , 
            Err(e) => Err(format!("Some error occuered while fetching cars {:?}", e))
            
        }
    } });

    view! {
        <Suspense fallback=move|| view! {<Spinner />}>
        <div>
        {
            move || {
                search_resource.get().map(|res| {
                    match  res {
                        Ok(cars) => {
                            view! {
                            <div>
                                <ShowSearchResult cars/ >
                            </div>
                            }
                        }, 
                        Err(e) => {
                                view! {
                                <div>
                                    <span>{e}</span>
                                </div>
                                }
                        }
                        
                    }
                })
            }
        }
        </div>
        </Suspense>
    }
}


#[component]
fn ShowSearchResult(cars: Vec<Car>) -> impl IntoView {
    view! {
        <div class= "grid grid-cols-1 sm:grid-cols-2  md:grid-cols-3 lg:grid-cols-4 gap-4 p-4">
        {
           cars.into_iter().map(|car| view! {<CarCard car />}).collect_view()
        }
        </div>
    }
    
}

#[component]
pub fn CarCard(car: Car) -> impl IntoView {
    // let car_principal = car.principal;
    let car = car.details;
    let details = car.clone();

    let car_type = format!("{:?}", car.car_type);
    let transmission_type = format!("{:?}", car.transmission_type);
    let car_status = format!("{:?}",  &car.status).to_case(Case::Title);
    let status = car.status.clone();
    let show_details_popup = create_rw_signal(false);
    let click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        show_details_popup.set(true);

    };

    

    view! { 
        <div class="p-4 bg-white rounded-xl shadow-md"
        class:opacity-50=move||format!("{:?}", status.clone()) != format!("{:?}",CarStatus::Available) &&format!("{:?}", status.clone()) != format!("{:?}",CarStatus::ComingSoon)  
        >   
            <button on:click=click>
            // Car Name and Type
            <div class="text-left mb-4">
                <h1 class="text-xl font-bold text-gray-800">{car.model}</h1>
                <p class="text-sm text-gray-400">{car_type}</p>
            </div>

            // Car Image
            <div class="mb-4">
                <img src="/img/car.svg" alt="Koenigsegg" class="w-full h-auto" />
            </div>

            </button>
            <PopupOverlay show=show_details_popup   >
                <CarDetailsPopup  car=details.clone() show_popup=show_details_popup/ >
            </PopupOverlay>

            // Car Details (Fuel, Transmission, Seating)
            <div class="flex justify-between items-center mb-4">
                <Show when=move||car.mileage.is_some()>
                    <div class="flex items-center">
                        <img src="/icons/mileage.svg" />
                        <span class="text-sm text-gray-600 ml-1">{car.mileage.unwrap()}Km</span>
                    </div>
                </Show>

                <div class="flex items-center">
                    <img src="/icons/car_transmission.svg" />
                    <span class="text-sm text-gray-600 ml-1">{transmission_type}</span>
                </div>

                <div class="flex items-center">
                    <img src="/icons/capacity.svg" />
                    <span class="text-sm text-gray-600 ml-1">{car.capacity}</span>
                </div>
            </div>

            // Pricing and Availability
            <div class="flex justify-between items-center">
                <div class="flex flex-col">
                    <div class="flex-1 text-lg font-bold text-gray-800">
                        "₹"{car.price_per_day}
                    <span class="text-sm text-gray-400">"/ day"</span>
                    </div>
                    <Show when=move||car.price_per_day != car.current_price_per_day >
                        <div class="flex-1 text-sm text-gray-400 line-through">
                            "₹"{car.price_per_day}/day
                        </div>
                    </Show>
                </div>
                {
                    match car.status {
                        CarStatus::Available => view! {
                                        <div class="px-3 py-1  fill-green-500 text-white rounded text-sm" style="background-color:#03B74B" >
                                            {car_status}
                                        </div>
                                    },
                        CarStatus::ComingSoon => view! {
                            <div class="px-3 py-1 border border-green-500 text-green-500 rounded text-sm">
                                {car_status}
                            </div>
                        },
                        _ => view! {
                            <div class="px-3 py-1 rounded text-white rounded text-sm" style="background-color:gray">
                                Not Available
                            </div>
                        }
                    }
                }
                
                
            </div>
        </div>
    }
}


#[component]
fn CarDetailsPopup(
    #[prop(into)] show_popup: SignalSetter<bool>,
    car: CarDetails
) -> impl IntoView {

    let car_type = format!("{:?}", car.car_type);
    let transmission_type = format!("{:?}", car.transmission_type);
    let car_status = format!("{:?}",  &car.status).to_case(Case::Title);
    let status = car.status.clone();
    view! { 
        <div class="p-4 bg-white rounded-xl shadow-md"
        class:opacity-50=move||format!("{:?}", status.clone()) != format!("{:?}",CarStatus::Available) &&format!("{:?}", status.clone()) != format!("{:?}",CarStatus::ComingSoon)  
        >   
            // Car Name and Type
            <div class="mb-4">
                <h1 class="text-xl font-bold text-gray-800">{car.model}</h1>
                <p class="text-sm text-gray-400">{car_type}</p>
            </div>

            // Car Image
            <div class="mb-4">
                <img src="/img/car.svg" alt="Koenigsegg" class="w-full h-auto" />
            </div>

            

            // Car Details (Fuel, Transmission, Seating)
            <div class="flex justify-between items-center mb-4">
                <div class="flex items-center">
                    <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
                        <path d="M21 13.05A12.042 12.042 0 0112.95 21L3 3h3l9.95 12.95A12.042 12.042 0 0121 13.05z"></path>
                    </svg>
                    <span class="text-sm text-gray-600 ml-1">"90L"</span>
                </div>

                <div class="flex items-center">
                    <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
                        <path d="M10 19l2 2l4-4"></path>
                    </svg>
                    <span class="text-sm text-gray-600 ml-1">{transmission_type}</span>
                </div>

                <div class="flex items-center">
                    <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
                        <path d="M16 17l4-4m0 0l-4-4m4 4H9"></path>
                    </svg>
                    <span class="text-sm text-gray-600 ml-1">"2 People"</span>
                </div>
            </div>

            // Pricing and Availability
            <div class="flex justify-between items-center">
                <div class="text-lg font-bold text-gray-800">
                    "$99.00"
                    <span class="text-sm text-gray-400">"/ day"</span>
                </div>
                {
                    match car.status {
                        CarStatus::Available => view! {
                                        <div class="px-3 py-1  fill-green-500 text-white rounded text-sm" style="background-color:#03B74B" >
                                            {car_status}
                                        </div>
                                    },
                        CarStatus::ComingSoon => view! {
                            <div class="px-3 py-1 border border-green-500 text-green-500 rounded text-sm">
                                {car_status}
                            </div>
                        },
                        _ => view! {
                            <div class="px-3 py-1 rounded text-white rounded text-sm" style="background-color:gray">
                                Not Available
                            </div>
                        }
                    }
                }
                
                
            </div>
        </div>
    }
}