

//This program should just be running on the CNC checking each file preferably asynchronously should windows be capable of doing such
use walkdir::WalkDir;
use std::env::Vars;
use std::error::Error;
use std::io;
use std::io::Read;
use std::io::Write;
use tokio::fs;

//libmpr import
pub struct Mpr{
    pub preamble: Vec<String>,
    pub vars: Vec<String>,
    pub board: Vec<String>,
    pub points: Vec<String>,
    pub contours: Vec<String>,
    pub comments: Vec<String>,
}

pub enum Data {
preamble,
vars,
board,
points,
contours,
comments,
}

impl Mpr {

pub fn new() -> Mpr
{
    Mpr { 
        preamble: Vec::<String>::new(),
        vars:     Vec::<String>::new(),
        board:    Vec::<String>::new(),
        points:   Vec::<String>::new(),
        contours: Vec::<String>::new(),
        comments: Vec::<String>::new(),
    }
}
pub fn dump(&self){
    for v in &self.vars {
        println!("{}",v);
    }
}
}
/*
pub fn load(&mut self, filename: String)
{
    let path = Path::new(filename.as_str());
    let mut fd = File::open(path).unwrap();
    let mut data = String::new();
    let err = fd.read_to_string(&mut data);
    match err {
        Ok(_) => println!("Read {}",filename),
        Err(err) => println!("Unable to read data! {}",err)
    }
    let mut collect_preamble = false;
    let mut collect_vars = false;
    let mut collect_board = false;
    let mut collect_points = false;
    let mut collect_contours = false;
    let mut collect_comments = false;
    for line in data.lines() { 
        if line == "[H" { 
            collect_preamble = true;
        }
        if line == "[001" { 
            collect_vars = true;
        }
        if line == "]1" { 
            collect_points = true;
        }
        if line == r"<100 \WerkStck\" {
            collect_points = false;
            collect_board = true;
        }
        if line == r"<105 \Konturfraesen\" { 
            collect_contours = true;
        }
        if line.is_empty() { 
            collect_preamble = false;
            collect_vars     = false;
            collect_board    = false;
        }
        if line == r"<101 \Kommentar\" {
            collect_contours = false;
            collect_comments = true;
        }

        if collect_preamble {
            self.preamble.push(line.to_string());
        }
        if collect_vars {
            self.vars.push(line.to_string());
        }
        if collect_points {
            self.points.push(line.to_string());
        }
        if collect_board {
            self.board.push(line.to_string());
        }
        if collect_contours {
            self.contours.push(line.to_string());
        }
        if collect_comments {
            self.comments.push(line.to_string());
        }
    }
}
pub fn replace(&mut self, search_str: String, replace_str: String, datatype: Data)
{
    match datatype{
        Data::preamble => {},
        Data::points => {},
        Data::contours => {},
        Data::board => {},
        Data::comments => {},
        Data::vars => {
            let mut i: usize = 0;
            loop{
                if i == self.vars.len() { break; }
                if self.vars[i] == search_str { 
                    self.vars[i] = replace_str;
                    break;
                }
                i = i + 1;
            }
        }

    }
}  
pub fn save(&mut self, filename: String)
{
    let path = Path::new(filename.as_str());
    let mut fd = File::create(path).unwrap();
    for item in &self.preamble {
        fd.write(item.as_bytes());
        fd.write("\r\n".as_bytes());
    }
    fd.write("\r\n".as_bytes());
    for item in &self.vars {
        fd.write(item.as_bytes());
        fd.write("\r\n".as_bytes());
    }
    fd.write("\r\n".as_bytes());
    for item in &self.points {
        fd.write(item.as_bytes());
        fd.write("\r\n".as_bytes());
    }
    for item in &self.board {
        fd.write(item.as_bytes());
        fd.write("\r\n".as_bytes());
    }
    fd.write("\r\n".as_bytes());
    for item in &self.contours {
        fd.write(item.as_bytes());
        fd.write("\r\n".as_bytes());
    }
    for item in &self.comments {
        fd.write(item.as_bytes());
        fd.write("\r\n".as_bytes());
    }
    fd.write("\r\n".as_bytes());
}
}
impl Default for Mpr {
fn default() -> Self { Self::new() }
}
*/
//end libmpr import
fn get_val(var: &str) -> String{
    let s = var.to_string();
    let mut r = String::new();
    let mut cap = false;
    for c in s.chars(){
        if c == '\"'{
            cap = true;
            continue;
        }
        if cap == true {
            if c == '\"'{
                cap = false;
                break;
            }
            r.push(c);
        }
    }
    r
}

async fn get_var(m: &Mpr, d: Data, var: &str) -> core::result::Result<String,Box<dyn std::error::Error + 'static>> {
    let mut rvar = var.to_string();
    rvar.push('=');
    let mut v = String::new();
    match d {
        Data::preamble => {
            for line in &m.preamble {
                if line.contains(&rvar) {
                   v = line.clone();
                }
            }
        },
        Data::vars => {
            for line in &m.vars {
                if line.contains(&rvar) {
                    v = line.clone();
                }
            }
        },
        Data::board => {
            for line in &m.board {
                if line.contains(&rvar) {
                    v = line.clone();
                }
            }
        },
        Data::points => {
            for line in &m.points {
                if line.contains(&rvar) {
                    v = line.clone();
                }
            }
        },
        Data::contours => {
            for line in &m.contours {
                if line.contains(&rvar) {
                    v = line.clone();
                }
            }
        },
        Data::comments => {
            for line in &m.comments {
                if line.contains(&rvar) {
                    v = line.clone();
                }
            }
        }
    }
    Ok(v)
}

//check_val take a reference to an MPR struct a Data::<type> parameter, the var name in str format, the val name in str format
//logic: if MPR::<Data::type> contains var and contains val return true else return false 
async fn check_val(m: &Mpr, d: Data, var: &str, val: &str) -> core::result::Result<bool,Box<dyn std::error::Error + 'static>> {
    let mut ret = false;
    match d {
        Data::preamble => {
            for line in &m.preamble {
                if line.contains(var) {
                    if line.contains(val){
                        ret = true;
                    }
                }
            }
        },
        Data::vars => {
            for line in &m.vars {
                if line.contains(var) {
                    if line.contains(val){
                       ret = true;
                    }
                }
            }
        },
        Data::board => {
            for line in &m.board {
                if line.contains(var) {
                    if line.contains(val){
                        ret = true;
                    }
                }
            }
        },
        Data::points => {
            for line in &m.points {
                if line.contains(var) {
                    if line.contains(val){
                        ret = true;
                    }
                }
            }
        },
        Data::contours => {
            for line in &m.contours {
                if line.contains(var) {
                    if line.contains(val){
                        ret = true;
                    }
                }
            }
        },
        Data::comments => {
            for line in &m.comments {
                if line.contains(var) {
                    if line.contains(val){
                        ret = true;
                    }
                }
            }
        }
    }
    Ok(ret)
}

//this parse function is to take a file and a mutable reference to an MPR struct
//fill in the struct and exit cleanly
//base logic is "For line in file if line == start of Data::<type> fill in MPR::<type>"

async fn parse(m: &mut Mpr ,f: String) -> core::result::Result<(),Box<dyn std::error::Error + 'static>>{
    let contents = fs::read(f).await?;
    //let cdata = contents.into::<Vec<String>>();
    let cdata = String::from_utf8(contents).unwrap();
    //println!("{:?}",cdata);
    let mut collect_preamble = false;
    let mut collect_vars = false;
    let mut collect_board = false;
    let mut collect_points = false;
    let mut collect_contours = false;
    let mut collect_comments = false;
    for line in cdata.lines() { 
        if line == "[H" { 
            collect_preamble = true;
        }
        if line == "[001" { 
            collect_vars = true;
        }
        if line == "]1" { 
            collect_points = true;
        }
        if line == r"<100 \WerkStck\" {
            collect_points = false;
            collect_board = true;
        }
        if line == r"<105 \Konturfraesen\" { 
            collect_contours = true;
        }
        if line.is_empty() { 
            collect_preamble = false;
            collect_vars     = false;
            collect_board    = false;
        }
        if line == r"<101 \Kommentar\" {
            collect_contours = false;
            collect_comments = true;
        }

        if collect_preamble {
            m.preamble.push(line.to_string());
        }
        if collect_vars {
            m.vars.push(line.to_string());
        }
        if collect_points {
            m.points.push(line.to_string());
        }
        if collect_board {
            m.board.push(line.to_string());
        }
        if collect_contours {
            m.contours.push(line.to_string());
        }
        if collect_comments {
            m.comments.push(line.to_string());
        }
    }
    
    Ok(())
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
const TO_MM:f64 = 25.4;
//let mut mpr_data = Mpr::new();
let mut files: Vec<String> = Vec::new();
for entry in WalkDir::new(".") {
    files.push(entry.as_ref().expect("Invalid String").file_name().to_os_string().into_string().unwrap());
    //println!("{}", entry.unwrap().path().display());
}
//we can print file names now
for f in WalkDir::new(".") {
 let mut mpr_new = Mpr::new();
 parse(&mut mpr_new,"banana.mpr".to_string()).await?;
 mpr_new.dump();
 let mut hasVarvar = check_val(&mpr_new,Data::vars,"t","18").await?;
 if hasVarvar == true {
    println!("We Have Async BOIS LETS GO!!!!!!!!");
 } else if hasVarvar == false {
    println!("Not Found");
 }
 let thickness_var = get_var(&mpr_new,Data::vars,"l").await?;
 let thickness_val = get_val(thickness_var.as_str());
 println!("=========================");
 println!("Var: {}",thickness_var);
 println!("Val: {}",thickness_val);
 
}

Ok(())
}//end of main