pub struct Base{
    pub title: String,
    pub status: String
}

impl Base{
    pub fn new(input_title: String, input_status: String) -> Base{
        return Base{title: input_title, status: input_status}
    }
}