#[cfg(feature = "ImBarcode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBarcode")]
/// *This icon requires the feature* `ImBarcode` *to be enabled*.
#[component]
pub fn Barcode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 2h2v10h-2zM3 2h1v10h-1zM5 2h1v10h-1zM8 2h1v10h-1zM12 2h1v10h-1zM15 2h1v10h-1zM10 2h0.5v10h-0.5zM7 2h0.5v10h-0.5zM13.5 2h0.5v10h-0.5zM0 13h1v1h-1zM3 13h1v1h-1zM5 13h1v1h-1zM10 13h1v1h-1zM15 13h1v1h-1zM12 13h2v1h-2zM7 13h2v1h-2z" /></svg>
   }
}