#[cfg(feature = "RiMapLineTaxiWifi")]
use leptos::*;
#[cfg(feature = "RiMapLineTaxiWifi")]
///This icon requires the feature `RiMapLineTaxiWifi` to be enabled.
#[component]
pub fn TaxiWifi(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 3v4H6.236l-2.001 4H22v10a1 1 0 0 1-1 1h-1a1 1 0 0 1-1-1v-1H5v1a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V11l2.447-4.894A2 2 0 0 1 6.237 5H9V3h3zm8 10H4v5h16v-5zM6.5 14a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zm11 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zm1-13a4.5 4.5 0 1 1 0 9 4.5 4.5 0 0 1 0-9zm0 5.167c-.491 0-.94.177-1.289.47l-.125.115L18.5 8.167l1.413-1.416a1.994 1.994 0 0 0-1.413-.584zm0-2.667a4.65 4.65 0 0 0-3.128 1.203l-.173.165.944.942a3.323 3.323 0 0 1 2.357-.977 3.32 3.32 0 0 1 2.201.83l.156.147.943-.943A4.652 4.652 0 0 0 18.5 3.5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
