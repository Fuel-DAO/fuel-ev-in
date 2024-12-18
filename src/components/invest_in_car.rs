use leptos::*;

#[component]
pub fn InvestInCar() -> impl IntoView {
    view! {
        <section class="relative w-full h-[72vh] lg:top-0 flex justify-center items-center mb-4">
            <div
                class="absolute w-4/5 lg:h-auto lg:left-1/6 lg:top-12 lg:pt-12 border-2 border-solid border-green-500 rounded-xl flex flex-col lg:flex-row lg:p-12"
                style="gap: 8%"
            >
                <div class="flex gap-0 justify-center items-center lg:w-2/5">
                    <img
                        src="/public/img/Group.png"
                        alt="Left Image"
                        class="object-cover w-full h-full rounded-xl"
                    />
                </div>

                <div class="flex flex-col justify-center p-4 opacity-100 lg:p-0 lg:h-auto lg:w-1/2 lg:gap-10">
                    <div class="flex gap-0 items-center lg:h-auto lg:w-full">
                        <p class="font-bold leading-none text-left font-baloo text-5xl">
                            I<span class="text-green-500">n</span>vest in a <br />Car
                            N<span class="text-green-500">e</span>twork
                        </p>
                    </div>

                    <div class="h-auto gap-2 lg:w-full">
                        <p class="font-normal text-left font-lato text-base leading-relaxed">
                        Invest in a car and earn proportional revenue on it! An exclusive, members-only service that enables global investment into car networks
                        </p>
                    </div>

                    <div class="flex">
                        <button class="w-40 lg:h-14 py-2 px-6 border-2 border-solid border-green-500 bg-green-500 text-white rounded-tl-md text-left">
                            Coming Soon
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
