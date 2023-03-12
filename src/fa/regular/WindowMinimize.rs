#[cfg(feature = "FaRegularWindowMinimize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaRegularWindowMinimize")]
/// *This icon requires the feature* `FaRegularWindowMinimize` *to be enabled*.
#[component]
pub fn WindowMinimize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M24 432c-13.3 0-24 10.7-24 24s10.7 24 24 24H488c13.3 0 24-10.7 24-24s-10.7-24-24-24H24z" /></svg>
   }
}