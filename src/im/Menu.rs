#[cfg(feature = "ImMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMenu")]
/// *This icon requires the feature* `ImMenu` *to be enabled*.
#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M1 3h14v3h-14zM1 7h14v3h-14zM1 11h14v3h-14z" /></svg>
   }
}