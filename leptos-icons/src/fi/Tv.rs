#[cfg(feature = "FiTv")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiTv")]
/// *This icon requires the feature* `FiTv` *to be enabled*.
#[component]
pub fn Tv(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="7" width="20" height="15" rx="2" ry="2" /><polyline points="17 2 12 7 7 2" /></svg>
   }
}