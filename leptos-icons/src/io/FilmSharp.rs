#[cfg(feature = "IoFilmSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFilmSharp")]
/// *This icon requires the feature* `IoFilmSharp` *to be enabled*.
#[component]
pub fn FilmSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M480,80H32V432H480ZM112,352v48H64V352Zm0-80v48H64V272Zm0-80v48H64V192Zm0-80v48H64V112ZM368,272H144V240H368Zm80,80v48H400V352Zm0-80v48H400V272Zm0-80v48H400V192Zm0-80v48H400V112Z" /></svg>
   }
}