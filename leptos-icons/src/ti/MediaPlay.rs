#[cfg(feature = "TiMediaPlay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaPlay")]
/// *This icon requires the feature* `TiMediaPlay` *to be enabled*.
#[component]
pub fn MediaPlay(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M10.396 18.433c2.641-2.574 6.604-6.433 6.604-6.433s-3.963-3.859-6.604-6.433c-.363-.349-.853-.567-1.396-.567-1.104 0-2 .896-2 2v10c0 1.104.896 2 2 2 .543 0 1.033-.218 1.396-.567z" /></svg>
   }
}