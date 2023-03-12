#[cfg(feature = "RiSystemFillArrowDropDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowDropDown")]
/// *This icon requires the feature* `RiSystemFillArrowDropDown` *to be enabled*.
#[component]
pub fn ArrowDropDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 14l-4-4h8z" /></g></svg>
   }
}