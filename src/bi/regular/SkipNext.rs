#[cfg(feature = "BiRegularSkipNext")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSkipNext")]
/// *This icon requires the feature* `BiRegularSkipNext` *to be enabled*.
#[component]
pub fn SkipNext(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 7v10l7-5zm9 10V7h-2v10z" /></svg>
   }
}