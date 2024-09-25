use leptos::*;

#[component]
pub fn Search() -> impl IntoView {
    view! {
        <div class="bg-gray-100 p-8">
            <div class="container mx-auto flex justify-center">
                <div class="bg-white shadow-md p-6 rounded-lg flex space-x-4">
                    <input type="text" placeholder="Add your location" class="border p-2 rounded w-1/3" />
                    <input type="date" class="border p-2 rounded" />
                    <input type="date" class="border p-2 rounded" />
                    <button class="bg-green-500 text-white px-6 py-2 rounded">"Search"</button>
                </div>
            </div>
        </div>
    }
}
