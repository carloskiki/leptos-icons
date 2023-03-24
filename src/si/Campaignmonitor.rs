#[cfg(feature = "SiCampaignmonitor")]
use leptos::*;
#[cfg(feature = "SiCampaignmonitor")]
///This icon requires the feature `SiCampaignmonitor` to be enabled.
#[component]
pub fn Campaignmonitor(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M23.836 4.27c-.29-.413-.86-.515-1.273-.226L.163 19.73c.167.235.437.39.747.39h22.18c.503 0 .91-.41.91-.914V4.78c-.004-.176-.058-.352-.164-.51zm-22.4-.226c-.413-.29-.982-.19-1.272.226-.107.154-.162.332-.164.51v14.45l10.664-8.736-9.227-6.45v-.002z"
        /> < title > { title } < / title > < / svg >
    }
}
