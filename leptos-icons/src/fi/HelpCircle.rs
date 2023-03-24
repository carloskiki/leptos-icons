#[cfg(feature = "FiHelpCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiHelpCircle")]
/// *This icon requires the feature* `FiHelpCircle` *to be enabled*.
#[component]
pub fn HelpCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10" /><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" /><line x1="12" y1="17" x2="12.01" y2="17" /></svg>
   }
}