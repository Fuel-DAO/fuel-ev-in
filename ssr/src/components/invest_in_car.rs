use leptos::*;

#[component]
pub fn InvestInCar() -> impl IntoView {
    view! {
        <section class="relative left-0 gap-0 w-[1440px] h-[722.64px] top-[0px]">
            <div
                class="absolute w-[1145.19px] h-[489.37px] left-[147.4px] top-12 pt-12 border-[2px] border-solid border-[#03B74B] rounded-[20px] flex p-12"
                style="gap: 121px;"
            >
                <div class="flex gap-0 justify-center items-center w-[443.63px] h-[393.37px]">
                    <img
                        src="/img/Group.png"
                        alt="Left Image"
                        class="object-cover w-full h-full rounded-[20px]"
                    />
                </div>

                <div class="flex flex-col justify-center opacity-100 w-[484.56px] h-[326.78px] gap-[40px]">
                    <div class="flex gap-0 items-center w-[484.56px] h-[144.81px]">
                        <p class="font-bold leading-none text-left font-baloo text-[75.41px]">
                            I<span style="color: #00b84c;">n</span>vest in a <br />Car
                            N<span style="color: #00b84c;">e</span>twork
                        </p>
                    </div>

                    <div class="h-auto w-[484.56px] gap-[9.99px]">
                        <p class="font-normal text-left font-lato text-[18px] leading-[21.6px]">
                            If you need consultation with us, you can write a message or call us, we will respond as quickly as possible
                        </p>
                    </div>

                    <div class="flex">
                        <button class="w-[164.94px] h-[57.96px] p-[9.99px_24.98px] gap-[9.99px] border-[2px] border-solid border-[#03B74B] bg-[#03B74B] text-white rounded-tl-[3px] rounded-tr-none rounded-br-none rounded-bl-none text-left">
                            Coming Soon
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
