#[cfg(feature = "BsSlashLg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsSlashLg")]
/// *This icon requires the feature* `BsSlashLg` *to be enabled*.
#[component]
pub fn SlashLg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-slash-lg" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M13.854 2.146a.5.5 0 0 1 0 .708l-11 11a.5.5 0 0 1-.708-.708l11-11a.5.5 0 0 1 .708 0Z" /></svg>
   }
}