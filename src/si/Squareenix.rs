#[cfg(feature = "SiSquareenix")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSquareenix")]
/// *This icon requires the feature* `SiSquareenix` *to be enabled*.
#[component]
pub fn Squareenix(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.723 0v24h20.554v-4.496H7.037V4.088h15.006V0zm9.751 9.46v4.497h8.584V9.459z" /></svg>
   }
}