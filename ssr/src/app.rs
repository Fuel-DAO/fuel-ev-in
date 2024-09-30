use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
//te
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/fuel-dao-leptos-ssr.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="FuelDao" />
        <main>
            <div class="flex flex-col min-h-screen">
                // <Header/>
                <Search />
                <BestPlacedForTrips />
                // <Services/>
                <Advantages />
                <InvestInCar />
                <GetInTouch />
                // <StayInTheKnow/>
                <Footer />
            </div>
        </main>
    }
}
