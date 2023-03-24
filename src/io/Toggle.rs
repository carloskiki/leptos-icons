#[cfg(feature = "IoToggle")]
use leptos::*;
#[cfg(feature = "IoToggle")]
///This icon requires the feature `IoToggle` to be enabled.
#[component]
pub fn Toggle(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,112H144C64.6,112,0,176.6,0,256S64.6,400,144,400H368c79.4,0,144-64.6,144-144S447.4,112,368,112Zm0,256A112,112,0,1,1,480,256,112.12,112.12,0,0,1,368,368Z"
        /> < title > { title } < / title > < / svg >
    }
}
