use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Form Submit
///
/// ## Features required
///
/// forms
///
/// see example in Form
pub struct FormSubmit {
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Text of submit. Required
    pub value: String,
    /// Type submit style
    #[prop_or(Palette::Standard)]
    pub submit_palette: Palette,
    /// the submit style according with the purpose
    #[prop_or(Style::Regular)]
    pub submit_style: Style,
    /// the size of the submit
    #[prop_or(Size::Medium)]
    pub size: Size,
    /// Whether the form control is disabled
    #[prop_or(false)]
    pub disabled: bool,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// general property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// general property to add custom id
    #[prop_or_default]
    pub id: String,
}

impl Component for FormSubmit {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="submit"
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
                class=format!(
                    "form-submit {} {} {} {}",
                    get_style(self.props.submit_style.clone()),
                    get_pallete(self.props.submit_palette.clone()),
                    get_size(self.props.size.clone()),
                    self.props.class_name,
                ),
                disabled=self.props.disabled
                id=self.props.id
                value=self.props.value
            />
        }
    }
}

#[wasm_bindgen_test]
fn should_create_form_submit() {
    let props = Props {
        value: "submit".to_string(),
        disabled: false,
        key: "".to_string(),
        code_ref: NodeRef::default(),
        id: "result".to_string(),
        class_name: "form-submit-test".to_string(),
        submit_style: Style::Regular,
        submit_palette: Palette::Standard,
        size: Size::Medium,
    };

    let form_submit: App<FormSubmit> = App::new();

    form_submit.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        props,
    );

    let form_submit_element = utils::document().get_element_by_id("result").unwrap();

    assert_eq!(form_submit_element.tag_name(), "INPUT");
}
