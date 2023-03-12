#[cfg(feature = "HiLgOutlinePlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlinePlus")]
/// *This icon requires the feature* `HiLgOutlinePlus` *to be enabled*.
#[component]
pub fn Plus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 4.5V19.5M19.5 12L4.5 12" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}