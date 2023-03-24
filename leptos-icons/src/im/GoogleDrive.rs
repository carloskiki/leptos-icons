#[cfg(feature = "ImGoogleDrive")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImGoogleDrive")]
/// *This icon requires the feature* `ImGoogleDrive` *to be enabled*.
#[component]
pub fn GoogleDrive(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6.844 10l-2.884 5h9.072l2.884-5z" /><path fill="#000000" d="M15.506 9l-4.619-8h-5.775l4.619 8z" /><path fill="#000000" d="M4.534 2l-4.534 7.856 2.888 5 4.534-7.856z" /></svg>
   }
}