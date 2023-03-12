#[cfg(feature = "RiSystemLineClose")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineClose")]
/// *This icon requires the feature* `RiSystemLineClose` *to be enabled*.
#[component]
pub fn Close(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 10.586l4.95-4.95 1.414 1.414-4.95 4.95 4.95 4.95-1.414 1.414-4.95-4.95-4.95 4.95-1.414-1.414 4.95-4.95-4.95-4.95L7.05 5.636z" /></g></svg>
   }
}