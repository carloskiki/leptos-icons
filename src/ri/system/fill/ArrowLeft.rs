#[cfg(feature = "RiSystemFillArrowLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowLeft")]
/// *This icon requires the feature* `RiSystemFillArrowLeft` *to be enabled*.
#[component]
pub fn ArrowLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 13v7l-8-8 8-8v7h8v2z" /></g></svg>
   }
}