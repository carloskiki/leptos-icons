#[cfg(feature = "RiSystemFillArrowDropRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowDropRight")]
/// *This icon requires the feature* `RiSystemFillArrowDropRight` *to be enabled*.
#[component]
pub fn ArrowDropRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14 12l-4 4V8z" /></g></svg>
   }
}