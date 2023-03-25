#[cfg(feature = "VsServerEnvironment")]
use leptos::*;
#[cfg(feature = "VsServerEnvironment")]
///This icon requires the feature `VsServerEnvironment` to be enabled.
#[component]
pub fn ServerEnvironment(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 3h4v1H6V3zm0 6h4v1H6V9zm0 2h4v1H6v-1zm9.14 5H.86l1.25-5H4V2a.95.95 0 0 1 .078-.383c.052-.12.123-.226.211-.32a.922.922 0 0 1 .32-.219A1.01 1.01 0 0 1 5 1h6a.95.95 0 0 1 .383.078c.12.052.226.123.32.211a.922.922 0 0 1 .219.32c.052.125.078.256.078.391v9h1.89l1.25 5zM5 13h6V2H5v11zm8.86 2l-.75-3H12v2H4v-2H2.89l-.75 3h11.72z"
        /> < title > { title } < / title > < / svg >
    }
}
