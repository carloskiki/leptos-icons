#[cfg(feature = "AiOutlinedCaretDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedCaretDown")]
/// *This icon requires the feature* `AiOutlinedCaretDown` *to be enabled*.
#[component]
pub fn CaretDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024"><path d="M840.4 300H183.6c-19.7 0-30.7 20.8-18.5 35l328.4 380.8c9.4 10.9 27.5 10.9 37 0L858.9 335c12.2-14.2 1.2-35-18.5-35z" /></svg>
   }
}