

use leptos::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::Cbor;

use crate::{canister::backend::CarDetails, components::{auth_cans_provider::AuthCansProvider, spinners::Spinner}, state::canisters::{authenticated_canisters, Canisters, CanistersAuthWire}};

// use crate::canister::backend::CarDetails;






#[component]
pub fn SearchResult( ) -> impl IntoView {
    view! { 
        <AuthCansProvider  let:cans>
            <SearchResultInner cans />
        </AuthCansProvider>
        
    }
}

#[server(input=Cbor)]
async fn fetch_all_cars(cans: CanistersAuthWire) -> Result<Vec<CarDetails>, ServerFnError> {
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
fn SearchResultInner(cans: Canisters<true>) -> impl IntoView {
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
fn ShowSearchResult(cars: Vec<CarDetails>) -> impl IntoView {
    view! {
        <div class= "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
        {
           cars.into_iter().map(|car| view! {<CarCard car />}).collect_view()
        }
        </div>
    }
    
}

#[component]
pub fn CarCard(car: CarDetails) -> impl IntoView {
    let car_type = format!("{:?}", car.car_type);
    let transmission_type = format!("{:?}", car.transmission_type);

    view! { 
        <div class="w-64 p-4 bg-white rounded-xl shadow-md">
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
                <div class="px-3 py-1 border border-green-500 text-green-500 rounded-full text-sm">
                    "Coming soon"
                </div>
            </div>
        </div>
    }
}
