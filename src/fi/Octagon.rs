#[cfg(feature = "FiOctagon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiOctagon")]
/// *This icon requires the feature* `FiOctagon` *to be enabled*.
#[component]
pub fn Octagon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" /></svg>
   }
}