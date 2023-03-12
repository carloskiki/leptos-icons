#[cfg(feature = "RiSystemFillArrowUpDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillArrowUpDown")]
/// *This icon requires the feature* `RiSystemFillArrowUpDown` *to be enabled*.
#[component]
pub fn ArrowUpDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 8H8.001L8 20H6V8H2l5-5 5 5zm10 8l-5 5-5-5h4V4h2v12h4z" /></g></svg>
   }
}