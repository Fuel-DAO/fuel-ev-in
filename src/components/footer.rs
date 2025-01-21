use leptos::*;
use leptos_icons::Icon;

use crate::components::{auth_cans_provider::AuthCansProvider, spinners::FullScreenSpinner};


#[component]
pub fn FooterSection() -> impl IntoView {
    view! { 
        <div class="bg-[#252525] text-white py-12 px-6 md:px-16">
            <div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-start space-y-12 md:space-y-0">
                // Left section: Stay in the Know
                <div class="w-full md:w-1/2 space-y-4">
                    <h2 class="text-3xl font-bold text-center md:text-left">"Stay in the know"</h2>
                    <p class="text-gray-300 text-center md:text-left">
                        "Sign up to get marketing emails from FuelEV, including promotions, rewards, travel experiences and information about FuelEV."
                    </p>

                    // Email and Subscribe button
                    <div class="flex flex-col md:flex-row space-y-4 md:space-y-0 space-x-0 md:space-x-4">
                        <input
                            type="email"
                            placeholder="Your email address"
                            class="py-3 px-4 w-full bg-white text-gray-900 rounded-md focus:outline-none"
                        />
                        <button class="py-3 px-6 w-full md:w-auto bg-green-500 text-white font-bold rounded-md hover:bg-green-600 transition duration-300">
                            "Subscribe"
                        </button>
                    </div>

                    <p class="text-sm text-gray-500 text-center md:text-left">
                        "You can opt out anytime. See our "
                        <a href="#" class="text-green-400 underline">"privacy statement."</a>
                    </p>
                </div>

                // Right section: Logo and Links
                <div class="w-full md:w-1/2 flex flex-col items-center md:items-end space-y-8">
                    // Logo
                    <img src="/public/icons/white.png" alt="Fuel DAO logo White" class="h-12"/>

                    // Links
                    // <div class="flex flex-col space-y-2 text-green-400 text-center md:text-right">
                    //     <a href="#" class="hover:underline">"Help"</a>
                    //     // <a href="#" class="hover:underline">"FAQs"</a>
                    //     <a href="#" class="hover:underline">"Contact Us"</a>
                    // </div>
                </div>
            </div>

            // Footer bottom
            <div class="mt-12 border-t border-gray-700 pt-8 flex flex-col md:flex-row justify-between items-center space-y-6 md:space-y-0">
                // Footer Links
                <div class="flex space-x-4 text-sm text-center md:text-left">
                    <a href="#" class="text-gray-400 hover:text-white">"Terms & Conditions"</a>
                    <a href="#" class="text-gray-400 hover:text-white">"Privacy Policy"</a>
                </div>

                // Social Media Icons
                <div class="flex space-x-6">
                    <a href="#" class="text-gray-400 hover:text-white">
                        <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">...</svg> // Replace with SVG for email icon
                    </a>
                    <a href="#" class="text-gray-400 hover:text-white">
                        <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">...</svg> // Replace with SVG for Twitter icon
                    </a>
                    <a href="#" class="text-gray-400 hover:text-white">
                        <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">...</svg> // Replace with SVG for Instagram icon
                    </a>
                </div>
            </div>
        </div>
    }
}



#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative  bg-[#252525] w-full h-[718.13px]">
            // <div class="hidden absolute top-0 bg-white md:block w-[792px] h-[35px] left-[324px] rounded-b-[35px]">
            //     <div class="absolute top-0 left-[-20px] w-[50px] h-[35px] bg-[#252525] shadow-[1px_-1px_0px_rgba(31,41,55,1)] rounded-tr-[80px]"></div>
            //     <div class="absolute top-0 right-[-20px] w-[50px] h-[35px] bg-[#252525] shadow-[-1px_-1px_0px_rgba(31,41,55,1)] rounded-tl-[80px]"></div>
            // </div>
            <div class="flex flex-col md:flex-row">
                <div class="flex flex-col justify-start p-2 opacity-100 transform lg:absolute rounded-[9px_0_0_0] lg:h-[250px] lg:w-[510px] lg:top-[160px] lg:left-[150px]">
                    <h3 class="font-bold text-left text-white font-[Poppins] text-[29.98px] leading-[44.98px] tracking-[0.075px]">
                        Stay in the Know
                    </h3>
                    <p class="gap-0 text-left text-white h-[162px] mt-[25px] lg:w-[500px]">
                        "Sign up to get marketing emails from FuelEV, including promotions, rewards, travel experiences and information about FuelEV."
                    </p>
                    <div class="flex flex-col items-center lg:flex-row gap-[6px] lg:mt-[70px]">
                        <input
                            type="text"
                            placeholder="Your email address"
                            class="p-2 text-white bg-gray-700 border border-gray-600 h-[52.47px] rounded-[9px] lg:w-[390.55px]"
                            style="font-family: Poppins; font-size: 16.49px; font-weight: 400; line-height: 15.37px; letter-spacing: 0.075px; text-align: left;"
                        />

                        <button class="w-[141.68px] h-[52.47px] text-white bg-[#02b74b] rounded-[9px] ">
                            Subscribe
                        </button>
                    </div>
                    <p class="mx-8 mt-2 text-white lg:mx-0 h-[20px] lg:w-[499.99px]">
                        You can opt out anytime. See our
                        <a href="#" class="text-[#09a446] underline">
                            privacy statement.
                        </a>
                    </p>
                </div>
                <div class="lg:absolute border-l border-[#4E5154] pt-4 lg:pt-0 lg:top-[160px] lg:left-[918.12px] w-[5.56px] h-[300px]">
                    <div class="mb-4">
                        <img
                            src="/public/img/app.svg"
                            alt="Logo"
                            class="w-full h-10"
                            style="max-width: 100%; height: auto;"
                        />
                    </div>

                    // <nav class="flex flex-col justify-start items-start h-full ml-[50px]">
                    //     <div class="mb-4">
                    //         <img
                    //             src="/public/img/app.svg"
                    //             alt="Logo"
                    //             class="z-10 h-10"
                    //             style="max-width: 100%; height: auto;"
                    //         />
                    //     </div>
                    //     <a href="#" class="mb-2 text-white">
                    //         Help
                    //     </a>
                    //     // <a href="#" class="mb-2 text-white">
                    //     //     FAQs
                    //     // </a>

                    //     <a href="#" class="text-white">
                    //         Contact Us
                    //     </a>
                    // </nav>
                </div>
            </div>
            <div class="flex absolute gap-0 justify-between items-start border-t border-[#4E5154] top-[589.35px] lg:left-[154.89px] lg:w-[1142px] h-[40.81px] px-4 lg:px-0 lg:p-[15px_0_0_0]">

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

#[component]
pub fn UserPrincipal() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canister>
            <p>{canister.user_principal().to_text()}</p>
        </AuthCansProvider>
    }
}
