#[cfg(feature = "RiMediaFillStopMini")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillStopMini")]
/// *This icon requires the feature* `RiMediaFillStopMini` *to be enabled*.
#[component]
pub fn StopMini(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 7v10a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1z" /></g></svg>
   }
}