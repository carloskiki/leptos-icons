#[cfg(feature = "IoCaretUpCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretUpCircle")]
/// *This icon requires the feature* `IoCaretUpCircle` *to be enabled*.
#[component]
pub fn CaretUpCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Zm74.14,252H181.86a16,16,0,0,1-12.29-26.23l74.13-89.09a16,16,0,0,1,24.6,0l74.13,89.09A16,16,0,0,1,330.14,300Z" /></svg>
   }
}