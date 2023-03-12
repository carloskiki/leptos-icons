#[cfg(feature = "BiRegularSkipPrevious")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSkipPrevious")]
/// *This icon requires the feature* `BiRegularSkipPrevious` *to be enabled*.
#[component]
pub fn SkipPrevious(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m16 7-7 5 7 5zm-7 5V7H7v10h2z" /></svg>
   }
}