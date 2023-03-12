#[cfg(feature = "BiRegularMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMinus")]
/// *This icon requires the feature* `BiRegularMinus` *to be enabled*.
#[component]
pub fn Minus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 11h14v2H5z" /></svg>
   }
}