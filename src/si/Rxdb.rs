#[cfg(feature = "SiRxdb")]
use leptos::*;
#[cfg(feature = "SiRxdb")]
///This icon requires the feature `SiRxdb` to be enabled.
#[component]
pub fn Rxdb(
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
        "M11.096 3.452h1.15v-1.15h1.151v1.15h6.411v4.932H4.192V1.15h1.15V2.3h1.151V0h1.15v1.15h1.152v2.302h1.15V1.151h1.151v2.301zM4.192 14.466h15.616V9.534H4.192v4.932zm15.616 1.15H4.192v4.932h6.41v1.15h1.151v-1.15h1.151v2.301h1.15v-2.301h1.152v2.301h1.15V24h1.151v-2.301h1.15v1.15h1.151v-7.233z"
        /> < title > { title } < / title > < / svg >
    }
}
