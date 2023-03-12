#[cfg(feature = "RiLogosFillVuejs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiLogosFillVuejs")]
/// *This icon requires the feature* `RiLogosFillVuejs` *to be enabled*.
#[component]
pub fn Vuejs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M1 3h4l7 12 7-12h4L12 22 1 3zm8.667 0L12 7l2.333-4h4.035L12 14 5.632 3h4.035z" /></g></svg>
   }
}