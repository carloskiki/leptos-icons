#[cfg(feature = "BiSolidWinkTongue")]
use leptos::*;
#[cfg(feature = "BiSolidWinkTongue")]
///This icon requires the feature `BiSolidWinkTongue` to be enabled.
#[component]
pub fn WinkTongue(
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
        "http://www.w3.org/2000/svg" d = "M14 19v-4h-4v4c0 1.103.897 2 2 2s2-.897 2-2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12c0 4.434 2.903 8.198 6.906 9.505A3.969 3.969 0 0 1 8 19v-2.499C6.412 15.027 6 13 6 13h12s-.411 2.027-2 3.501V19c0 .953-.349 1.816-.906 2.504C19.097 20.197 22 16.434 22 12c0-5.514-4.486-10-10-10zm-3.5 9a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 8.5 11zm4.5 0s.5-2 2.5-2c1.999 0 2.5 2 2.5 2h-5z"
        /> < title > { title } < / title > < / svg >
    }
}
