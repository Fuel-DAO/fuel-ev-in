use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::state::canisters::Canisters;
use base_route::BaseRoute;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(Canisters::default());

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fuel-dao-leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=BaseRoute>
                        <Route path="" view=HomePage/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="FuelDao"/>
        <main>
            <div class="min-h-screen flex flex-col">
                <Header/>
                <Search/>
                <BestPlacedForTrips/>
                // <Services/>
                <Advantages/>
                <GetInTouch/>
               // <StayInTheKnow/>
                <Footer/>
            </div>
        </main>
    }
}
