#[cfg(feature = "ImTablet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTablet")]
/// *This icon requires the feature* `ImTablet` *to be enabled*.
#[component]
pub fn Tablet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M12.5 0h-10c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h10c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM7.5 15.5c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5 0.5 0.224 0.5 0.5-0.224 0.5-0.5 0.5zM12 14h-9v-12h9v12z" /></svg>
   }
}