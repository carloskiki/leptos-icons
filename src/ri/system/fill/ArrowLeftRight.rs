#[cfg(feature = "RiSystemFillArrowLeftRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowLeftRight")]
/// *This icon requires the feature* `RiSystemFillArrowLeftRight` *to be enabled*.
#[component]
pub fn ArrowLeftRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M16 16v-4l5 5-5 5v-4H4v-2h12zM8 2v3.999L20 6v2H8v4L3 7l5-5z" /></g></svg>
   }
}