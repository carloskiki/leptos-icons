#[cfg(feature = "SiAllocine")]
use leptos::*;
#[cfg(feature = "SiAllocine")]
///This icon requires the feature `SiAllocine` to be enabled.
#[component]
pub fn Allocine(
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
        "M16.434.001a.826.826 0 00-.164.008l-3.423.543a2.635 2.635 0 01-2.189 3.01 2.629 2.629 0 01-3.01-2.185l-3.417.538a.818.818 0 00-.677.931l3.24 20.467a.818.818 0 00.931.677l3.423-.543a2.635 2.635 0 012.189-3.01 2.629 2.629 0 013.01 2.185l3.422-.543a.818.818 0 00.677-.93L17.2.685a.816.816 0 00-.767-.685zm-3.22 6.534c.066 0 .128.005.185.017.423.09.975.6 1.315.955.178.187.192.519.048.73l-1.228 1.795a.89.89 0 01-.437.283c-.504.125-1.248-.95-1.771 1.507-.524 2.458.59 1.776 1.003 2.098a.828.828 0 01.283.437l.394 2.14a.613.613 0 01-.341.649c-.456.182-1.167.427-1.589.336-.907-.192-2.342-2.4-1.57-6.044.725-3.415 2.71-4.89 3.708-4.903Z"
        /> < title > { title } < / title > < / svg >
    }
}
