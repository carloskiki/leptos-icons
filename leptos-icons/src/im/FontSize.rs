#[cfg(feature = "ImFontSize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFontSize")]
/// *This icon requires the feature* `ImFontSize` *to be enabled*.
#[component]
pub fn FontSize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M1 8h6v2h-2v6h-2v-6h-2zM15 4h-3.934v12h-2.133v-12h-3.934v-2h10z" /></svg>
   }
}