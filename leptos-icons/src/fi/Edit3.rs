#[cfg(feature = "FiEdit3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiEdit3")]
/// *This icon requires the feature* `FiEdit3` *to be enabled*.
#[component]
pub fn Edit3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h9" /><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" /></svg>
   }
}