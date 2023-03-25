#[cfg(feature = "SiAmazoneks")]
use leptos::*;
#[cfg(feature = "SiAmazoneks")]
///This icon requires the feature `SiAmazoneks` to be enabled.
#[component]
pub fn Amazoneks(
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
        "m14.67 8.505-2.875 3.335 3.128 3.648h-1.168l-2.808-3.277v3.368h-.842V8.421h.842v2.947l2.603-2.863Zm7.225 6.751-3.369-2.021V8.421a.42.42 0 0 0-.209-.364l-4.843-2.825V1.159l8.42 4.976Zm.635-9.724L13.267.06a.422.422 0 0 0-.635.362v5.053c0 .15.08.288.208.363l4.844 2.826v4.81a.42.42 0 0 0 .205.362l4.21 2.526a.42.42 0 0 0 .638-.361V5.895a.42.42 0 0 0-.207-.363ZM11.977 23.1l-9.872-5.248V6.135l8.421-4.976v4.084L6.09 8.066a.422.422 0 0 0-.195.355v7.158a.42.42 0 0 0 .226.373l5.665 2.948a.42.42 0 0 0 .387 0l5.496-2.84 3.382 2.03Zm10.135-5.356-4.21-2.526a.42.42 0 0 0-.411-.013l-5.51 2.847-5.244-2.729v-6.67l4.436-2.824a.422.422 0 0 0 .195-.355V.42a.421.421 0 0 0-.635-.362L1.47 5.532a.421.421 0 0 0-.207.363v12.21c0 .156.086.299.223.372l10.297 5.474a.421.421 0 0 0 .401-.004l9.915-5.473a.422.422 0 0 0 .013-.73Z"
        /> < title > { title } < / title > < / svg >
    }
}
