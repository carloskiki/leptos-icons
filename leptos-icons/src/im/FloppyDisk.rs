#[cfg(feature = "ImFloppyDisk")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFloppyDisk")]
/// *This icon requires the feature* `ImFloppyDisk` *to be enabled*.
#[component]
pub fn FloppyDisk(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 0h-14v16h16v-14l-2-2zM8 2h2v4h-2v-4zM14 14h-12v-12h1v5h9v-5h1.172l0.828 0.828v11.172z" /></svg>
   }
}