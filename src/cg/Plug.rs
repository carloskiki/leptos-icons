#[cfg(feature = "CgPlug")]
use leptos::*;
#[cfg(feature = "CgPlug")]
///This icon requires the feature `CgPlug` to be enabled.
#[component]
pub fn Plug(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M9 2C8.44771 2 8 2.44772 8 3V5C8 5.55228 8.44771 6 9 6C9.55229 6 10 5.55228 10 5V3C10 2.44772 9.55229 2 9 2ZM8 9H16V11C16 13.2091 14.2091 15 12 15C9.79086 15 8 13.2091 8 11V9ZM13 16.917C15.8377 16.441 18 13.973 18 11V7H6V11C6 13.973 8.16229 16.441 11 16.917V22C11 22.5523 11.4477 23 12 23C12.5523 23 13 22.5523 13 22V16.917ZM14 3C14 2.44772 14.4477 2 15 2C15.5523 2 16 2.44772 16 3V5C16 5.55228 15.5523 6 15 6C14.4477 6 14 5.55228 14 5V3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
