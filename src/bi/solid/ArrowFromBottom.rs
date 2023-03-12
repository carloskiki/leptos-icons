#[cfg(feature = "BiSolidArrowFromBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowFromBottom")]
/// *This icon requires the feature* `BiSolidArrowFromBottom` *to be enabled*.
#[component]
pub fn ArrowFromBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 18h12v2H6zm6-14-6 6h5v6h2v-6h5z" /></svg>
   }
}