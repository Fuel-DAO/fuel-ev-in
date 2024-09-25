use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-900 text-white p-8">
            <div class="container mx-auto text-center">
                <p>"© 2024 Your Company. All rights reserved."</p>
                <div class="flex justify-center space-x-4 mt-4">
                    <a href="#" class="hover:text-green-500">"Privacy Policy"</a>
                    <a href="#" class="hover:text-green-500">"Terms of Service"</a>
                    <a href="#" class="hover:text-green-500">"Contact"</a>
                </div>
            </div>
        </footer>
    }
}
