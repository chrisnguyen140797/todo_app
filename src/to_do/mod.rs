pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes{
    Done(Done),
    Pending(Pending)
}

pub fn to_do_factory(item_type: &String, item_title: String) -> 
Result<ItemTypes, &'static str>{
if item_type == "Done"{
    let done = Done::new(item_title);
    Ok(ItemTypes::Done(done))
}
else if item_type == "Pending"{
    let pending = Pending::new(item_title);
    Ok(ItemTypes::Pending(pending))
}
else{
    Err("Not found")
}
}