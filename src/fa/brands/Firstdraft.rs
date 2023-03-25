#[cfg(feature = "FaBrandsFirstdraft")]
use leptos::*;
#[cfg(feature = "FaBrandsFirstdraft")]
///This icon requires the feature `FaBrandsFirstdraft` to be enabled.
#[component]
pub fn Firstdraft(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M384 192h-64v128H192v128H0v-25.6h166.4v-128h128v-128H384V192zm-25.6 38.4v128h-128v128H64V512h192V384h128V230.4h-25.6zm25.6 192h-89.6V512H320v-64h64v-25.6zM0 0v384h128V256h128V128h128V0H0z"
        /> < title > { title } < / title > < / svg >
    }
}
