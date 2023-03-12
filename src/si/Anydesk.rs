#[cfg(feature = "SiAnydesk")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAnydesk")]
/// *This icon requires the feature* `SiAnydesk` *to be enabled*.
#[component]
pub fn Anydesk(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8.322 3.677L0 12l8.322 8.323L16.645 12zm7.371.01l-1.849 1.85 6.49 6.456-6.49 6.49 1.85 1.817L24 11.993Z" /></svg>
   }
}