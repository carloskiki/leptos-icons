#[cfg(feature = "ImBlocked")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBlocked")]
/// *This icon requires the feature* `ImBlocked` *to be enabled*.
#[component]
pub fn Blocked(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.657 2.343c-1.511-1.511-3.52-2.343-5.657-2.343s-4.146 0.832-5.657 2.343c-1.511 1.511-2.343 3.52-2.343 5.657s0.832 4.146 2.343 5.657c1.511 1.511 3.52 2.343 5.657 2.343s4.146-0.832 5.657-2.343c1.511-1.511 2.343-3.52 2.343-5.657s-0.832-4.146-2.343-5.657zM14 8c0 1.294-0.412 2.494-1.111 3.475l-8.364-8.364c0.981-0.699 2.181-1.111 3.475-1.111 3.308 0 6 2.692 6 6zM2 8c0-1.294 0.412-2.494 1.111-3.475l8.364 8.364c-0.981 0.699-2.181 1.111-3.475 1.111-3.308 0-6-2.692-6-6z" /></svg>
   }
}