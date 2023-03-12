#[cfg(feature = "RiSystemFillArrowDownS")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowDownS")]
/// *This icon requires the feature* `RiSystemFillArrowDownS` *to be enabled*.
#[component]
pub fn ArrowDownS(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 16l-6-6h12z" /></g></svg>
   }
}