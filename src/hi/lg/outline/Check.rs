#[cfg(feature = "HiLgOutlineCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlineCheck")]
/// *This icon requires the feature* `HiLgOutlineCheck` *to be enabled*.
#[component]
pub fn Check(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 12.75L10.5 18.75L19.5 5.25" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}