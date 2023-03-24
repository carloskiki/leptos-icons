#[cfg(feature = "BiLogosKickstarter")]
use leptos::*;
#[cfg(feature = "BiLogosKickstarter")]
///This icon requires the feature `BiLogosKickstarter` to be enabled.
#[component]
pub fn Kickstarter(
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
        "m10.198 9.127 3.309-4.799c.628-.902 1.434-1.354 2.427-1.354.806 0 1.506.287 2.097.861.59.574.885 1.26.885 2.059 0 .59-.155 1.113-.469 1.563l-2.983 4.34 3.649 4.627c.363.461.547 1 .547 1.617 0 .816-.286 1.518-.86 2.105a2.81 2.81 0 0 1-2.084.879c-.895 0-1.577-.291-2.047-.873L10.2 14.574v3.074c0 .879-.152 1.561-.457 2.049-.555.885-1.356 1.328-2.417 1.328-.963 0-1.711-.328-2.24-.977-.496-.602-.744-1.396-.744-2.387V6.258c0-.936.253-1.709.755-2.318.525-.643 1.255-.965 2.186-.965.887 0 1.624.322 2.216.965.329.357.537.717.625 1.082.053.227.079.646.079 1.266v2.84h-.005z"
        /> < title > { title } < / title > < / svg >
    }
}
