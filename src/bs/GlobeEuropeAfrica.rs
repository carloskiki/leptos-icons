#[cfg(feature = "BsGlobeEuropeAfrica")]
use leptos::*;
#[cfg(feature = "BsGlobeEuropeAfrica")]
///This icon requires the feature `BsGlobeEuropeAfrica` to be enabled.
#[component]
pub fn GlobeEuropeAfrica(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-globe-europe-africa" viewBox = "0 0 16 16" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0ZM3.668 2.501l-.288.646a.847.847 0 0 0 1.479.815l.245-.368a.809.809 0 0 1 1.034-.275.809.809 0 0 0 .724 0l.261-.13a1 1 0 0 1 .775-.05l.984.34c.078.028.16.044.243.054.784.093.855.377.694.801-.155.41-.616.617-1.035.487l-.01-.003C8.274 4.663 7.748 4.5 6 4.5 4.8 4.5 3.5 5.62 3.5 7c0 1.96.826 2.166 1.696 2.382.46.115.935.233 1.304.618.449.467.393 1.181.339 1.877C6.755 12.96 6.674 14 8.5 14c1.75 0 3-3.5 3-4.5 0-.262.208-.468.444-.7.396-.392.87-.86.556-1.8-.097-.291-.396-.568-.641-.756-.174-.133-.207-.396-.052-.551a.333.333 0 0 1 .42-.042l1.085.724c.11.072.255.058.348-.035.15-.15.415-.083.489.117.16.43.445 1.05.849 1.357L15 8A7 7 0 1 1 3.668 2.501Z"
        /> < title > { title } < / title > < / svg >
    }
}
