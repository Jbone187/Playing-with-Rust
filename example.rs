use std::process::Command;


fn main(){
    let a = "Hello from A";

   match a {
       "Data" => println!("From a"),
       _ => println!("Error")
   }

   let c = 5;
   let d = 6;
   let value = c + d;

   println!("{}", value);

add(10, 45);

loop_the();

another("How are you");

bash();

}


fn add(e:u8, f:u8){

let total = e + f;

println!("{}", total);

}

fn loop_the(){

   let mut list = vec![];

    for j in 1..10{

        list.push(j);

        println!("{:?}",list);
    }

}

fn another(word: &str){

       println!("{}", word);

}

fn bash(){

    Command::new("ls")
    .current_dir(".")
    .spawn()
    .expect("Error");
}
