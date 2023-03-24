#[cfg(feature = "ImCopy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImCopy")]
/// *This icon requires the feature* `ImCopy` *to be enabled*.
#[component]
pub fn Copy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M10 4v-4h-7l-3 3v9h6v4h10v-12h-6zM3 1.414v1.586h-1.586l1.586-1.586zM1 11v-7h3v-3h5v3l-3 3v4h-5zM9 5.414v1.586h-1.586l1.586-1.586zM15 15h-8v-7h3v-3h5v10z" /></svg>
   }
}