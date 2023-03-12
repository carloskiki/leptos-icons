#[cfg(feature = "VsLoading")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLoading")]
/// *This icon requires the feature* `VsLoading` *to be enabled*.
#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.917 7A6.002 6.002 0 0 0 2.083 7H1.071a7.002 7.002 0 0 1 13.858 0h-1.012z" /></svg>
   }
}