#[cfg(feature = "BiSolidMapAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMapAlt")]
/// *This icon requires the feature* `BiSolidMapAlt` *to be enabled*.
#[component]
pub fn MapAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m9 6.882-7-3.5v13.236l7 3.5 6-3 7 3.5V7.382l-7-3.5-6 3zM15 15l-6 3V9l6-3v9z" /></svg>
   }
}