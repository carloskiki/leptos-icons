#[cfg(feature = "BiSolidTree")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTree")]
/// *This icon requires the feature* `BiSolidTree` *to be enabled*.
#[component]
pub fn Tree(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m20 18-4-5h3l-4-5h2l-5-6-5 6h2l-4 5h3l-4 5h7v4h2v-4z" /></svg>
   }
}