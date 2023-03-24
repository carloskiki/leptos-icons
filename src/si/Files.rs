#[cfg(feature = "SiFiles")]
use leptos::*;
#[cfg(feature = "SiFiles")]
///This icon requires the feature `SiFiles` to be enabled.
#[component]
pub fn Files(
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
        "M12.367 2.453a.822.822 0 0 0-.576.238L.241 14.213a.822.822 0 0 0-.241.584v.066c0-.323.209-.608.516-.709l7.275-2.318a2.437 2.437 0 0 0 1.584-1.592l2.318-7.267a.757.757 0 0 1 .719-.524zM0 14.863v5.047c0 .904.733 1.637 1.637 1.637h20.726c.904 0 1.637-.733 1.637-1.637V4.09c0-.904-.733-1.637-1.637-1.637h-9.951v.5l.088 9.861c.01 1.175-.962 2.14-2.137 2.14L0 14.862zM12 3.66l-2.148 6.735v.001a2.94 2.94 0 0 1-1.909 1.916l-6.716 2.141h9.136c.905 0 1.638-.734 1.637-1.639zm-10.363.975c-.905 0-1.638.734-1.637 1.638v7.473l9.135-9.111Z"
        /> < title > { title } < / title > < / svg >
    }
}
