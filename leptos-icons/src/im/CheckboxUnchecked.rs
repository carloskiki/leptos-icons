#[cfg(feature = "ImCheckboxUnchecked")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImCheckboxUnchecked")]
/// *This icon requires the feature* `ImCheckboxUnchecked` *to be enabled*.
#[component]
pub fn CheckboxUnchecked(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 0h-12c-1.1 0-2 0.9-2 2v12c0 1.1 0.9 2 2 2h12c1.1 0 2-0.9 2-2v-12c0-1.1-0.9-2-2-2zM14 14h-12v-12h12v12z" /></svg>
   }
}