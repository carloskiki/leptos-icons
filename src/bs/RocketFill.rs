#[cfg(feature = "BsRocketFill")]
use leptos::*;
#[cfg(feature = "BsRocketFill")]
///This icon requires the feature `BsRocketFill` to be enabled.
#[component]
pub fn RocketFill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-rocket-fill" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.175 1.991c.81 1.312 1.583 3.43 1.778 6.819l1.5 1.83A2.5 2.5 0 0 1 14 12.202V15.5a.5.5 0 0 1-.9.3l-1.125-1.5c-.166-.222-.42-.4-.752-.57-.214-.108-.414-.192-.627-.282l-.196-.083C9.7 13.793 8.85 14 8 14c-.85 0-1.7-.207-2.4-.635-.068.03-.133.057-.198.084-.211.089-.411.173-.625.281-.332.17-.586.348-.752.57L2.9 15.8a.5.5 0 0 1-.9-.3v-3.298a2.5 2.5 0 0 1 .548-1.562l.004-.005L4.049 8.81c.197-3.323.969-5.434 1.774-6.756.466-.767.94-1.262 1.31-1.57a3.67 3.67 0 0 1 .601-.41A.549.549 0 0 1 8 0c.101 0 .17.027.25.064.037.017.086.041.145.075.118.066.277.167.463.315.373.297.85.779 1.317 1.537ZM9.5 6c0-1.105-.672-2-1.5-2s-1.5.895-1.5 2S7.172 8 8 8s1.5-.895 1.5-2Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 14.5c.5 0 .999-.046 1.479-.139L8.4 15.8a.5.5 0 0 1-.8 0l-1.079-1.439c.48.093.98.139 1.479.139Z"
        /> < title > { title } < / title > < / svg >
    }
}
