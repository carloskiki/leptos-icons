#[cfg(feature = "OcLgRepoPush")]
use leptos::*;
#[cfg(feature = "OcLgRepoPush")]
///This icon requires the feature `OcLgRepoPush` to be enabled.
#[component]
pub fn RepoPush(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4.75 0A2.75 2.75 0 0 0 2 2.75v16.5A2.75 2.75 0 0 0 4.75 22h11a.75.75 0 0 0 0-1.5h-11c-.69 0-1.25-.56-1.25-1.25V18A1.5 1.5 0 0 1 5 16.5h7.25a.75.75 0 0 0 0-1.5H5c-.546 0-1.059.146-1.5.401V2.75c0-.69.56-1.25 1.25-1.25H18.5v7a.75.75 0 0 0 1.5 0V.75a.75.75 0 0 0-.75-.75H4.75Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m20 13.903 2.202 2.359a.75.75 0 0 0 1.096-1.024l-3.5-3.75a.75.75 0 0 0-1.096 0l-3.5 3.75a.75.75 0 1 0 1.096 1.024l2.202-2.36v9.348a.75.75 0 0 0 1.5 0v-9.347Z"
        /> < title > { title } < / title > < / svg >
    }
}
