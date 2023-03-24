#[cfg(feature = "SiKotlin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiKotlin")]
/// *This icon requires the feature* `SiKotlin` *to be enabled*.
#[component]
pub fn Kotlin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 24H0V0h24L12 12Z" /></svg>
   }
}