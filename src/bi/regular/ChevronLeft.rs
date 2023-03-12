#[cfg(feature = "BiRegularChevronLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronLeft")]
/// *This icon requires the feature* `BiRegularChevronLeft` *to be enabled*.
#[component]
pub fn ChevronLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13.293 6.293 7.586 12l5.707 5.707 1.414-1.414L10.414 12l4.293-4.293z" /></svg>
   }
}