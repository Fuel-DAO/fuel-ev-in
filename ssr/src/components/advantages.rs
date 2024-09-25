use leptos::{component, view, IntoView};

#[component]
pub fn Advantages() -> impl IntoView {
    view! {
        <section class="bg-white py-12">
            <div class="container mx-auto text-center">
                <h2 class="text-3xl font-bold mb-8">"Why Choose Us ?"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 gap-8">
                    <div class="flex flex-col items-center">
                        <img src="/icons/rocket.svg" alt="Easy Rent" class="w-12 h-12 mb-4" />
                        <h3 class="text-lg font-bold">"Easy Rent"</h3>
                        <p class="text-gray-700">"We provide an easy and quick process..."</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <img src="/icons/check.svg" alt="Premium Quality" class="w-12 h-12 mb-4" />
                        <h3 class="text-lg font-bold">"Premium Quality"</h3>
                        <p class="text-gray-700">"Top-notch vehicles ensuring comfort..."</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <img src="/icons/agent.svg" alt="Professional Agent" class="w-12 h-12 mb-4" />
                        <h3 class="text-lg font-bold">"Professional Agent"</h3>
                        <p class="text-gray-700">"Our agents are highly trained to assist..."</p>
                    </div>
                                    </div>
            </div>
        </section>
    }
}
