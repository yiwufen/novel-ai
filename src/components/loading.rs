use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="fixed inset-0 flex items-center justify-center">
            <div class="spinner"><img src="https://img-s-msn-com.akamaized.net/tenant/amp/entityid/BB1lE3kW.img?w=600&h=406&m=6" /></div>
        </div>
    }
}