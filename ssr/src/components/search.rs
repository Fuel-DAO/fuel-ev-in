use leptos::*;
use leptos_icons::Icon;
#[component]
pub fn Search() -> impl IntoView {
    view! {
        <section
            class="relative h-screen text-white bg-center bg-cover"
            style="background-image: url('img/hero-car-slide.png');"
        >
            <div class="absolute inset-0 justify-center p-4 pl-12 bg-black opacity-50">
                <img
                    src="/img/fueldao.svg"
                    alt="Logo"
                    class="flex justify-between items-center h-8"
                />
            </div>
            // Navbar
            <div class="flex absolute inset-x-0 top-0 z-20 justify-center items-center">
                <div class="relative w-full bg-white max-w-[756.75px] rounded-b-[75px]">
                    <ul class="flex justify-center items-center gap-[10%] list-none p-6 uppercase text-black">

                        <li>
                            <a href="#">List your property</a>
                        </li>
                        <li>
                            <a href="#">Support</a>
                        </li>
                        <li>
                            <a href="#">Trips</a>
                        </li>
                        <li>
                            <a href="#">Sign in</a>
                        </li>

                    </ul>
                    <span class="absolute top-0 left-[-18px] w-[20px] h-[20px]  shadow-[5px_-5px_0px_rgba(127,127,127,1)] rounded-tr-[80px]"></span>
                    <span class="absolute top-0 right-[-18px] w-[20px] h-[20px]  shadow-[-5px_-5px_0px_rgba(127,127,127,1)] rounded-tl-[80px]"></span>
                </div>
                <div class="absolute top-0 z-50 bg-yellow-500 transform left-[-30px] h-[10px] w-[10px] rotate-[-20deg] rounded-tr-[20px]"></div>
            </div>

            <div class="flex relative z-10 items-center mx-auto max-w-screen-xl h-full">
                <div class="p-10" style="margin-bottom: 50px;">
                    <h1 class="text-7xl font-bold">Explore your place <br />to stay</h1>
                    <div class="relative mt-8">
                        <form class="flex items-center space-x-[10px] w-[1074px] h-[106px] p-[27px_34px] bg-[#1D1D1D9C] backdrop-blur-[3px] border-t-[1px] border-l-[1px] border-gray-400 rounded-[22.5px] mx-auto">
                            <div class="flex w-[1005.5px] h-[52.5px] gap-[21px]">
                                // first field
                                <div class="bg-[#1D1D1D9C] flex items-center bg-opacity-0 w-[238.75px] h-[52.5px] p-[13px_12px] gap-[10px] rounded-[9px]">
                                    <button class="pt-4 pb-4">
                                        <Icon
                                            class="rounded-full w-[24px] h-[24px]"
                                            icon=icondata::BiSearchRegular
                                        />
                                    </button>

                                    <input
                                        type="text"
                                        placeholder="Add your location"
                                        class="w-[213.81px] h-[24px]  pt-4 pr-4 pb-4 pl-0  bg-[#252525] bg-opacity-0 text-white placeholder-white"
                                    />
                                </div>
                                // second field
                                <div class="flex items-center bg-[#1D1D1D9C] bg-opacity-0 w-[281px] h-[52px] p-[0px_11px] gap-[6px] rounded-tl-[9px] rounded-[9px] ">
                                    // <!-- First part of the input -->
                                    <div class="flex justify-between items-center w-[241.67px] h-[52.5px] gap-[2px]">
                                        // <!-- First sub-field -->
                                        <div class="flex items-center w-[178.34px] h-[22.34px] bg-[#252525] text-white placeholder-white p-4 bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon
                                                    class="w-[22px] h-[22px]"
                                                    icon=icondata::BsCalendar3
                                                />
                                            </button>

                                            <input
                                                type="text"
                                                placeholder="Pickup Date"
                                                class="bg-[#252525] pl-4 bg-opacity-0 text-white w-full placeholder-white"
                                            />
                                        </div>

                                        // <!-- Separator -->
                                        <div class="border-white opacity-0 rotate-90 w-[2.5px] h-[0px] border-t-[1.5px]"></div>

                                        // <!-- Second sub-field -->
                                        <div class="flex items-center w-[90.34px] h-[22.34px] bg-[#252525] text-white placeholder-white pt-4 pb-4 pl-0  bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon class="w-[24px] h-[24px]" icon=icondata::WiTime10 />
                                            </button>
                                            <input
                                                type="text"
                                                placeholder="Time"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"
                                            />
                                        </div>
                                    </div>
                                </div>
                                // third field
                                <div class="flex items-center bg-[#1D1D1D9C] bg-opacity-0 w-[281px] h-[52px] p-[0px_11px] gap-[2px] rounded-[9px]">
                                    // <!-- Placeholder container -->
                                    <div class="flex justify-between items-center w-[240.67px] h-[52.5px] gap-[2px]">
                                        // <!-- First part of the placeholder -->
                                        <div class="flex items-center w-[178.34px] h-[22.34px] bg-[#252525] text-white  bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon
                                                    class="w-[22px] h-[22px]"
                                                    icon=icondata::BsCalendar3
                                                />
                                            </button>
                                            <input
                                                type="text"
                                                placeholder="Return Date"
                                                class="bg-[#252525] pl-4  bg-opacity-0 text-white w-full placeholder-white"
                                            />
                                        </div>

                                        // <!-- Separator -->
                                        <div class="border-white opacity-0 rotate-90 w-[2.5px] h-[0px] border-t-[1.5px]"></div>

                                        // <!-- Second part of the placeholder -->
                                        <div class="flex items-center w-[98.34px] h-[22.34px] bg-[#252525] text-white  bg-opacity-0">
                                            <button class="pt-4 pb-4">
                                                <Icon class="w-[24px] h-[24px]" icon=icondata::WiTime10 />
                                            </button>
                                            <input
                                                type="text"
                                                placeholder="Time"
                                                class="bg-[#252525] pl-2  bg-opacity-0 text-white w-full placeholder-white"
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <button class="flex justify-center items-center bg-green-600 hover:bg-green-700 w-[141.75px] h-[52.5px] p-[19px_39px] gap-[10px] rounded-[9px]">
                                <span class="font-semibold text-white text-[15px] leading-[13.98px] tracking-[0.075em] font-poppins">
                                    Search
                                </span>
                            </button>
                        </form>
                    </div>

                    <div class="absolute right-0 p-4 mr-10 shadow-lg opacity-100 transform rotate-0 w-[431.3px] h-[185px] top-[564px]">
                        <p class="relative pl-4 font-bold text-left text-white font-poppins text-[27px] leading-[35.1px] tracking-[0.075px] before:content-[''] before:absolute before:top-0 before:left-0 before:w-[4px] before:h-full before:bg-white">
                            We provide a variety of the best vehicles for those of you who need it.
                        </p>

                        <p class="p-4">"Don’t worry about the quality of the service."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
