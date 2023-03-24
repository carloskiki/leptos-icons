#[cfg(feature = "IoChevronExpandOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronExpandOutline")]
/// *This icon requires the feature* `IoChevronExpandOutline` *to be enabled*.
#[component]
pub fn ChevronExpandOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M136 208L256 104L376 208" stroke="currentColor" fill="none" stroke-width="32" stroke-linecap="round" stroke-linejoin="round" /><path d="M136 304L256 408L376 304" stroke="currentColor" fill="none" stroke-width="32" stroke-linecap="round" stroke-linejoin="round" /></svg>
   }
}