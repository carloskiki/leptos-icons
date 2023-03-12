#[cfg(feature = "RiSystemFillAlert")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillAlert")]
/// *This icon requires the feature* `RiSystemFillAlert` *to be enabled*.
#[component]
pub fn Alert(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12.866 3l9.526 16.5a1 1 0 0 1-.866 1.5H2.474a1 1 0 0 1-.866-1.5L11.134 3a1 1 0 0 1 1.732 0zM11 16v2h2v-2h-2zm0-7v5h2V9h-2z" /></g></svg>
   }
}