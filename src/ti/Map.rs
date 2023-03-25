#[cfg(feature = "TiMap")]
use leptos::*;
#[cfg(feature = "TiMap")]
///This icon requires the feature `TiMap` to be enabled.
#[component]
pub fn Map(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.383 3.076c-.373-.155-.804-.069-1.09.217l-3.867 3.867-4.301-3.441c-.396-.316-.973-.287-1.332.074l-4.5 4.5c-.188.187-.293.441-.293.707v10c0 .404.243.77.617.924.124.053.254.076.383.076.26 0 .516-.102.707-.293l3.867-3.867 4.301 3.441c.396.316.971.285 1.332-.074l4.5-4.5c.188-.187.293-.441.293-.707v-10c0-.404-.243-.77-.617-.924zm-13.383 13.51v-7.172l3-3v7.24c-.07.043-3 2.932-3 2.932zm4.125-2.867l-.125-.068v-7.469s3.959 3.143 4 3.166v7.473l-3.875-3.102zm7.875-.133l-3 3v-7.236c.07-.043 3-2.936 3-2.936v7.172z"
        /> < title > { title } < / title > < / svg >
    }
}
