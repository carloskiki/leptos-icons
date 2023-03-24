#[cfg(feature = "BiLogosDropbox")]
use leptos::*;
#[cfg(feature = "BiLogosDropbox")]
///This icon requires the feature `BiLogosDropbox` to be enabled.
#[component]
pub fn Dropbox(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M7.004 3.5 2 6.689l5.004 3.186 5.002-3.186zm10.005 0-5.003 3.189 5.003 3.186 5.003-3.186zM2 13.062l5.004 3.187 5.002-3.187-5.002-3.187zm15.009-3.187-5.003 3.187 5.003 3.187 5.003-3.187zM7.004 17.311l5.002 3.189 5.003-3.189-5.003-3.186z"
        /> < title > { title } < / title > < / svg >
    }
}
