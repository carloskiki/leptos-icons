#[cfg(feature = "IoDiscOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDiscOutline")]
/// *This icon requires the feature* `IoDiscOutline` *to be enabled*.
#[component]
pub fn DiscOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="208" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><circle cx="256" cy="256" r="96" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><circle cx="256" cy="256" r="32" /></svg>
   }
}