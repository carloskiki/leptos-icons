#[cfg(feature = "IoIceCream")]
use leptos::*;
#[cfg(feature = "IoIceCream")]
///This icon requires the feature `IoIceCream` to be enabled.
#[component]
pub fn IceCream(
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
        "M183,352c-21.84-.52-39-18.9-39-40.74V277.19a8,8,0,0,0-6-7.74C104.25,260.6,80,229.74,80,192a80.14,80.14,0,0,1,66.27-78.82,8,8,0,0,0,6.62-6.83,104,104,0,0,1,206.22,0,8,8,0,0,0,6.62,6.83A80,80,0,0,1,352,272a74.33,74.33,0,0,1-47.45-17.41,7.93,7.93,0,0,0-9.92-.14A62.89,62.89,0,0,1,256,268a80.47,80.47,0,0,1-21.8-3.18,8,8,0,0,0-10.2,7.69V312A40,40,0,0,1,183,352Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M263.39,299.7a8,8,0,0,0-7.39,7.91V312a72.11,72.11,0,0,1-50.69,68.76,8,8,0,0,0-4.91,10.78l40.91,94.8A16,16,0,0,0,256,496h0a16,16,0,0,0,14.69-9.7l73.78-172.15a8,8,0,0,0-6.2-11.07,106.31,106.31,0,0,1-35.9-11.59,8,8,0,0,0-7.13-.2A95,95,0,0,1,263.39,299.7Z"
        /> < title > { title } < / title > < / svg >
    }
}
