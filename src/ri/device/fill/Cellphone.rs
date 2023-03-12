#[cfg(feature = "RiDeviceFillCellphone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDeviceFillCellphone")]
/// *This icon requires the feature* `RiDeviceFillCellphone` *to be enabled*.
#[component]
pub fn Cellphone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M7 2h11a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V0h2v2zm0 2v5h10V4H7z" /></g></svg>
   }
}