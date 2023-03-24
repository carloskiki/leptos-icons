#[cfg(feature = "SiAffinityphoto")]
use leptos::*;
#[cfg(feature = "SiAffinityphoto")]
///This icon requires the feature `SiAffinityphoto` to be enabled.
#[component]
pub fn Affinityphoto(
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
        "M10.44 0l-.48.831 5.88 10.185L22.2 0zm12.84 0l-8.577 14.856H24V.711A.72.72 0 0023.28 0zM9.42 1.767L5.76 8.106h7.32zm1.563 7.257h-.018c-.36.005-.7.216-.879.523l-1.083 1.88-.008.014a1.052 1.052 0 000 1.02 16710.388 16710.388 0 001.093 1.894c.184.31.53.5.885.501.002 0 1.38.002 2.067-.001.36-.005.699-.205.878-.512.364-.631.731-1.261 1.093-1.894.176-.314.17-.703-.007-1.011l-.01-.015-1.078-1.87-.006-.009a1.053 1.053 0 00-.879-.52h-.012zM5.22 9.04L0 18.082v.39l.003 4.871a.72.72 0 00.662.655L9.3 9.04zm2.94 3.845L1.736 24h12.84zm2.757 2.906L15.657 24h7.623a.72.72 0 00.72-.72v-7.488Z"
        /> < title > { title } < / title > < / svg >
    }
}
