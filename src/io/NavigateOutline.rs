#[cfg(feature = "IoNavigateOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoNavigateOutline")]
/// *This icon requires the feature* `IoNavigateOutline` *to be enabled*.
#[component]
pub fn NavigateOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,64,64,240.14H264a8,8,0,0,1,8,8V448Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}