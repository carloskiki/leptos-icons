#[cfg(feature = "RiDesignFillShape2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignFillShape2")]
/// *This icon requires the feature* `RiDesignFillShape2` *to be enabled*.
#[component]
pub fn Shape2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 2h5v5H2V2zm0 15h5v5H2v-5zM17 2h5v5h-5V2zm0 15h5v5h-5v-5zM8 4h8v2H8V4zM4 8h2v8H4V8zm14 0h2v8h-2V8zM8 18h8v2H8v-2z" /></g></svg>
   }
}