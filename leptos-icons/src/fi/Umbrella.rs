#[cfg(feature = "FiUmbrella")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiUmbrella")]
/// *This icon requires the feature* `FiUmbrella` *to be enabled*.
#[component]
pub fn Umbrella(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7" /></svg>
   }
}