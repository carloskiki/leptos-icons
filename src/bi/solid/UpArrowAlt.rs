#[cfg(feature = "BiSolidUpArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUpArrowAlt")]
/// *This icon requires the feature* `BiSolidUpArrowAlt` *to be enabled*.
#[component]
pub fn UpArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 18v-6h5l-6-7-6 7h5v6z" /></svg>
   }
}