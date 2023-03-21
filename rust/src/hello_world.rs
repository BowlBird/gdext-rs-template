use godot::prelude::*;


//acts as base class for each file
//change from 'Node' to whatever component you are using.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct HelloWorld {

    //add extra fields here

    #[base]
    base: Base<Node>,
}

//add functions to be used here
#[godot_api]
impl HelloWorld {
    #[func]
    pub fn example(&mut self) {

    }
}

/** for Godot related functions
 * 
 *  these are virtual functions, and there are more than these two. (such as input)
 *  I only put these as a nice transition from something like Unity as these two functions
 *  are similar to start and update. 
 */
#[godot_api]
impl NodeVirtual for HelloWorld {

    //initalize
    fn init(base: Base<Node>) -> Self {

        godot_print!("Hello, World!");
        
        HelloWorld {
            base,
        }
    }

    //after initalized
    fn ready(&mut self) {
        
    }

    //per frame method call
    fn process(&mut self, _delta:f64) {

    }
}