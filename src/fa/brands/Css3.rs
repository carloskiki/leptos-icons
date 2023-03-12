#[cfg(feature = "FaBrandsCss3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsCss3")]
/// *This icon requires the feature* `FaBrandsCss3` *to be enabled*.
#[component]
pub fn Css3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M480 32l-64 368-223.3 80L0 400l19.6-94.8h82l-8 40.6L210 390.2l134.1-44.4 18.8-97.1H29.5l16-82h333.7l10.5-52.7H56.3l16.3-82H480z" /></svg>
   }
}