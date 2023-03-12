#[cfg(feature = "IoLogoVercel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoVercel")]
/// *This icon requires the feature* `IoLogoVercel` *to be enabled*.
#[component]
pub fn LogoVercel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill-rule="evenodd" d="M256,48,496,464H16Z" /></svg>
   }
}