use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route
                    path=""
                    view=move || {
                        view! { <Index /> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Index() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-screen bg-sky-100">
            <div class="h-[8%] flex justify-center items-center bg-red-400">Header</div>
            <main class="flex flex-row h-auto flex-grow w-ful">
                <div class="w-1/6 bg-slate-400  flex justify-center items-center">Left</div>
                <div class="w-4/6 bg-purple-400 flex justify-center items-center">Center</div>
                <div class="w-1/6 bg-orange-400 flex justify-center items-center">Right</div>
            </main>
        </div>
    }
}
