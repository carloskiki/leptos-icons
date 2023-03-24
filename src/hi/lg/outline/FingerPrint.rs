#[cfg(feature = "HiLgOutlineFingerPrint")]
use leptos::*;
#[cfg(feature = "HiLgOutlineFingerPrint")]
///This icon requires the feature `HiLgOutlineFingerPrint` to be enabled.
#[component]
pub fn FingerPrint(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.86391 4.24259C9.04956 3.45731 10.4714 3 12 3C16.1421 3 19.5 6.35786 19.5 10.5C19.5 13.4194 18.9443 16.2089 17.9324 18.7685M5.7426 6.36391C4.95732 7.54956 4.5 8.97138 4.5 10.5C4.5 11.9677 4.07875 13.3369 3.3501 14.4931M5.33889 18.052C7.14811 16.0555 8.25 13.4065 8.25 10.5C8.25 8.42893 9.92893 6.75 12 6.75C14.0711 6.75 15.75 8.42893 15.75 10.5C15.75 11.0269 15.7286 11.5487 15.686 12.0646M12.0003 10.5C12.0003 14.2226 10.6443 17.6285 8.39916 20.2506M15.033 15.6543C14.4852 17.5743 13.6391 19.3685 12.5479 20.9836"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
