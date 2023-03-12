#[cfg(feature = "BiRegularChevronUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronUp")]
/// *This icon requires the feature* `BiRegularChevronUp` *to be enabled*.
#[component]
pub fn ChevronUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m6.293 13.293 1.414 1.414L12 10.414l4.293 4.293 1.414-1.414L12 7.586z" /></svg>
   }
}