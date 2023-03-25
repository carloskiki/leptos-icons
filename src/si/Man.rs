#[cfg(feature = "SiMan")]
use leptos::*;
#[cfg(feature = "SiMan")]
///This icon requires the feature `SiMan` to be enabled.
#[component]
pub fn Man(
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
        "M10.979 14.943h2.05L15.46 18.7h-2.054l-.263-.409h-2.278l-.264.41H8.548zm1.025 1.568l-.458.711h.916l-.458-.712zM0 17.372C0 10.704 5.372 5.3 12 5.3s12 5.405 12 12.073c0 .449-.024.892-.072 1.328H22.58c.054-.435.082-.878.082-1.328 0-5.924-4.774-10.726-10.662-10.726-5.889 0-10.661 4.802-10.661 10.726 0 .45.027.893.08 1.328H.073A12.254 12.274 0 0 1 0 17.372zm2.237-2.43h1.83l1.22 1.228 1.22-1.227h1.831V18.7H6.363v-1.38l-1.075 1.082-1.076-1.082v1.38H2.237v-3.757zm13.42 0h1.927l2.17 1.62v-1.62h1.975V18.7h-1.942l-2.156-1.605V18.7h-1.975Z"
        /> < title > { title } < / title > < / svg >
    }
}
