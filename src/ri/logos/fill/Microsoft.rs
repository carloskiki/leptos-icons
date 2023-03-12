#[cfg(feature = "RiLogosFillMicrosoft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiLogosFillMicrosoft")]
/// *This icon requires the feature* `RiLogosFillMicrosoft` *to be enabled*.
#[component]
pub fn Microsoft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11.5 3v8.5H3V3h8.5zm0 18H3v-8.5h8.5V21zm1-18H21v8.5h-8.5V3zm8.5 9.5V21h-8.5v-8.5H21z" /></g></svg>
   }
}