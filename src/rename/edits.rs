// EXPERIMENTAL FEATURE IN PROGRESS

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Edits {
    pub ids: Vec<(String, usize)>, // ID, count
    pub folders: Vec<EditFolder>
}

impl Edits {
    /// Gets a revision and removes the others. 
    pub fn get_revision(&mut self, id: String) -> Option<Vec<EditRevision>> {

        None
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditFolder {
    pub path: String,
    pub files: Vec<EditFile>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditFile {
    pub name_original: String,
    pub name_current: String,
    pub name_modifications: Vec<EditRevision>,
    pub path_original: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EditRevision {
    pub edit_id: String,
    pub edited_file_name: String,
    pub edited_path_full: String,
    pub edit_type: EditType
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum EditType {
    Redo,
    #[default]
    Undo
}