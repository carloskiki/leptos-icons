#[cfg(feature = "ImForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImForward")]
/// *This icon requires the feature* `ImForward` *to be enabled*.
#[component]
pub fn Forward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4.096 0c-1.777 3.219-2.076 8.13 4.904 7.966v-3.966l6 6-6 6v-3.881c-8.359 0.218-9.29-7.378-4.904-12.119z" /></svg>
   }
}