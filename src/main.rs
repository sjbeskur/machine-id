mod linux;

fn main() {

    match linux::get_machine_id(){
        Ok(result) =>{
            println!("machine_id : {}", result);
        },
        Err(e) =>{
            eprintln!("{}",e);
        }
    }
    
}
