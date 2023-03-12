#[cfg(feature = "SiBandsintown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBandsintown")]
/// *This icon requires the feature* `SiBandsintown` *to be enabled*.
#[component]
pub fn Bandsintown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M18.783 0H24v12h-5.217V0zm-6.261 5h5.217v7h-5.217V5zM6.26 5h5.217v7H6.261V5zM24 24H0V0h5.217v19h13.566v-1H6.26v-5H24v11Z" /></svg>
   }
}