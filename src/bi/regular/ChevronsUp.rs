#[cfg(feature = "BiRegularChevronsUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronsUp")]
/// *This icon requires the feature* `BiRegularChevronsUp` *to be enabled*.
#[component]
pub fn ChevronsUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m6.293 11.293 1.414 1.414L12 8.414l4.293 4.293 1.414-1.414L12 5.586z" /><path d="m6.293 16.293 1.414 1.414L12 13.414l4.293 4.293 1.414-1.414L12 10.586z" /></svg>
   }
}