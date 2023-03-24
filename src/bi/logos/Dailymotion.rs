#[cfg(feature = "BiLogosDailymotion")]
use leptos::*;
#[cfg(feature = "BiLogosDailymotion")]
///This icon requires the feature `BiLogosDailymotion` to be enabled.
#[component]
pub fn Dailymotion(
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
        "M13.551 11.485a2.327 2.327 0 0 0-2.328 2.332c0 1.314 1.013 2.313 2.441 2.313l-.012.002c1.192 0 2.193-.983 2.193-2.28.001-1.349-1.001-2.367-2.294-2.367z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 3v18h18V3H3zm15.52 15.605h-2.682v-1.058c-.825.81-1.667 1.103-2.786 1.103-1.142 0-2.124-.371-2.947-1.114-1.086-.956-1.648-2.227-1.648-3.701 0-1.351.524-2.561 1.507-3.506.878-.859 1.946-1.298 3.139-1.298 1.14 0 2.018.385 2.647 1.192V6.118l2.77-.574v-.002l.002.003h-.002v13.06z"
        /> < title > { title } < / title > < / svg >
    }
}
