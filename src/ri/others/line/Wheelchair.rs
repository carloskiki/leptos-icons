#[cfg(feature = "RiOthersLineWheelchair")]
use leptos::*;
#[cfg(feature = "RiOthersLineWheelchair")]
///This icon requires the feature `RiOthersLineWheelchair` to be enabled.
#[component]
pub fn Wheelchair(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M8 10.341v2.194C6.804 13.227 6 14.52 6 16c0 2.21 1.79 4 4 4 1.48 0 2.773-.804 3.465-2h2.193c-.823 2.33-3.046 4-5.658 4-3.314 0-6-2.686-6-6 0-2.613 1.67-4.835 4-5.659zM12 17c-1.657 0-3-1.343-3-3v-4c0-1.044.534-1.964 1.343-2.501C9.533 6.964 9 6.044 9 5c0-1.657 1.343-3 3-3s3 1.343 3 3c0 1.044-.534 1.964-1.343 2.501C14.467 8.036 15 8.956 15 10v4.999l1.434.001c.648 0 1.253.314 1.626.836l.089.135 2.708 4.515-1.714 1.028L16.433 17 15 16.999 12 17zm0-8c-.552 0-1 .448-1 1v4c0 .552.448 1 1 1h.999L13 10c0-.552-.448-1-1-1zm0-5c-.552 0-1 .448-1 1s.448 1 1 1 1-.448 1-1-.448-1-1-1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
