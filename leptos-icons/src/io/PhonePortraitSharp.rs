#[cfg(feature = "IoPhonePortraitSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPhonePortraitSharp")]
/// *This icon requires the feature* `IoPhonePortraitSharp` *to be enabled*.
#[component]
pub fn PhonePortraitSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M382,0H130a18,18,0,0,0-18,18V494a18,18,0,0,0,18,18H382a18,18,0,0,0,18-18V18A18,18,0,0,0,382,0ZM148,448V64H364V448Z" /></svg>
   }
}