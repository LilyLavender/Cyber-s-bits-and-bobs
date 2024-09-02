#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

/* 
mod status;
mod custom;
mod counter;
mod normals;
mod dk;
*/
mod throwmove;
//mod zss;


#[skyline::main(name = "smashline_test")]
pub fn main() {
    /* 
    custom::install();
    smashline::clone_weapon(
        "mario",
        "fireball",
        "edge",
        "bunshin",
        false
      );
    status::install();
    normals::install();
    counter::install();
    */
    //zss::install();
    
    //dk::install();  
    throwmove::install();
}