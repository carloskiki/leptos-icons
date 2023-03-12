#[cfg(feature = "FiMoreHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMoreHorizontal")]
/// *This icon requires the feature* `FiMoreHorizontal` *to be enabled*.
#[component]
pub fn MoreHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1" /><circle cx="19" cy="12" r="1" /><circle cx="5" cy="12" r="1" /></svg>
   }
}