use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn BestPlacedForTrips() -> impl IntoView {
    let items = vec![
        ("Goa", "/public/img/nearby/Goa1.jpg", ""),
        ("Chikmagalur", "/public/img/nearby/Chikmagalur.jpg", ""),
        ("Coorg", "/public/img/nearby/Coorg.jpg", ""),
        ("Wayanad", "/public/img/nearby/Wayanad.jpeg", ""),
        ("Hyderabad", "/public/img/nearby/Hyderabad.jpeg", ""),
        
    ];

    let length = items.len();
    let (current_index, set_current_index) = create_signal(2); // Start with the 3rd item (index 2)

    // Functions to rotate the carousel
    let rotate_left = move |_| {
        set_current_index.update(|index| *index = (*index + length - 1) % length);
    };

    let rotate_right = move |_| {
        set_current_index.update(|index| *index = (*index + 1) % length);
    };

    // Function to handle mouse click on images
    let handle_image_click = move |index: usize| {
        set_current_index.set(index);
    };
   

    // Calculate the 5 indices to be displayed in the carousel
    let get_display_count = move || {
        // Capture the media query and check if the screen width is less than or equal to 640px
        5
        // if use_media_query("(max-width: 640px)").get_untracked() {
        //     3 // Display 3 items on small screens
        // } else {
        //     5 // Display 5 items on larger screens
        // }
    };
    // let log_size = {
    //     move || {
    //         logging::log!(
    //             "mobile screen: {:?}",
    //             use_media_query("(max-width: 640px)").get()
    //         )
    //     }
    // };
    // log_size();
    // Calculate the display indices to be displayed in the carousel
    let get_display_indices = |current: usize| {
        let count = get_display_count();
        (0..count)
            .map(|i| (current + i) % length) // Wrap around the array
            .collect::<Vec<_>>()
    };

    view! {
        // <ImageCarousel />
        <section class="p-2 py-12 bg-gray-100">
            <div class="container mx-auto text-center">
                <h2 class="mb-8 text-3xl font-bold">"Top spots near Mumbai"</h2>
                <div class="flex relative justify-center items-center">

                    <div class="flex absolute left-0 items-center h-full">
                        <button on:click=rotate_left>
                            <Icon
                                class="bg-gray-300 rounded-full w-[24px] h-[24px] lg:w-[30px] lg:h-[30px]"
                                icon=icondata::BsArrowLeftCircle
                            />
                        </button>
                    </div>

                    <div class="flex overflow-hidden space-x-4 carousel">
                        {get_display_indices(current_index.get())
                            .into_iter()
                            .enumerate()
                            .map(move |(idx, i)| {
                                let is_first = idx == 0;
                                let is_last = idx == get_display_count() - 1;
                                // let blur_class = if is_first || is_last {
                                //     "filter blur-sm"
                                // } else {
                                //     ""
                                // };
                                // let scale = if is_first || is_last { 0.75 } else { 1.0 };
                                // let transform_style = format!(
                                //     "scale({}) translateX({}px)",
                                //     scale,
                                //     (idx as f32 - 1.0) * 20.0,
                                // );
                                view! {
                                    // Adjusted for dynamic display count
                                    // Adjusted for dynamic index
                                    <div
                                        // style=transform_style
                                        class=format!(
                                            "flex flex-col justify-center items-center carousel-item transition-transform duration-300 ease-in-out transform {}",
                                            if is_first || is_last {
                                                "w-[80%] sm:w-[90%] lg:w-auto"
                                            } else {
                                                "w-[80%] sm:w-[95%] lg:w-auto"
                                            },
                                        )
                                        // Handle mouse click
                                        on:click=move |_| handle_image_click(i)
                                    >
                                        <img
                                            src=items[i].1
                                            alt=items[i].0
                                            class=format!(
                                                "object-cover rounded-lg shadow-lg {}",
                                                if is_first || is_last {
                                                    " h-32 w-40 lg:h-[65%]"
                                                } else {
                                                    "h-40 w-40 lg:h-auto"
                                                },
                                            )
                                        />
                                        <div class=format!(
                                            "flex flex-col items-center mt-2 {}",
                                            if is_first || is_last { "w-[80%]" } else { "w-full" },
                                        )>

                                            <div class="flex justify-between w-full">
                                                <h3 class="text-lg font-bold text-center">{items[i].0}</h3>
                                                // <div class="flex flex-col items-end">
                                                //     <p class="font-bold text-green-500">{items[i].2}</p>
                                                //     <p class="text-sm text-gray-500">{"per month"}</p>
                                                // </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>

                    <div class="flex absolute right-0 items-center h-full">
                        <button on:click=rotate_right>
                            <Icon
                                class="bg-gray-300 rounded-full w-[24px] h-[24px] lg:w-[30px] lg:h-[30px]"
                                icon=icondata::BsArrowRightCircle
                            />
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}


#[component]
pub fn ImageCarousel() -> impl IntoView {
    let images = vec![
        "/public/img/nearby/Goa1.jpg",
        "/public/img/nearby/Chikmagalur.jpg",
        "/public/img/nearby/Coorg.jpg",
        "/public/img/nearby/Wayanad.jpeg",
        "/public/img/nearby/Hyderabad.jpeg",
    ];

    let imgs = images.clone();

    let image_count = images.len();
    let (current_index, set_current_index) = create_signal(0);

    // Navigate carousel
    let previous_image = move |_| {
        set_current_index.update(|index| *index = (*index + image_count - 1) % image_count);
    };
    let next_image = move |_| {
        set_current_index.update(|index| *index = (*index + 1) % image_count);
    };

    view! {
        <section class="carousel relative w-full h-64 bg-gray-200 overflow-hidden">
            // Current Image
            <div class="h-full w-full">
                <img
                    class="object-cover w-full h-full transition-opacity duration-500"
                    src=move || imgs.clone()[current_index.get()]
                    alt=format!("Image {}", current_index.get() + 1)
                />
            </div>

            // Navigation Buttons
            <button
                class="absolute left-4 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white rounded-full p-2 hover:bg-gray-700"
                on:click=previous_image
            >
                <Icon icon=icondata::BsArrowLeftCircle class="w-6 h-6" />
            </button>
            <button
                class="absolute right-4 top-1/2 transform -translate-y-1/2 bg-gray-800 text-white rounded-full p-2 hover:bg-gray-700"
                on:click=next_image
            >
                <Icon icon=icondata::BsArrowRightCircle class="w-6 h-6" />
            </button>

            // Dots for navigation
            <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex space-x-2">
                {move || images.iter().enumerate().map(move |(index, _)| {
                    let is_active = current_index.get() == index;
                    view! {
                        <div
                            class=format!(
                                "w-3 h-3 rounded-full {}",
                                if is_active { "bg-gray-800" } else { "bg-gray-400" }
                            )
                            on:click=move |_| set_current_index.set(index)
                        />
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
