#[cfg(feature = "BiRegularHdd")]
use leptos::*;
#[cfg(feature = "BiRegularHdd")]
///This icon requires the feature `BiRegularHdd` to be enabled.
#[component]
pub fn Hdd(
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
        "m21.983 13.821-1.851-10.18A1.998 1.998 0 0 0 18.165 2H5.835a2 2 0 0 0-1.968 1.643l-1.85 10.178.019.003c-.012.06-.036.114-.036.176v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c0-.063-.024-.116-.035-.176l.018-.003zM5.835 4h12.331l1.637 9H4.198l1.637-9zM4 19v-4h16l.002 4H4z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M17 16h2v2h-2zm-3 0h2v2h-2z"
        /> < title > { title } < / title > < / svg >
    }
}
