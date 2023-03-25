#[cfg(feature = "SiAffinitydesigner")]
use leptos::*;
#[cfg(feature = "SiAffinitydesigner")]
///This icon requires the feature `SiAffinitydesigner` to be enabled.
#[component]
pub fn Affinitydesigner(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.44 0L0 18.083v5.197a.72.72 0 00.713.72h10.023L5.7 15.277 14.52 0zm5.16 0l-4.86 8.418 3.718 6.439H24V.718A.72.72 0 0023.28 0zm-5.4 9.353l-2.064 3.575a1.289 1.289 0 000 1.288c.23.4.656.64 1.117.64h4.125zm-3.122 6.44L11.816 24h11.471a.72.72 0 00.713-.718v-7.49Z"
        /> < title > { title } < / title > < / svg >
    }
}
