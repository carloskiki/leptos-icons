#[cfg(feature = "RiSystemFillArrowRightS")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowRightS")]
/// *This icon requires the feature* `RiSystemFillArrowRightS` *to be enabled*.
#[component]
pub fn ArrowRightS(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M16 12l-6 6V6z" /></g></svg>
   }
}