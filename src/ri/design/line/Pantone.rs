#[cfg(feature = "RiDesignLinePantone")]
use leptos::*;
#[cfg(feature = "RiDesignLinePantone")]
///This icon requires the feature `RiDesignLinePantone` to be enabled.
#[component]
pub fn Pantone(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M5.764 8l-.295-.73a1 1 0 0 1 .553-1.302l9.272-3.746a1 1 0 0 1 1.301.552l5.62 13.908a1 1 0 0 1-.553 1.302L12.39 21.73a1 1 0 0 1-1.302-.553L11 20.96V21H7a1 1 0 0 1-1-1v-.27l-3.35-1.353a1 1 0 0 1-.552-1.302L5.764 8zM8 19h2.209L8 13.533V19zm-2-6.244l-1.673 4.141L6 17.608v-4.852zm1.698-5.309l4.87 12.054 7.418-2.997-4.87-12.053-7.418 2.996zm2.978 2.033a1 1 0 1 1-.749-1.855 1 1 0 0 1 .75 1.855z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
