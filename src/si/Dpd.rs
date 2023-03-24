#[cfg(feature = "SiDpd")]
use leptos::*;
#[cfg(feature = "SiDpd")]
///This icon requires the feature `SiDpd` to be enabled.
#[component]
pub fn Dpd(
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
        "M16.01 10.71a.364.364 0 01-.343-.006l-.558-.331a.43.43 0 01-.182-.312l-.014-.65a.363.363 0 01.165-.3l6.7-3.902L12.377.085A.799.799 0 0012 0a.798.798 0 00-.377.085l-9.4 5.124 10.53 6.13c.098.054.172.181.172.295v8.944c0 .112-.08.241-.178.294l-.567.315c-.171.062-.256.043-.361 0l-.569-.315a.362.362 0 01-.175-.294v-7.973a.223.223 0 00-.095-.156L1.702 7.048v10.579c0 .236.167.528.371.648l9.556 5.636c.102.06.237.09.371.089a.745.745 0 00.371-.09l9.557-5.635a.835.835 0 00.37-.648V7.047Z"
        /> < title > { title } < / title > < / svg >
    }
}
