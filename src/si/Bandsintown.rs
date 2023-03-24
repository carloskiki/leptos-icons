#[cfg(feature = "SiBandsintown")]
use leptos::*;
#[cfg(feature = "SiBandsintown")]
///This icon requires the feature `SiBandsintown` to be enabled.
#[component]
pub fn Bandsintown(
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
        "M18.783 0H24v12h-5.217V0zm-6.261 5h5.217v7h-5.217V5zM6.26 5h5.217v7H6.261V5zM24 24H0V0h5.217v19h13.566v-1H6.26v-5H24v11Z"
        /> < title > { title } < / title > < / svg >
    }
}
