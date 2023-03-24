#[cfg(feature = "IoLogoEuro")]
use leptos::*;
#[cfg(feature = "IoLogoEuro")]
///This icon requires the feature `IoLogoEuro` to be enabled.
#[component]
pub fn LogoEuro(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M231.8,272V224H376l8-48H231.8v-8.12c0-38.69,16.47-62.56,87.18-62.56,28.89,0,61.45,2.69,102.5,9.42l10.52-70A508.54,508.54,0,0,0,315.46,32C189.26,32,135,76.4,135,158.46V176l-55,0v48h55v48H80v48h55v33.54C135,435.6,189.23,480,315.43,480a507.76,507.76,0,0,0,116.44-12.78l-10.58-70c-41.05,6.73-73.46,9.42-102.35,9.42-70.7,0-87.14-20.18-87.14-67.94V320H360.27l7.87-48Z"
        /> < title > { title } < / title > < / svg >
    }
}
