#[cfg(feature = "IoMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMenu")]
/// *This icon requires the feature* `IoMenu` *to be enabled*.
#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="88" y1="152" x2="424" y2="152" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:48px" /><line x1="88" y1="256" x2="424" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:48px" /><line x1="88" y1="360" x2="424" y2="360" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:48px" /></svg>
   }
}