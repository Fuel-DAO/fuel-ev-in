use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn BestPlacedForTrips() -> impl IntoView {
    let items = vec![
        ("Villa Avenil", "/img/trips.svg", "$200 / night"),
        ("Villa Edo", "/img/trips.svg", "$180 / night"),
        ("Villa Raffa", "/img/trips.svg", "$250 / night"),
        ("Villa Peony", "/img/trips.svg", "$300 / night"),
        ("Villa Amara", "/img/trips.svg", "$220 / night"),
        ("Villa Avenil", "/img/2.jpg", "$200 / night"),
        ("Villa Edo", "/img/trips.svg", "$180 / night"),
        ("Villa Raffa", "/img/trips.svg", "$250 / night"),
        ("Villa Peony", "/img/trips.svg", "$300 / night"),
        ("Villa Amara", "/img/trips.svg", "$220 / night"),
    ];

    let length = items.len();
    let (current_index, set_current_index) = create_signal(2); // Start with the 3rd item (index 2)

    // Functions to rotate the carousel
    let rotate_left = move |_| {
        set_current_index.update(|index| {
            *index = (*index + length - 1) % length; // Rotate left
        });
    };

    let rotate_right = move |_| {
        set_current_index.update(|index| {
            *index = (*index + 1) % length; // Rotate right
        });
    };

    // Function to handle mouse click on images
    let handle_image_click = move |index: usize| {
        set_current_index.set(index); // Set current index to the clicked image's index
    };

    // Calculate the 5 indices to be displayed in the carousel
    let get_display_indices = |current: usize| {
        let mut indices = Vec::new();
        for i in 0..5 {
            indices.push((current + i) % length); // Wrap around the array
        }
        indices
    };

    view! {
        <section class="py-12 bg-gray-100">
            <div class="container mx-auto text-center">
                <h2 class="mb-8 text-3xl font-bold">"Best placed for trips"</h2>
                <div class="flex relative justify-center items-center">

                    <div class="flex absolute left-0 items-center h-full">
                        <button on:click=rotate_left>
                            <Icon
                                class="bg-gray-300 rounded-full w-[24px] h-[24px]"
                                icon=icondata::BsArrowLeftCircle
                            />
                        </button>
                    </div>

                    <div class="flex overflow-hidden space-x-4 carousel">
                        {get_display_indices(current_index.get())
                            .into_iter()
                            .map(move |i| {
                                let offset = ((i as i32) - (current_index.get() as i32)).abs();
                                let scale = if offset == 0 {
                                    1.0
                                } else if offset == 1 {
                                    0.8
                                } else {
                                    0.6
                                };
                                let opacity = 1.0 - (offset as f32) * 0.2;
                                let transform_style = format!(
                                    "scale({}) translateX({}px)",
                                    scale,
                                    (offset as f32) * 20.0,
                                );
                                view! {
                                    <div
                                        class="carousel-item"
                                        style=transform_style
                                        style:opacity=opacity.to_string()
                                        class="transition-transform duration-300 ease-in-out transform cursor-pointer"
                                        // Handle mouse click
                                        on:click=move |_| handle_image_click(i)
                                    >
                                        <img
                                            src=items[i].1
                                            alt=items[i].0
                                            class="object-cover rounded-lg shadow-lg"
                                        />
                                        <h3 class="mt-2 text-lg font-bold">{items[i].0}</h3>
                                        <p class="font-bold text-green-500">{items[i].2}</p>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>

                    <div class="flex absolute right-0 items-center h-full">
                        <button on:click=rotate_right>
                            <Icon
                                class="bg-gray-300 rounded-full w-[24px] h-[24px]"
                                icon=icondata::BsArrowRightCircle
                            />
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
