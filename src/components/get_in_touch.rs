use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn GetInTouch() -> impl IntoView {
    view! {
        <section class="flex flex-col lg:flex-row items-center justify-center gap-12 lg:gap-24 px-4 lg:px-16 py-12 lg:py-20">
            // <!-- Contact Information Section -->
            <div class="flex flex-col gap-12 lg:w-1/2">
                // <!-- Header -->
                <div class="flex flex-col gap-6">
                    <h3 class="text-green-500 text-lg font-bold">GET IN TOUCH</h3>
                    <h1 class="text-4xl font-bold">Contact Us</h1>
                    <p class="text-gray-600 text-sm leading-relaxed">
                        If you need consultation with us, you can write a message or call us. We will respond as quickly as possible.
                    </p>
                </div>

                // <!-- Contact Details -->
                <div class="flex flex-col gap-6">
                    <div class="flex items-center gap-4">
                        <Icon class="text-green-500 w-6 h-6" icon=icondata::FiMail />
                        <p>bookings@fueldao.io</p>
                    </div>
                    <div class="flex items-center gap-4">
                        <Icon class="text-green-500 w-6 h-6" icon=icondata::TiLocationOutline />
                        <p>Mumbai, India</p>
                    </div>
                </div>

                // <!-- Social Media Links -->
                <div class="flex gap-4">
                    <button class="text-green-500 w-8 h-8">
                        <Icon icon=icondata::BiLinkedinSquare class="w-full h-full" />
                    </button>
                    <button class="text-green-500 w-8 h-8">
                        <Icon icon=icondata::BsTwitterX class="w-full h-full" />
                    </button>
                    <button class="text-green-500 w-8 h-8">
                        <Icon icon=icondata::LuSend class="w-full h-full" />
                    </button>
                    <button class="text-green-500 w-8 h-8">
                        <Icon icon=icondata::BiInstagram class="w-full h-full" />
                    </button>
                </div>
            </div>

            // <!-- Google Map Section -->
            <div class="lg:w-1/2">
                <iframe
                    src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d77850.86027949052!2d72.87719094676066!3d19.07457493347684!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x3be7c6306644edc1%3A0x5da4ed8f8d648c69!2sMumbai%2C%20Maharashtra%2C%20India!5e0!3m2!1sen!2sus!4v1733463154618!5m2!1sen!2sus"
                    class="w-full h-80 lg:h-[450px] border-0"
                    allowfullscreen=""
                    loading="lazy"
                    referrerpolicy="no-referrer-when-downgrade"
                >
                </iframe>
            </div>
        </section>
    }
}
