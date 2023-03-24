#[cfg(feature = "SiOpenverse")]
use leptos::*;
#[cfg(feature = "SiOpenverse")]
///This icon requires the feature `SiOpenverse` to be enabled.
#[component]
pub fn Openverse(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.882 1.018C2.182 1.018 0 3.214 0 5.932c0 2.704 2.182 4.915 4.882 4.915Zm7.118 0c-2.686 0-4.882 2.196-4.882 4.914 0 2.704 2.182 4.915 4.882 4.915zm7.118 0c-2.696 0-4.881 2.2-4.881 4.914 0 2.714 2.185 4.915 4.88 4.915 2.697 0 4.883-2.2 4.883-4.915 0-2.714-2.186-4.914-4.882-4.914zm0 12.093c-2.696 0-4.881 2.2-4.881 4.915 0 2.714 2.185 4.914 4.88 4.914 2.697 0 4.883-2.2 4.883-4.914 0-2.714-2.186-4.915-4.882-4.915ZM12 13.126c-2.686 0-4.882 2.196-4.882 4.9S9.3 22.94 12 22.94zm-7.118.04c-2.7 0-4.882 2.197-4.882 4.9 0 2.719 2.182 4.916 4.882 4.916Z"
        /> < title > { title } < / title > < / svg >
    }
}
