#[cfg(feature = "RiDesignFillTableAlt")]
use leptos::*;
#[cfg(feature = "RiDesignFillTableAlt")]
///This icon requires the feature `RiDesignFillTableAlt` to be enabled.
#[component]
pub fn TableAlt(
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
        "M7 14V3H3a1 1 0 0 0-1 1v10h5zm8 0V3H9v11h6zm7 0V4a1 1 0 0 0-1-1h-4v11h5zm-1 7a1 1 0 0 0 1-1v-4H2v4a1 1 0 0 0 1 1h18z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
