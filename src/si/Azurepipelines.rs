#[cfg(feature = "SiAzurepipelines")]
use leptos::*;
#[cfg(feature = "SiAzurepipelines")]
///This icon requires the feature `SiAzurepipelines` to be enabled.
#[component]
pub fn Azurepipelines(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M1.7997 22.1998h4.2001V24H.0004v-6.0002h1.7993v4.2zM12.6229 9.8975l-7.6353 7.6353 1.4792 1.4792 7.6345-7.6362-1.4784-1.4783zM4.365 15.1793l-.391-.3908c-.5042-.5042-.5828-1.3004-.1869-1.8934l4.6036-6.8958H.9979c-.5513.0014-.9975.4487-.9975 1v5.5002l3.5021 3.6036.8625-.9238zM23.9996.9999v8.9306a2 2 0 0 1-.8893 1.6632l-12.2834 8.2005a.9999.9999 0 0 1-1.2622-.1246L8.04 18.1457c.3313-.3313 6.4656-6.4672 6.7684-6.7698l-2.1855-2.1855L5.8533 15.96l-1.5258-1.525a1.0001 1.0001 0 0 1-.1246-1.2623L12.4034.8893A2 2 0 0 1 14.0666 0h8.9331c.5522 0 .9999.4477.9999.9999zm-2.9997 5.0003c0-1.6567-1.343-2.9997-2.9997-2.9997s-2.9997 1.343-2.9997 2.9997 1.343 2.9997 2.9997 2.9997 2.9997-1.343 2.9997-2.9997zM10.272 20.4625c-.4008 0-.7776-.156-1.0609-.4392l-.368-.3679-.8435.8417 3.4996 3.4996h5.5003c.5522 0 .9999-.4477.9999-.9999v-7.3899L11.1045 20.21a1.4944 1.4944 0 0 1-.8325.2525z"
        /> < title > { title } < / title > < / svg >
    }
}
