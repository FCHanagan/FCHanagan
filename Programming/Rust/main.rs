
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


////////p//////////////////////////////////////////////////////////
// display_table                                                // 
//////////////////////////////////////////////////////////////////
  
 fn display_table(my_array: &[Scor]) {
            
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
      
           for inx in 0..my_array.len() {
        
           
              let tss = my_array[inx].y.to_string() + "   ------>  " +
                       &my_array[inx].x.to_string();

              let transform = c.transform.trans(50.0, 50.0 + 50.0 * (inx as f64));

              text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &tss,                          
                &mut glyphs,
                &c.draw_state,
                transform, g
              ).unwrap();
           
          }   
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

////////////////////////////////////////////////////////////////
//      print_vec
////////////////////////////////////////////////////////////////
 fn print_vec(v: &[Scor]) {
    println!("{:?}", v);
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

     
      println!("   {:?} ",v);

      let mut my_array = ["--","--", "--", "--", "--", "--", "--", "--", "--", "--"];
     
      println!("{:?}", my_array);
  
      let slice =  &mut my_array[0..10];

      let binding0 =  v[0].x.to_string() + "  ----->  " +  &v[0].y;
      let binding1 =  v[1].x.to_string() + "  ----->  " +  &v[1].y;


      
    // print_vec(&v.as_slice());



     println!("Vector length: {}", v.len());

     
      
     let f =  File::create(s).expect("File open failed");

     let mut f = BufWriter::new(f);
     

     
     for ix in 0..v.len() {
      
     //  let my_array[0] = &(v[0].x.to_string() + "  ----->   " +  &v[0].y); 
       f.write_all(v[ix].x.to_string().as_bytes()).expect("Unable to write data");
       f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");
       f.write_all(v[ix].y.to_string().as_bytes()).expect("Unable to write data");
       f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");     
     }
 
      // display_table(&my_array);
      // display_table(&my_array);
        display_table(&v.as_slice()); 

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
