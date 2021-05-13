use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Done{
    pub super_struct: Base
}

impl Done{
    pub fn new(input_title: String)-> Done{
        let base:Base = Base::new(input_title,String::from("Done"));
        return Done{super_struct: base}
    }
}

impl Create for Done{}
impl Delete for Done{}
impl Edit for Done{}
impl Get for Done{}