#[cfg(feature = "SiBulma")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBulma")]
/// *This icon requires the feature* `SiBulma` *to be enabled*.
#[component]
pub fn Bulma(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.25 0l-6 6 -1.5 10.5 7.5 7.5 9 -6 -6 -6 4.5 -4.5 -7.5 -7.5Z" /></svg>
   }
}