#[cfg(feature = "CgArrowLongUpE")]
use leptos::*;
#[cfg(feature = "CgArrowLongUpE")]
///This icon requires the feature `CgArrowLongUpE` to be enabled.
#[component]
pub fn ArrowLongUpE(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.0321 1.01318L16.2425 5.28781L14.8177 6.69131L13.014 4.86019L12.9436 16.9774L14.9425 16.9869L14.914 22.9868L8.91412 22.9583L8.9426 16.9584L10.9436 16.9679L11.0142 4.82318L9.16107 6.64852L7.75757 5.22367L12.0321 1.01318ZM10.9236 20.9678L12.9236 20.9773L12.9331 18.9773L10.9331 18.9678L10.9236 20.9678Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
