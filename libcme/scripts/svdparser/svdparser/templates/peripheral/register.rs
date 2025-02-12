/// /*% register_name %*/
///
/// /*% register_description %*/
#[bitfield(/*% register_size %*/)]
#[derive(PartialEq, Eq)]
pub struct /*% register_name_upper %*/ {
    /*! register_fields --->
    /// %field_description%
    #[bits(%field_width%)]
    pub %field_name%: %field_type%,
    !*/
}