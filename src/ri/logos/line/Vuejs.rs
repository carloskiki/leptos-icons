#[cfg(feature = "RiLogosLineVuejs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiLogosLineVuejs")]
/// *This icon requires the feature* `RiLogosLineVuejs` *to be enabled*.
#[component]
pub fn Vuejs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3.316 3L12 18l8.684-15H23L12 22 1 3h2.316zm4.342 0L12 10.5 16.342 3h2.316L12 14.5 5.342 3h2.316z" /></g></svg>
   }
}