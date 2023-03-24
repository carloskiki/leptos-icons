#[cfg(feature = "FiPlusCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPlusCircle")]
/// *This icon requires the feature* `FiPlusCircle` *to be enabled*.
#[component]
pub fn PlusCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><line x1="12" y1="8" x2="12" y2="16" /><line x1="8" y1="12" x2="16" y2="12" /></svg>
   }
}