use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative mx-auto bg-[#252525] w-[1440px] h-[718.13px]">
            <div class="absolute top-0 bg-white w-[792px] h-[35px] left-[324px] rounded-b-[35px]">
                <div class="absolute top-0 left-[-20px] w-[50px] h-[35px] bg-[#252525] shadow-[1px_-1px_0px_rgba(31,41,55,1)] rounded-tr-[80px]"></div>
                <div class="absolute top-0 right-[-20px] w-[50px] h-[35px] bg-[#252525] shadow-[-1px_-1px_0px_rgba(31,41,55,1)] rounded-tl-[80px]"></div>
            </div>

            <div class="flex absolute flex-col justify-start p-2 opacity-100 transform top-[160px] left-[150px] w-[510px] h-[250px] rounded-[9px_0_0_0]">
                <h3 class="font-bold text-left text-white font-[Poppins] text-[29.98px] leading-[44.98px] tracking-[0.075px]">
                    Stay in the Know
                </h3>
                <p class="gap-0 text-left text-white w-[500px] h-[162px] mt-[25px]">
                    "Sign up to get marketing emails from Bookme.com, including promotions, rewards, travel experiences, and information about Bookme.com and Booking.com Transport Limited’s products and services."
                </p>
                <div class="flex items-center mt-[70px] gap-[6px]">
                    <input
                        type="text"
                        placeholder="Your email address"
                        class="p-2 text-white bg-gray-700 border border-gray-600 w-[390.55px] h-[52.47px] rounded-[9px]"
                        style="font-family: Poppins; font-size: 16.49px; font-weight: 400; line-height: 15.37px; letter-spacing: 0.075px; text-align: left;"
                    />

                    <button class="w-[141.68px] h-[52.47px] text-white bg-[#02b74b] rounded-[9px] ">
                        Subscribe
                    </button>
                </div>
                <p class="mt-2 text-white w-[499.99px] h-[20px]">
                    You can opt out anytime. See our <a href="#" class="text-[#09a446] underline">
                        privacy statement.
                    </a>
                </p>
            </div>
            <div class="absolute border-l border-[#4E5154] top-[160px] left-[918.12px] w-[5.56px] h-[300px]">
                <div class="mb-4">
                    <img
                        src="/img/fueldao.png"
                        alt="Logo"
                        class="w-full h-10"
                        style="max-width: 100%; height: auto;"
                    />
                </div>

                <nav class="flex flex-col justify-start items-start h-full ml-[50px]">
                    <div class="mb-4">
                        <img
                            src="/img/fueldao.png"
                            alt="Logo"
                            class="z-10 h-10"
                            style="max-width: 100%; height: auto;"
                        />
                    </div>
                    <a href="#" class="mb-2 text-white">
                        Help
                    </a>
                    <a href="#" class="mb-2 text-white">
                        FAQs
                    </a>

                    <a href="#" class="text-white">
                        Contact Us
                    </a>
                </nav>
            </div>

            <div class="flex absolute gap-0 justify-between items-start border-t border-[#4E5154] top-[589.35px] left-[154.89px] w-[1142px] h-[40.81px] p-[15px_0_0_0]">

                <div class="flex-1 text-white">Terms and Conditions</div>
                <div class="flex-1 text-white">Privacy Policy</div>
                <div class="flex flex-1 gap-6 justify-end items-center text-white">
                    <button class="border-t border-transparent icon w-[29.97px] h-[29.97px]">
                        <Icon icon=icondata::BsTwitterX class="w-full h-full text-white" />
                    </button>
                    <button class="border-t border-transparent icon w-[29.97px] h-[29.97px]">
                        <Icon icon=icondata::LuSend class="w-full h-full text-white" />
                    </button>
                    <button class="border-t border-transparent icon w-[33.97px] h-[33.97px]">
                        <Icon icon=icondata::BiInstagram class="w-full h-full text-white" />
                    </button>
                </div>
            </div>
        </footer>
    }
}
