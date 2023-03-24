#[cfg(feature = "SiNaver")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiNaver")]
/// *This icon requires the feature* `SiNaver` *to be enabled*.
#[component]
pub fn Naver(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M16.273 12.845 7.376 0H0v24h7.726V11.156L16.624 24H24V0h-7.727v12.845Z" /></svg>
   }
}