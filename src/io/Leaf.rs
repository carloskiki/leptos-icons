#[cfg(feature = "IoLeaf")]
use leptos::*;
#[cfg(feature = "IoLeaf")]
///This icon requires the feature `IoLeaf` to be enabled.
#[component]
pub fn Leaf(
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
        "M161.35,242a16,16,0,0,1,22.62-.68c73.63,69.36,147.51,111.56,234.45,133.07,11.73-32,12.77-67.22,2.64-101.58-13.44-45.59-44.74-85.31-90.49-114.86-40.84-26.38-81.66-33.25-121.15-39.89-49.82-8.38-96.88-16.3-141.79-63.85-5-5.26-11.81-7.37-18.32-5.66-7.44,2-12.43,7.88-14.82,17.6-5.6,22.75-2,86.51,13.75,153.82,25.29,108.14,65.65,162.86,95.06,189.73,38,34.69,87.62,53.9,136.93,53.9A186,186,0,0,0,308,461.56c41.71-6.32,76.43-27.27,96-57.75-89.49-23.28-165.94-67.55-242-139.16A16,16,0,0,1,161.35,242Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M467.43,384.19c-16.83-2.59-33.13-5.84-49-9.77a157.71,157.71,0,0,1-12.13,25.68c-.73,1.25-1.5,2.49-2.29,3.71a584.21,584.21,0,0,0,58.56,12,16,16,0,1,0,4.87-31.62Z"
        /> < title > { title } < / title > < / svg >
    }
}
