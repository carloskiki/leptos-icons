#[cfg(feature = "CgArrangeBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgArrangeBack")]
/// *This icon requires the feature* `CgArrangeBack` *to be enabled*.
#[component]
pub fn ArrangeBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 3H11V7H17V13H21V21H13V17H7V11H3V3ZM15 13H13V15H9V11H11V9H15V13Z" fill="currentColor" /></svg>
   }
}