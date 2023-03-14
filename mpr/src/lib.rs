use tokio::fs;
pub mod mpr{
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

async fn parse(f: String) -> core::result::Result<Mpr,Box<dyn std::error::Error + 'static>>{
    let contents = tokio::fs::read(f).await?;
    //let cdata = contents.into::<Vec<String>>();
    let cdata = String::from_utf8(contents).unwrap();
    //println!("{:?}",cdata);
    let mut m = Mpr::new();
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
    
    Ok(m)
}
//change vars should iterate over the MPR and modify var to be val <var="val">
async fn change_var(m: &mut Mpr, var: &str, val: &str) -> core::result::Result<bool,Box<dyn std::error::Error + 'static>>{
    let mut ret = false;
    let mut replace_string = String::new();
    let mut search_string = String::new();
    for c in var.chars() {
        search_string.push(c);
    }
    search_string.push('=');

    for c in var.chars(){
        replace_string.push(c);
    }
    replace_string.push('=');
    replace_string.push('"');
    for c in val.chars() {
        replace_string.push(c);
    }
    
            for line in &mut m.preamble {
                if line.contains(&search_string) {
                   *line = replace_string.clone();
                }
            }
     
      
            for line in &mut m.vars {
                if line.contains(&search_string) {
                    *line = replace_string.clone();
                }
            }
      
      
            for line in &mut m.board {
                if line.contains(&search_string) {
                    *line = replace_string.clone();
                }
            }
        
       
            for line in &mut m.points {
                if line.contains(&search_string) {
                    *line = replace_string.clone();
                }
            }
     
       
            for line in &mut m.contours {
                if line.contains(&search_string) {
                    *line = replace_string.clone();
                }
            }
      
       
            for line in &mut m.comments {
                if line.contains(&search_string) {
                    *line = replace_string.clone();
                }
            }
       
   Ok(false)
}

async fn write_mpr(m: &Mpr,f: &str) -> core::result::Result<bool,Box<dyn std::error::Error +'static>>{
    
    let mut buffer = String::new();
    for item in &m.preamble {
        for c in item.chars(){
            buffer.push(c);
        }
        for c in "\r\n".chars() {
            buffer.push(c);
        }
    }
    for c in "\r\n".chars() {
        buffer.push(c);
    }

    for item in &m.vars {
        for c in item.chars(){
            buffer.push(c);
        }
        for c in "\r\n".chars() {
            buffer.push(c);
        }
    }
    for c in "\r\n".chars() {
        buffer.push(c);
    }
    for item in &m.points {
        for c in item.chars(){
            buffer.push(c);
        }
        for c in "\r\n".chars() {
            buffer.push(c);
        }
    }
    for item in &m.board {
        for c in item.chars(){
            buffer.push(c);
        }
        for c in "\r\n".chars() {
            buffer.push(c);
        }
    }
    for c in "\r\n".chars() {
        buffer.push(c);
    }    
    for item in &m.contours {
        for c in item.chars(){
            buffer.push(c);
        }
        for c in "\r\n".chars() {
            buffer.push(c);
        }
    }
    for item in &m.comments {
        for c in item.chars(){
            buffer.push(c);
        }
        for c in "\r\n".chars() {
            buffer.push(c);
        }
    }
    for c in "\r\n".chars() {
        buffer.push(c);
    }
    tokio::fs::write(f,buffer.as_bytes()).await?;

Ok(false)
}


async fn point_copy_to_offset(m: &mut Mpr, offset_val: String) ->core::result::Result<bool,Box<dyn std::error::Error + 'static>> {
  //TODO: Implement point_copy_to_offset

    Ok(false)
}


} //end mod