#[cfg(feature = "RiBusinessLineMailClose")]
use leptos::*;
#[cfg(feature = "RiBusinessLineMailClose")]
///This icon requires the feature `RiBusinessLineMailClose` to be enabled.
#[component]
pub fn MailClose(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M22 14h-2V7.238l-7.928 7.1L4 7.216V19h11v2H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v10zM4.511 5l7.55 6.662L19.502 5H4.511zm16.903 14l2.122 2.121-1.415 1.415L20 20.414l-2.121 2.122-1.415-1.415L18.586 19l-2.122-2.121 1.415-1.415L20 17.586l2.121-2.122 1.415 1.415L21.414 19z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
