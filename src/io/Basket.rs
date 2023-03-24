#[cfg(feature = "IoBasket")]
use leptos::*;
#[cfg(feature = "IoBasket")]
///This icon requires the feature `IoBasket` to be enabled.
#[component]
pub fn Basket(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M424.11,192H360L268.8,70.4a16,16,0,0,0-25.6,0L152,192H87.89a32.57,32.57,0,0,0-32.62,32.44,30.3,30.3,0,0,0,1.31,9l46.27,163.14a50.72,50.72,0,0,0,48.84,36.91H360.31a51.21,51.21,0,0,0,49-36.86l46.33-163.36a15.62,15.62,0,0,0,.46-2.36l.53-4.93a13.3,13.3,0,0,0,.09-1.55A32.57,32.57,0,0,0,424.11,192ZM256,106.67,320,192H192Zm0,245a37.7,37.7,0,1,1,37.88-37.7A37.87,37.87,0,0,1,256,351.63Z"
        /> < title > { title } < / title > < / svg >
    }
}
