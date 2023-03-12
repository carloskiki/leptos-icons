#[cfg(feature = "FiTerminal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiTerminal")]
/// *This icon requires the feature* `FiTerminal` *to be enabled*.
#[component]
pub fn Terminal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 17 10 11 4 5" /><line x1="12" y1="19" x2="20" y2="19" /></svg>
   }
}