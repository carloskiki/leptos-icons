#[cfg(feature = "SiElasticcloud")]
use leptos::*;
#[cfg(feature = "SiElasticcloud")]
///This icon requires the feature `SiElasticcloud` to be enabled.
#[component]
pub fn Elasticcloud(
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
        "M13.318 0c-6.628 0-12 5.372-12 12 0 2.008.495 3.9 1.368 5.563a14.299 14.299 0 0 1 5.09-3.664c.307-.13.624-.22.948-.28A4.842 4.842 0 0 1 8.443 12a4.875 4.875 0 0 1 7.494-4.11 2.218 2.218 0 0 0 2.055.164 12.047 12.047 0 0 0 4.69-3.554A11.975 11.975 0 0 0 13.318 0zM9.426 15.77c-.266.01-.531.069-.783.175a12.044 12.044 0 0 0-4.69 3.555c2.2 2.742 5.576 4.5 9.365 4.5 3.789 0 7.165-1.758 9.364-4.5a12.048 12.048 0 0 0-4.69-3.555 2.217 2.217 0 0 0-2.055.165 4.845 4.845 0 0 1-2.62.765 4.846 4.846 0 0 1-2.618-.765 2.193 2.193 0 0 0-1.273-.34z"
        /> < title > { title } < / title > < / svg >
    }
}
