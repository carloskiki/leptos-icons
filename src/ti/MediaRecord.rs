#[cfg(feature = "TiMediaRecord")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaRecord")]
/// *This icon requires the feature* `TiMediaRecord` *to be enabled*.
#[component]
pub fn MediaRecord(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M18 12c0-1.657-.672-3.157-1.757-4.243-1.086-1.085-2.586-1.757-4.243-1.757-1.656 0-3.156.672-4.242 1.757-1.086 1.086-1.758 2.586-1.758 4.243 0 1.656.672 3.156 1.758 4.242s2.586 1.758 4.242 1.758c1.657 0 3.157-.672 4.243-1.758 1.085-1.086 1.757-2.586 1.757-4.242z" /></svg>
   }
}