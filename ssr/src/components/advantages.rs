use leptos::{component, view, IntoView};

#[component]
pub fn Advantages() -> impl IntoView {
    view! {
        <section class="py-12 bg-white">
            <div class="container mx-auto text-center gap-50">
                // first sub div
                <div
                    class="flex flex-col gap-3 items-center w-[1236.88px] h-[134px]"
                    style="margin-bottom: 49.99px;"
                >
                    // <!-- First Part -->
                    <div class="flex flex-col gap-2 px-2 h-auto max-w-[400px]">

                        <div class="flex flex-col gap-2 justify-center items-center">
                            <p class="font-bold text-left font-lato text-[20px] leading-[24px]">
                                ADVANTAGES
                            </p>
                        </div>
                        <div class="flex flex-col gap-2 pt-2">
                            <p class="font-medium text-left whitespace-nowrap font-lato text-[43px] leading-[51.59px]">
                                Why Choose Us?
                            </p>
                        </div>
                    </div>
                    // <!-- Second Part -->
                    <div class="flex flex-col gap-2 justify-center items-center p-2 pt-2 w-full h-[38px]">
                        <p class="text-left">
                            We present many guarantees and advantages when you rent a car with us for your trip. Here are some of the advantages that you will get
                        </p>
                    </div>
                </div>
                // second sub div
                // <h2 class="mb-8 text-3xl font-bold">"Why Choose Us ?"</h2>
                <div
                    class="grid grid-cols-1 gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3"
                    style="margin-bottom: 79.99px;"
                >
                    <div class="flex flex-col items-center">
                        <img src="/icons/rocket.svg" alt="Easy Rent" class="mb-4 w-12 h-12" />
                        <h3 class="text-lg font-bold">"Easy Rent"</h3>
                        <p class="text-gray-700">"We provide an easy and quick process..."</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <img src="/icons/check.svg" alt="Premium Quality" class="mb-4 w-12 h-12" />
                        <h3 class="text-lg font-bold">"Premium Quality"</h3>
                        <p class="text-gray-700">"Top-notch vehicles ensuring comfort..."</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <img
                            src="/icons/agent.svg"
                            alt="Professional Agent"
                            class="mb-4 w-12 h-12"
                        />
                        <h3 class="text-lg font-bold">"Professional Agent"</h3>
                        <p class="text-gray-700">"Our agents are highly trained to assist..."</p>
                    </div>
                </div>
                <div
                    class="grid grid-cols-1 gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3"
                    style="margin-bottom: 79.99px;"
                >
                    <div class="flex flex-col items-center">
                        <img src="/icons/car-safety.svg" alt="Easy Rent" class="mb-4 w-12 h-12" />
                        <h3 class="text-lg font-bold">"Easy Rent"</h3>
                        <p class="text-gray-700">"We provide an easy and quick process..."</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <img src="/icons/refund.svg" alt="Premium Quality" class="mb-4 w-12 h-12" />
                        <h3 class="text-lg font-bold">"Premium Quality"</h3>
                        <p class="text-gray-700">"Top-notch vehicles ensuring comfort..."</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <img
                            src="/icons/live-monitoring.svg"
                            alt="Professional Agent"
                            class="mb-4 w-12 h-12"
                        />
                        <h3 class="text-lg font-bold">"Professional Agent"</h3>
                        <p class="text-gray-700">"Our agents are highly trained to assist..."</p>
                    </div>
                </div>

            </div>
        </section>
    }
}
