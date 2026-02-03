use strum::Display;

#[derive(Clone, Default, Debug, Display, PartialEq)]
pub enum Action {
    //
    #[strum(to_string = "Create")]
    Create,

    #[strum(to_string = "Delete")]
    Delete,

    #[strum(to_string = "Edit")]
    Edit,

    #[default]
    #[strum(to_string = "View")]
    View,
}

impl Action {
    //
    pub fn is_view(&self) -> bool {
        self == &Action::View
    }

    pub fn is_edit(&self) -> bool {
        self == &Action::Edit
    }
}
