#[cfg(feature = "SiFlipboard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFlipboard")]
/// *This icon requires the feature* `SiFlipboard` *to be enabled*.
#[component]
pub fn Flipboard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v24h24V0H0zm19.2 9.6h-4.8v4.8H9.6v4.8H4.8V4.8h14.4v4.8z" /></svg>
   }
}