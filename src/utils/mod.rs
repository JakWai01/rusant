/*
 * Represent the behaviour of the UI in several states.
 */
pub enum UiState {
    /*
     * The start state: No contact is selected/displayed.
     */
    NORMAL,

    /*
     * A single contact is selected and displayed 
     */
    SHOWING,

    /*
     * Zero or more contacts are selected (but this can be changed).
     * One contact might be displayed.
     */
    SELECTING,

    /*
     * A new contact is being created 
     */
    CREATING,
}

impl Default for UiState {
    fn default() -> UiState {
        UiState::NORMAL
    }
}