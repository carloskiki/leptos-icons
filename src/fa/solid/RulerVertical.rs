#[cfg(feature = "FaSolidRulerVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidRulerVertical")]
/// *This icon requires the feature* `FaSolidRulerVertical` *to be enabled*.
#[component]
pub fn RulerVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 512"><path d="M0 48C0 21.5 21.5 0 48 0H208c26.5 0 48 21.5 48 48V96H176c-8.8 0-16 7.2-16 16s7.2 16 16 16h80v64H176c-8.8 0-16 7.2-16 16s7.2 16 16 16h80v64H176c-8.8 0-16 7.2-16 16s7.2 16 16 16h80v64H176c-8.8 0-16 7.2-16 16s7.2 16 16 16h80v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V48z" /></svg>
   }
}