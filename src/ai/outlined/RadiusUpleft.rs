#[cfg(feature = "AiOutlinedRadiusUpleft")]
use leptos::*;
#[cfg(feature = "AiOutlinedRadiusUpleft")]
///This icon requires the feature `AiOutlinedRadiusUpleft` to be enabled.
#[component]
pub fn RadiusUpleft(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M656 200h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8zm58 624h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zM192 650h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 174h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm696-696h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 174h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 174h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 174h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm0 174h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm-348 0h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm-174 0h-56c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8zm174-696H358c-127 0-230 103-230 230v182c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8V358c0-87.3 70.7-158 158-158h182c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8z"
        /> < title > { title } < / title > < / svg >
    }
}
