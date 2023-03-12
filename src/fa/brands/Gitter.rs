#[cfg(feature = "FaBrandsGitter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsGitter")]
/// *This icon requires the feature* `FaBrandsGitter` *to be enabled*.
#[component]
pub fn Gitter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M66.4 322.5H16V0h50.4v322.5zM166.9 76.1h-50.4V512h50.4V76.1zm100.6 0h-50.4V512h50.4V76.1zM368 76h-50.4v247H368V76z" /></svg>
   }
}