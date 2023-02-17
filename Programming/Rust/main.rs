
use std::fs::File;
extern crate piston_window;
extern crate find_folder;

use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::cmp::Ordering;

use piston_window::*;

#[derive(Default, Debug)]
struct Scor {
    x: i32,
    y: String,
}


//////////////////////////////////////////////////////////////////
// display_table                                                // 
//////////////////////////////////////////////////////////////////
fn  display_table(my_array: &[&str]) {
    
    let mut window: PistonWindow = WindowSettings::new(
            "High Scores Table",
            [400, 550]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();
 
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
       
    let mut glyphs = window.load_font(assets.join("FiraSans-Bold.ttf")).unwrap();
    
    window.set_lazy(true);
    
    while let Some(e) = window.next() {
    
        window.draw_2d(&e, |c, g, device| {
      
         for inx in 0..10 {

           let transform = c.transform.trans(50.0, 50.0 + 50.0 * (inx as f64));

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
    }
}


//////////////////////////////////////////////////////////////////
// create_data_files                                            // 
//////////////////////////////////////////////////////////////////

 fn create_data_file(s: &String) {  
   
    let _file = std::fs::File::create(s).expect("create failed");       
}
   
//////////////////////////////////////////////////////////////////
// read_data_buffered                                           // 
//////////////////////////////////////////////////////////////////

fn read_data_buffered(s: &String) {  

    let mut v: Vec<Scor> = Vec::new();   

    let f = File::open(s).expect("Unable to open file");
    
    let f = BufReader::new(f);

    let mut i:i32 =0;
   
    let mut lxn = 0;

    for line in f.lines() {
       
           
        let line = line.expect("Unable to read line");
        i+=1;
        
          if i%2 == 1 {
      
            let m_i: i32 = line.parse().unwrap();
           
            lxn = m_i;                
        } else {
                
          let my_init = line;
         
             v.push(Scor {
                  y: my_init.into(),
                  x: lxn,
          });    
       }   
    }

    v.sort_by(|a, b| {
        if a.x < b.x {
            Ordering::Greater
        } else if a.x == b.x {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    if v.len() > 10 { 
        let i_remove = v.len() - 10;
     //   println!("remove =  { }",i_remove);

        for _n in 0..i_remove{
            v.pop();
        }
     }

     
    let mut my_array = ["--","--", "--", "--", "--", "--", "--", "--", "--", "--"];
     
  //   println!("{:?}", my_array);

       

/*
     let binding = 0; 

     for ix in 0..v.len() {

        let binding = v[ix].x.to_string() + "  ----->  " +  &v[ix].y;

        my_array[ix] = &binding;
     }
*/
   /*  
     let icount = 0;
     
   //  let binding = " ";
     
     let binding = v[icount].x.to_string() + "  ----->   " +  &v[icount].y;
     
     my_array[icount] = &binding;

     
     for icn in 0..v.len() {
        
         let binding = v[icn].x.to_string() + "  ----->   " +  &v[icn].y;

         my_array[icn] = &binding;   
     }
   
     */

     println!("Vector length: {}", v.len());

     
      
     let f =  File::create(s).expect("File open failed");

     let mut f = BufWriter::new(f);
     

     
     for ix in 0..v.len() {
      
      // my_array[ix] = v[ix].x.to_string() + "  ----->   " +  &v[ix].y; 
       f.write_all(v[ix].x.to_string().as_bytes()).expect("Unable to write data");
       f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");
       f.write_all(v[ix].y.to_string().as_bytes()).expect("Unable to write data");
       f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");     
     }
 
       display_table(&my_array);

}           
 
//////////////////////////////////////////////////////////////////
// append_data_buffered                                         // 
//////////////////////////////////////////////////////////////////

fn append_data_buffered(s: &String,i: &i32, u_init: &String) {

  
    let f = OpenOptions::new()
        .append(true)
        .open(s)
        .expect("Unable to open file");

    let mut f = BufWriter::new(f);
 
    f.write_all(i.to_string().as_bytes()).expect("Unable to write data");   
    f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");
    f.write_all(u_init.to_string().as_bytes()).expect("Unable to write data");
    f.write_all("\n".to_string().as_bytes()).expect("Unable to write data"); 
}

/////////////////////////////////////////////////////////////////////////////
//          main routine                                                   //
/////////////////////////////////////////////////////////////////////////////
fn main() {

 let s = "data.txt".to_string();
  
 let  rs = Path::new("data.txt").exists();
    
    if rs == true{
        println!("File exists");
    }
    else{
        println!("File does not exist");
        create_data_file(&s); 
    }  

let  _ui = "FH".to_string();
let   _ik = "jj".to_string();
 let dd = "DD".to_string(); 

let md = 13;
 append_data_buffered(&s,&md,&dd);

read_data_buffered(&s);

}
