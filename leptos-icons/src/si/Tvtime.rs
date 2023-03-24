#[cfg(feature = "SiTvtime")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiTvtime")]
/// *This icon requires the feature* `SiTvtime` *to be enabled*.
#[component]
pub fn Tvtime(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v24h24V0zm4.8 4.8h14.4v4.8h-4.8v9.6H9.6V9.6H4.8Z" /></svg>
   }
}