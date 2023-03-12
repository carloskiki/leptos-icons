#[cfg(feature = "RiSystemFillLoader5")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillLoader5")]
/// *This icon requires the feature* `RiSystemFillLoader5` *to be enabled*.
#[component]
pub fn Loader5(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 3a9 9 0 0 1 9 9h-2a7 7 0 0 0-7-7V3z" /></g></svg>
   }
}