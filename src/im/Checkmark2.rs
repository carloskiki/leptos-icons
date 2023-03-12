#[cfg(feature = "ImCheckmark2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImCheckmark2")]
/// *This icon requires the feature* `ImCheckmark2` *to be enabled*.
#[component]
pub fn Checkmark2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6.21 14.339l-6.217-6.119 3.084-3.035 3.133 3.083 6.713-6.607 3.084 3.035-9.797 9.643zM1.686 8.22l4.524 4.453 8.104-7.976-1.391-1.369-6.713 6.607-3.133-3.083-1.391 1.369z" /></svg>
   }
}