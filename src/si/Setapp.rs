#[cfg(feature = "SiSetapp")]
use leptos::*;
#[cfg(feature = "SiSetapp")]
///This icon requires the feature `SiSetapp` to be enabled.
#[component]
pub fn Setapp(
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
        "M13.0949 8.1332a.619.619 0 0 1 0-.874l2.7712-2.7733a.619.619 0 0 1 .877 0l2.7703 2.7722a.619.619 0 0 1 0 .8751l-2.7703 2.7722a.619.619 0 0 1-.877 0zm-1.5331-1.5331L8.7906 3.8299a.618.618 0 0 1 0-.877L11.5618.1815a.619.619 0 0 1 .876 0l2.7732 2.7712a.619.619 0 0 1 0 .877L12.4378 6.6a.619.619 0 0 1-.876 0zm0 2.1902a.619.619 0 0 1 .876 0l2.7732 2.7712a.619.619 0 0 1 0 .877l-2.7732 2.7712a.619.619 0 0 1-.876 0l-2.7712-2.7692a.618.618 0 0 1 0-.877zm-4.3044 2.1151L4.4862 8.1332a.619.619 0 0 1 0-.876l2.7712-2.7713a.619.619 0 0 1 .8761 0l2.7722 2.7712a.621.621 0 0 1 0 .8761l-2.7732 2.7722a.619.619 0 0 1-.876 0zm9.4847 2.1902 2.7723 2.7712a.618.618 0 0 1 0 .875l-2.7703 2.7723a.619.619 0 0 1-.876 0l-2.7732-2.7722a.621.621 0 0 1 0-.8751l2.7732-2.7722a.619.619 0 0 1 .875 0zm-4.3043 4.3033 2.7722 2.7722a.618.618 0 0 1 0 .876l-2.7722 2.7713a.619.619 0 0 1-.876 0l-2.7712-2.7712a.619.619 0 0 1 0-.877l2.7712-2.7713a.619.619 0 0 1 .876 0zm-1.532-1.5321a.619.619 0 0 1 0 .875l-2.7723 2.7733a.621.621 0 0 1-.876 0l-2.7723-2.7722a.619.619 0 0 1 0-.8751l2.7722-2.7722a.619.619 0 0 1 .8761 0z"
        /> < title > { title } < / title > < / svg >
    }
}
