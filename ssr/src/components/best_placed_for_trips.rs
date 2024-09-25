use leptos::*;

#[component]
pub fn BestPlacedForTrips() -> impl IntoView {
    let items = vec![
        ("Villa Avenil", "/img/trips.svg", "$200 / night"),
        ("Villa Edo", "/img/trips.svg", "$180 / night"),
        ("Villa Raffa", "/img/trips.svg", "$250 / night"),
        ("Villa Peony", "/img/trips.svg", "$300 / night"),
        ("Villa Amara", "/img/trips.svg", "$220 / night"),
    ];

    let length = items.len();

    let (current_index, set_current_index) = create_signal(2); // Start with the 3rd item centered

    /*


     let rotate_left = move |_| {
        set_current_index.update(move |index| *index = (*index + 1) % length);
    };

    let rotate_right = move |_| {
        set_current_index.update(move |index| *index = (*index - 1) % length);
    };  */

    view! {
        <section class="bg-gray-100 py-12">
            <div class="container mx-auto text-center">
                <h2 class="text-3xl font-bold mb-8">"Best placed for trips"</h2>
                <div class="relative flex items-center justify-center">
                    <div class="carousel flex space-x-4 overflow-hidden">
                      {
                             (0..length).map( move |i| {
                                let offset = ((i as i32) - (current_index.get() as i32)).abs();
                                let scale = 1.0 - (offset as f32) * 0.2;
                                let opacity = 1.0 - (offset as f32) * 0.3;

                                let transform_style = format!(
                                    "scale({}) translateX({}px)",
                                    scale,
                                    (offset as f32) * 30.0
                                );

                                view! {
                                    <div class="carousel-item" style={transform_style} style:opacity={opacity.to_string()} class="transition-transform transform duration-300 ease-in-out">
                                        <img src={items[i].1} alt={items[i].0} class="rounded-lg shadow-lg object-cover" />
                                        <h3 class="text-lg font-bold mt-2">{items[i].0}</h3>
                                        <p class="text-green-500 font-bold">{items[i].2}</p>
                                    </div>
                                }
                            }).collect_view()
                        }
                    </div>
                </div>
            </div>
        </section>
    }
}
