use leptos::*;
use leptos_icons::*;
#[component]
pub fn Search() -> impl IntoView {
    view! {
        <section
            class="relative h-screen text-white bg-center bg-cover"
            style="background-image: url('hero-bg.jpg');"
        >
            <div class="absolute inset-0 justify-center bg-black opacity-50">
                <img
                    src="/img/fueldao.svg"
                    alt="Logo"
                    class="flex justify-between items-center h-8"
                />
            </div>

            <div class="flex absolute inset-x-0 top-0 z-20 justify-center items-center">
                <div class="relative w-full bg-white max-w-[756.75px] rounded-b-[75px]">
                    <ul class="flex justify-center items-center gap-[10%] list-none p-2 uppercase text-black">
                        <li>
                            <a href="#">Home</a>
                        </li>
                        <li>
                            <a href="#">Products</a>
                        </li>
                        <li>
                            <a href="#">Contact</a>
                        </li>
                    </ul>
                    <span class="absolute top-0 left-[-18px] w-[20px] h-[20px]  shadow-[5px_-5px_0px_rgba(127,127,127,1)] rounded-tr-[80px]"></span>
                    <span class="absolute top-0 right-[-18px] w-[20px] h-[20px]  shadow-[-5px_-5px_0px_rgba(127,127,127,1)] rounded-tl-[80px]"></span>
                </div>
                <div class="absolute top-0 z-50 bg-yellow-500 transform left-[-30px] h-[10px] w-[10px] rotate-[-20deg] rounded-tr-[20px]"></div>
            </div>

            <div class="flex relative z-10 items-center mx-auto max-w-screen-xl h-full">
                <div class="p-10" style="margin-bottom: 50px;">
                    <h1 class="text-7xl font-bold">Explore your place to <br />stay</h1>
                    <div class="relative mt-8">
                        <form class="flex items-center space-x-[10px] w-[1074px] h-[106px] p-[27px_34px] bg-[#1D1D1D9C] backdrop-blur-[3px] border-t-[1px] border-l-[1px] border-gray-400 rounded-[22.5px] mx-auto">
                            <input
                                type="text"
                                placeholder="Add your location"
                                class="p-4 w-full text-gray-900"
                            />
                            <input type="date" class="p-4 text-gray-900" />
                            <input type="time" class="p-4 text-gray-900" />
                            <input type="date" class="p-4 text-gray-900" />
                            <input type="time" class="p-4 text-gray-900" />
                            <button class="py-4 px-6 text-white bg-green-600 hover:bg-green-700">
                                Search
                            </button>
                        </form>
                    </div>

                    <div class="absolute right-0 p-4 mr-10 shadow-lg opacity-100 transform rotate-0 w-[431.3px] h-[175px] top-[564px]">
                        <p class="relative p-4 font-bold text-left text-white font-poppins text-[27px] leading-[35.1px] tracking-[0.075px] before:content-[''] before:absolute before:top-0 before:left-0 before:w-[4px] before:h-full before:bg-white">
                            We provide a variety of the best vehicles for those of you who need it.
                        </p>

                    // <p>"Don’t worry about the quality of the service."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
