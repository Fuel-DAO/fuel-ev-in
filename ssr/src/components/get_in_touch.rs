use leptos::*;

#[component]
pub fn GetInTouch() -> impl IntoView {
    view! {
        <section class="bg-gray-100 py-12">
            <div class="container mx-auto text-center">
                <h2 class="text-3xl font-bold mb-8">"Get in Touch"</h2>
                <div class="flex flex-col md:flex-row justify-center items-center">
                    <div class="text-left md:w-1/2 p-4">
                        <h3 class="text-lg font-bold mb-2">"Contact Us"</h3>
                        <p class="text-gray-700 mb-4">"Feel free to reach out with any questions or concerns..."</p>
                        <p><strong>"Phone:"</strong> "+1 222 123 0001"</p>
                        <p><strong>"Email:"</strong> "fueldao@support.com"</p>
                        <p><strong>"Office Hours:"</strong> "08:00 - 21:00"</p>
                        <p><strong>"Address:"</strong> "Main road, Sekaten, India 42222"</p>
                    </div>
                    <div class="w-full md:w-1/2 p-4">
                        <iframe
                            src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3164.28354119872!2d-122.0842497!3d37.4220653!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x808fb5aaf1c3f15d%3A0xa44bc518fdb3f92a!2sGoogleplex!5e0!3m2!1sen!2sin!4v1549380919454"
                            width="100%" height="250" style="border:0;" allowfullscreen="" loading="lazy">
                        </iframe>
                    </div>
                </div>
            </div>
        </section>
    }
}
