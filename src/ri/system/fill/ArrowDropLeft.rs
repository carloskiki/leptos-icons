#[cfg(feature = "RiSystemFillArrowDropLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowDropLeft")]
/// *This icon requires the feature* `RiSystemFillArrowDropLeft` *to be enabled*.
#[component]
pub fn ArrowDropLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 12l4-4v8z" /></g></svg>
   }
}