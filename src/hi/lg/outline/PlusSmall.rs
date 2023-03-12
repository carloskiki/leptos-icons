#[cfg(feature = "HiLgOutlinePlusSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgOutlinePlusSmall")]
/// *This icon requires the feature* `HiLgOutlinePlusSmall` *to be enabled*.
#[component]
pub fn PlusSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 6V18M18 12L6 12" stroke="#0F172A" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}