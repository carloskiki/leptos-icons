#[cfg(feature = "IoPieChart")]
use leptos::*;
#[cfg(feature = "IoPieChart")]
///This icon requires the feature `IoPieChart` to be enabled.
#[component]
pub fn PieChart(
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
        "M66.1,357a16,16,0,0,1-14.61-9.46A224,224,0,0,1,256,32a16,16,0,0,1,16,16V256a16,16,0,0,1-9.47,14.61L72.63,355.56A15.93,15.93,0,0,1,66.1,357Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M313.59,68.18A8,8,0,0,0,304,76V256a48.07,48.07,0,0,1-28.4,43.82L103.13,377a8,8,0,0,0-3.35,11.81,208.42,208.42,0,0,0,48.46,50.41A206.32,206.32,0,0,0,272,480c114.69,0,208-93.31,208-208C480,171.55,408.42,87.5,313.59,68.18Z"
        /> < title > { title } < / title > < / svg >
    }
}
