use icondata::*;
use leptos::ev::MouseEvent;
use leptos::html::Input;
use leptos::HtmlElement;
use leptos::*;
use leptos_icons::Icon;
// use speedate::DateTime;
use leptos::*;

#[component]
pub fn Search() -> impl IntoView {
    // let input_ref = create_node_ref::<html::Input>();
    // let date_value = create_rw_signal(String::new());
    let (is_hamburger_open, set_hamburger_open) = create_signal(false);

    // Function to toggle the menu
    let toggle_menu = move || {
        set_hamburger_open.update(|open| *open = !*open);
    };
    let hamburger_menu_class = create_memo(move |_| {
        if is_hamburger_open.get() {
            "flex flex-col gap-6 items-center p-6 text-black uppercase bg-white"
        } else {
            "hidden" // Use hidden class when the menu is closed
        }
    });
    let (date_value, set_date_value): (ReadSignal<String>, WriteSignal<String>) =
        create_signal(String::new());

    // Create a node reference for the input
    let input_ref = create_node_ref::<Input>();

    // Get today's date using speedate
    // let today: DateTime = DateTime::now(0).unwrap();
    // let today_str = format!(
    //     "{}-{:02}-{:02}",
    //     today.date.year,
    //     today.date.month, // month is already 1-indexed
    //     today.date.day
    // );
    // let on_click = move |ev: MouseEvent| {
    //     // Get the input element from the NodeRef
    //     ev.prevent_default();
    //     ev.stop_propagation();

    //     if let Some(node) = input_ref.get() {
    //         // Set the type to 'date' and min date
    //         node.set_attribute("type", "date").unwrap();
    //         node.set_attribute("min", &today_str).unwrap();

    //         // Focus the input to show the date picker
    //         node.focus().unwrap()
    //     }
    // };

    view! {
        <section
            class="relative h-screen text-white bg-center bg-cover"
            style="background-image: url('img/hero-car-slide.png');"
        >
            <div class="absolute inset-0 justify-center p-4 pl-12 bg-black opacity-50">
                <img
                    src="/img/fueldao.svg"
                    alt="Logo"
                    class="flex justify-between items-center h-8 sm:h-4 md:h-6 lg:h-8 opacity-1"
                />

            </div>

            // hamburger menu
            <div class="flex absolute inset-x-0 top-0 z-20 justify-center items-center md:hidden">
                <div class="relative w-full md:bg-white max-w-[756.75px] md:rounded-b-[75px]">
                    <div class="flex justify-end items-center p-6 md:hidden">
                        <button
                            id="menu-btn"
                            class="focus:outline-none"
                            on:click=move |_| toggle_menu()
                        >
                            <svg
                                class="w-8 h-8 text-black"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M4 6h16M4 12h16M4 18h16"
                                ></path>
                            </svg>

                        </button>
                    </div>
                    // <!-- Hidden vertical menu for small devices (shown when hamburger clicked) -->

                    <div class=hamburger_menu_class>
                        <a href="#" class="block">
                            List your property
                        </a>
                        <a href="#" class="block">
                            Support
                        </a>
                        <a href="#" class="block">
                            Trips
                        </a>
                        <a href="#" class="block">
                            Sign in
                        </a>
                    </div>

                </div>
            </div>

            // navbar
            <div class="hidden absolute inset-x-0 top-0 z-20 justify-center items-center md:flex">
                <div class="hidden relative w-full bg-white md:block max-w-[756.75px] rounded-b-[75px]">
                    <ul
                        id="menu"
                        class="hidden md:flex justify-center items-center gap-[10%] list-none p-6 uppercase text-black"
                    >
                        <li class="sm:px-2 md:px-4">
                            <a href="#">List your property</a>
                        </li>
                        <li class="sm:px-2 md:px-4">
                            <a href="#">Support</a>
                        </li>
                        <li class="sm:px-2 md:px-4">
                            <a href="#">Trips</a>
                        </li>
                        <li class="sm:px-2 md:px-4">
                            <a href="#">Sign in</a>
                        </li>
                    </ul>

                    // Ensure these spans are hidden on small devices
                    <span class="hidden absolute top-0 md:block left-[-18px] w-[20px] h-[20px] rounded-tr-[80px]"></span>
                    <span class="hidden absolute top-0 md:block right-[-18px] w-[20px] h-[20px] rounded-tl-[80px]"></span>
                </div>

            </div>

            <div class="hidden absolute inset-x-0 bottom-0 z-20 justify-center items-center md:flex">
                <div class="relative w-full bg-white max-w-[756.75px] rounded-t-[75px]">
                    <ul class="flex justify-center items-center gap-[10%] list-none p-6 uppercase text-black"></ul>
                    <span class="absolute bottom-0 left-[-18px] w-[20px] h-[20px] rounded-br-[80px]"></span>
                    <span class="absolute bottom-0 right-[-18px] w-[20px] h-[20px] rounded-bl-[80px]"></span>
                </div>
                <div class="absolute bottom-0 z-50 bg-yellow-500 transform left-[-30px] h-[10px] w-[10px] rotate-[20deg] rounded-bl-[20px]"></div>
            </div>
            // hero section
            <div class="flex relative z-10 items-center mx-auto max-w-screen-xl h-full">
                <div class="p-4 lg:p-10" style="margin-bottom: 70px;">
                    <h1 class="text-5xl font-bold lg:text-7xl">Explore your place <br />to stay</h1>
                    <div class="relative mt-8">
                        <form class="flex flex-col lg:flex-row items-center space-x-[10px] w-4/5 md:w-full  lg:w-[1074px] h-full lg:h-[106px] p-12 pt-4  lg:p-[27px_34px] bg-[#1D1D1D9C] backdrop-blur-[3px] border-t-[1px] border-l-[1px] border-gray-400 rounded-[22.5px] ">
                            <div class="flex flex-col gap-4 px-2 lg:flex-row lg:gap-[21px] lg:w-[1005.5px]">
                                // first field
                                <div class="bg-[#1D1D1D9C] flex   items-center bg-opacity-0 w-[238.75px] h-[52.5px] lg:p-[13px_12px] gap-[10px] rounded-[9px]">
                                    <button class="pt-4 pb-4 pl-2">
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
                                    <div class="flex flex-row justify-between items-center w-[241.67px] gap-[2px] lg:h-[52.5px]">
                                        // <!-- First sub-field -->
                                        // <DateInputWithIcon
                                        // placeholder="Pickup Date"
                                        // icon=BsCalendar3
                                        // />
                                        //
                                        <div class="flex items-center w-[178.34px] lg:h-[22.34px] bg-[#252525] text-white placeholder-white lg:p-4 bg-opacity-0">
                                            <button
                                                class="pt-4 pb-4 pl-2 lg:pl-0"
                                                on:click=move |_| {
                                                    if let Some(input) = input_ref.get() {
                                                        input.click();
                                                    }
                                                }
                                            >

                                                <Icon
                                                    on:click=move |_| {
                                                        if let Some(input) = input_ref.get() {
                                                            input.click();
                                                        }
                                                    }

                                                    class="w-[22px] h-[22px]"
                                                    icon=icondata::BsCalendar3
                                                />

                                            </button>

                                            <input
                                                type="text"
                                                placeholder="Pickup Date"
                                                class="bg-[#252525] pl-2 bg-opacity-0 text-white w-full placeholder-white"
                                                ref=input_ref
                                                on:input=move |ev| {
                                                    ev.prevent_default();
                                                    let value = ev.target().unwrap().value_of();
                                                    set_date_value.set(value.to_string().into());
                                                }
                                                prop:value=date_value
                                            />

                                        </div>

                                        // <!-- Separator -->
                                        <div class="border-white opacity-0 rotate-90 w-[2.5px] h-[0px] border-t-[1.5px]"></div>

                                        // <!-- Second sub-field -->
                                        <div class="flex items-center w-[90.34px] lg:h-[22.34px] bg-[#252525] text-white placeholder-white pt-4 pb-4 pl-0  bg-opacity-0">
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
                            <button class="flex justify-center items-center py-3 px-8 mt-8 w-full font-semibold text-white bg-green-600 rounded-md lg:mt-0 lg:w-auto hover:bg-green-700">
                                Search
                            </button>
                        // <button class="flex justify-center items-center bg-green-600 hover:bg-green-700 w-[141.75px] h-[52.5px] p-[19px_39px] gap-[10px] rounded-[9px]">
                        // <span class="font-semibold text-white text-[15px] leading-[13.98px] tracking-[0.075em] font-poppins">
                        // Search
                        // </span>
                        // </button>
                        </form>
                    </div>

                    <div class="absolute right-0 shadow-lg opacity-100 transform rotate-0 lg:p-4 lg:mr-10 lg:top-[564px] lg:w-[431.3px] lg:h-[185px]">
                        <p class="relative pl-4 font-bold text-left text-white font-poppins tracking-[0.075px] before:content-[ lg:text-[27px] lg:leading-[35.1px]''] before:absolute before:top-0 before:left-0 before:w-[4px] before:h-full before:bg-white">
                            We provide a variety of the best vehicles for those of you who need it.
                        </p>

                        <p class="p-4">"Don’t worry about the quality of the service."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
