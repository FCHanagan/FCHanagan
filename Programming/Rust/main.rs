/////////////////////////////////////////////////
//////////////////////////////////////////////////
//////////////////////////////////////////////////
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

////////////////////////////////////////////////////
////////////////////////////////////////////////////
////////////////////////////////////////////////////

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
// use std::io::prelude::*;
use std::cmp::Ordering;


//fn main() {

#[derive(Default, Debug)]
struct Scor {
    x: i32,
    y: String,
}





//////////////////////////////////////////////////////////////
//let mut v: Vec<Scor> = Vec::new();
 // let mut v: Vec<Scor> = Vec::new();
////////////////////////////////////////////////////////////////////////////
 fn create_data_file(s: &String) {
   // let int_var:i32 = 108;
   // let str_var = int_var.to_string();
///////////////////////////////////////////////////////////////////////////////////////////////
    let file = std::fs::File::create(s).expect("create failed");
   }
///////////////////////////////////////////////////////////////////////////

/*
fn read_data_file(s: &String) {
    let mut data = String::new();
   // let mut f = File::open("data.txt").expect("Unable to open file");
    let mut f = File::open(s).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    println!("read: {}", data);
    }
*/
 ///////////////////////////////////////////////////////////////////////////

fn read_data_buffered(s: &String) {
 //////////////////////////////////////////////
    let mut v: Vec<Scor> = Vec::new();
//////////////////////////////////////////////////
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

        for n in 0..i_remove{
            v.pop();
        }
     }



     for n in 0..v.len()  {
     println!("{ }  { }",v[n].x,v[n].y);
     }

     println!("Vector length: {}", v.len());

  //   println!("{ }  { }",v[v.len()-1].x,v[v.len()-1].y);

     let f =  File::create(s).expect("File open failed");

     let mut f = BufWriter::new(f);


     ////////////////////////////////////////////////////////////////////////////////////////////
     for ix in 0..v.len(){
       f.write_all(v[ix].x.to_string().as_bytes()).expect("Unable to write data");
       f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");
       f.write_all(v[ix].y.to_string().as_bytes()).expect("Unable to write data");
       f.write_all("\n".to_string().as_bytes()).expect("Unable to write data");
     }
}
 ////////////////////////////////////////////////////////////////////////////////

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

 ///////////////////////////////////////////////////////////////////////////////
fn main() {

////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let width = COLS * SQUARE_WIDTH;
    let height= ROWS * SQUARE_WIDTH;



 let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("High Scores:", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

///////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////



 let s = "data.txt".to_string();

// let ind:i32 = 0;

//read in the score

let mut input_text = String::new();
 /*
io::stdin()
.read_line(&mut input_text)
.expect("failed to read from stdin");

let trimmed = input_text.trim();
match trimmed.parse::<i32>() {
Ok(ind) => println!("your integer input: {}", ind),
Err(..) => println!("this was not an integer: {}", trimmed),
 }
*/

  let mut rs:bool=true;

    rs = Path::new("data.txt").exists();

    if rs == true{
        println!("File exists");
    }
    else{
        println!("File does not exist");
        create_data_file(&s);
    }



let  ui = "FH".to_string();
let  ik = "jj".to_string();
 let dd = "DD".to_string();



let md = 4;
 append_data_buffered(&s,&md,&ik);


//////////////////////////////////////
read_data_buffered(&s);

}
