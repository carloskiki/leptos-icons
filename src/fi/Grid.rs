#[cfg(feature = "FiGrid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiGrid")]
/// *This icon requires the feature* `FiGrid` *to be enabled*.
#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7" /><rect x="14" y="3" width="7" height="7" /><rect x="14" y="14" width="7" height="7" /><rect x="3" y="14" width="7" height="7" /></svg>
   }
}