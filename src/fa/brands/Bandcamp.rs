#[cfg(feature = "FaBrandsBandcamp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaBrandsBandcamp")]
/// *This icon requires the feature* `FaBrandsBandcamp` *to be enabled*.
#[component]
pub fn Bandcamp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M256,8C119,8,8,119,8,256S119,504,256,504,504,393,504,256,393,8,256,8Zm48.2,326.1h-181L207.9,178h181Z" /></svg>
   }
}