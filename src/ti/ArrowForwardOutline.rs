#[cfg(feature = "TiArrowForwardOutline")]
use leptos::*;
#[cfg(feature = "TiArrowForwardOutline")]
///This icon requires the feature `TiArrowForwardOutline` to be enabled.
#[component]
pub fn ArrowForwardOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 19.999c-.096 0-.191-.015-.286-.042-.424-.126-.714-.516-.714-.958v-1c0-4.8 3.381-8.864 8-9.796v-1.704c0-.534.208-1.036.585-1.414.756-.757 2.075-.756 2.829-.001l6.288 6.203c.191.188.298.443.298.712s-.107.524-.298.712l-6.293 6.207c-.746.746-2.067.751-2.823-.005-.378-.378-.586-.88-.586-1.414v-1.437c-2.495.201-4.523.985-6.164 3.484-.19.288-.505.453-.836.453zm8-5.989l1-.01v3.499l5.576-5.5-5.576-5.496v3.497s-.384-.004-.891.052c-3.416.378-6.125 2.864-6.892 6.08 2.121-1.728 4.551-2.066 6.783-2.122z"
        /> < title > { title } < / title > < / svg >
    }
}
