#[cfg(feature = "SiBandcamp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBandcamp")]
/// *This icon requires the feature* `SiBandcamp` *to be enabled*.
#[component]
pub fn Bandcamp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 18.75l7.437-13.5H24l-7.438 13.5H0z" /></svg>
   }
}