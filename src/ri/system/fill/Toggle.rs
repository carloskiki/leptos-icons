#[cfg(feature = "RiSystemFillToggle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillToggle")]
/// *This icon requires the feature* `RiSystemFillToggle` *to be enabled*.
#[component]
pub fn Toggle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M8 5h8a7 7 0 0 1 0 14H8A7 7 0 0 1 8 5zm8 10a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" /></g></svg>
   }
}