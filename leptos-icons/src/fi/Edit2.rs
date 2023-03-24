#[cfg(feature = "FiEdit2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiEdit2")]
/// *This icon requires the feature* `FiEdit2` *to be enabled*.
#[component]
pub fn Edit2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z" /></svg>
   }
}