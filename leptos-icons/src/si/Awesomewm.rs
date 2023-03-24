#[cfg(feature = "SiAwesomewm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAwesomewm")]
/// *This icon requires the feature* `SiAwesomewm` *to be enabled*.
#[component]
pub fn Awesomewm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 24V8.25h16.5V7.5H0V0h24v24h-7.5v-8.25h-9v.75h8.25V24z" /></svg>
   }
}