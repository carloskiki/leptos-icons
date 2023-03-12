#[cfg(feature = "RiDevelopmentFillCursor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDevelopmentFillCursor")]
/// *This icon requires the feature* `RiDevelopmentFillCursor` *to be enabled*.
#[component]
pub fn Cursor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13.91 12.36L17 20.854l-2.818 1.026-3.092-8.494-4.172 3.156 1.49-14.909 10.726 10.463z" /></g></svg>
   }
}