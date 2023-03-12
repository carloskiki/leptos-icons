#[cfg(feature = "BiRegularRuble")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRuble")]
/// *This icon requires the feature* `BiRegularRuble` *to be enabled*.
#[component]
pub fn Ruble(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 21h2v-3h6v-2h-6v-2h4.5c2.757 0 5-2.243 5-5s-2.243-5-5-5H9a1 1 0 0 0-1 1v7H5v2h3v2H5v2h3v3zm2-15h4.5c1.654 0 3 1.346 3 3s-1.346 3-3 3H10V6z" /></svg>
   }
}