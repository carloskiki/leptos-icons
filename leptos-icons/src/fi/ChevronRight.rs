#[cfg(feature = "FiChevronRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiChevronRight")]
/// *This icon requires the feature* `FiChevronRight` *to be enabled*.
#[component]
pub fn ChevronRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6" /></svg>
   }
}