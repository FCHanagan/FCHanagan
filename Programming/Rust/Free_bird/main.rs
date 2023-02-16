extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "High Scores Table",
            [400, 550]
        )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();
 
    
    let my_array = ["apple", "banana", "cherry", "prune", "pineapple",
    "watermellon","Grape","Lemon", "Lime","Rutabega"];

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
       
   // println!("{:?}", assets);
    let mut glyphs = window.load_font(assets.join("FiraSans-Bold.ttf")).unwrap();
    

    window.set_lazy(true);
    
   //  let mut icounter = 5;

    while let Some(e) = window.next() {

    
        window.draw_2d(&e, |c, g, device| {
      

         for inx in 0..10 {


           let transform = c.transform.trans(50.0, 50.0 + 50.0 * (inx as f64));

           // clear([0.0, 0.0, 0.0, 1.0], g);

            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                my_array[inx],              
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
           
          }   

             // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    //  let icounter = icounter + 1;
    }
}