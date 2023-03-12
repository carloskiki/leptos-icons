#[cfg(feature = "SiLinuxfoundation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiLinuxfoundation")]
/// *This icon requires the feature* `SiLinuxfoundation` *to be enabled*.
#[component]
pub fn Linuxfoundation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M4.8 19.2h9.6V24H0V9.6h4.8v9.6zM0 0v7.2h4.8V4.822h14.4V19.2h-2.4V24H24V0H0z" /></svg>
   }
}