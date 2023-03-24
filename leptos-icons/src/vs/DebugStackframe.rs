#[cfg(feature = "VsDebugStackframe")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDebugStackframe")]
/// *This icon requires the feature* `VsDebugStackframe` *to be enabled*.
#[component]
pub fn DebugStackframe(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"><path d="M14.5 7.15l-4.26-4.74L9.31 2H4.25L3 3.25v9.48l1.25 1.25h5.06l.93-.42 4.26-4.74V7.15zm-5.19 5.58H4.25V3.25h5.06l4.26 4.73-4.26 4.75z" /></svg>
   }
}