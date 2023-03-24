#[cfg(feature = "OcLgHubot")]
use leptos::*;
#[cfg(feature = "OcLgHubot")]
///This icon requires the feature `OcLgHubot` to be enabled.
#[component]
pub fn Hubot(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 13C0 6.373 5.373 1 12 1s12 5.373 12 12v8.657a.75.75 0 0 1-1.5 0V13c0-5.799-4.701-10.5-10.5-10.5S1.5 7.201 1.5 13v8.657a.75.75 0 0 1-1.5 0V13Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 19.75a.75.75 0 0 1 .75-.75h6.5a.75.75 0 0 1 0 1.5h-6.5a.75.75 0 0 1-.75-.75ZM5.25 9.5h13.5c.966 0 1.75.784 1.75 1.75v3.5a1.75 1.75 0 0 1-1.75 1.75H5.25a1.75 1.75 0 0 1-1.75-1.75v-3.5c0-.966.784-1.75 1.75-1.75Zm.22 1.47a.75.75 0 0 0 0 1.06l3 3a.75.75 0 0 0 1.06 0L12 12.56l2.47 2.47a.75.75 0 0 0 1.06 0l3-3a.749.749 0 0 0-.326-1.275.749.749 0 0 0-.734.215L15 13.44l-2.47-2.47a.75.75 0 0 0-1.06 0L9 13.44l-2.47-2.47a.75.75 0 0 0-1.06 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
