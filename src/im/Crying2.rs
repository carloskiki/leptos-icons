#[cfg(feature = "ImCrying2")]
use leptos::*;
#[cfg(feature = "ImCrying2")]
///This icon requires the feature `ImCrying2` to be enabled.
#[component]
pub fn Crying2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM5 11.5c0 0.276-0.224 0.5-0.5 0.5s-0.5-0.224-0.5-0.5v-1c0-0.276 0.224-0.5 0.5-0.5s0.5 0.224 0.5 0.5v1zM5 8.5c0 0.276-0.224 0.5-0.5 0.5s-0.5-0.224-0.5-0.5v-1c0-0.276 0.224-0.5 0.5-0.5s0.5 0.224 0.5 0.5v1zM5.5 6h-2c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h2c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5zM9.854 13.229c-0.098 0.098-0.226 0.146-0.354 0.146s-0.256-0.049-0.354-0.146c-0.072-0.072-0.46-0.229-1.146-0.229s-1.075 0.157-1.146 0.229c-0.195 0.195-0.512 0.195-0.707 0s-0.195-0.512 0-0.707c0.471-0.471 1.453-0.521 1.854-0.521s1.383 0.051 1.854 0.521c0.195 0.195 0.195 0.512 0 0.707zM12 11.5c0 0.276-0.224 0.5-0.5 0.5s-0.5-0.224-0.5-0.5v-1c0-0.276 0.224-0.5 0.5-0.5s0.5 0.224 0.5 0.5v1zM12 8.5c0 0.276-0.224 0.5-0.5 0.5s-0.5-0.224-0.5-0.5v-1c0-0.276 0.224-0.5 0.5-0.5s0.5 0.224 0.5 0.5v1zM12.5 6h-2c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h2c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5z"
        /> < title > { title } < / title > < / svg >
    }
}
