#[cfg(feature = "ImDrive")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImDrive")]
/// *This icon requires the feature* `ImDrive` *to be enabled*.
#[component]
pub fn Drive(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M3 14h10c1.657 0 3-1.343 3-3h-16c0 1.657 1.343 3 3 3zM13 12h1v1h-1v-1zM15 2h-14l-1 8h16z" /></svg>
   }
}