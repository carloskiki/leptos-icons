#[cfg(feature = "OcSmCommandPalette")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmCommandPalette")]
/// *This icon requires the feature* `OcSmCommandPalette` *to be enabled*.
#[component]
pub fn CommandPalette(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="m6.354 8.04-4.773 4.773a.75.75 0 1 0 1.061 1.06L7.945 8.57a.75.75 0 0 0 0-1.06L2.642 2.206a.75.75 0 0 0-1.06 1.061L6.353 8.04ZM8.75 11.5a.75.75 0 0 0 0 1.5h5.5a.75.75 0 0 0 0-1.5h-5.5Z" /></svg>
   }
}