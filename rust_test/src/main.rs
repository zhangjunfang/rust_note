
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::string::String;
//use std::io;
use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
//use std::io::prelude::*;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Error;
//use std::boxed::Box;
//use std::cell::RefCell;
use std::str::Split;
fn main() {
    _foo();
  //swap_file();
}

pub   fn swap_file(){
    let  a_additional_path="src/a.txt";
    let  b_additional_path="src/foo.txt";
    let  rs=get_current_dir();
    let  mut a_path=file_path_jion(&rs,a_additional_path);
    let  mut b_path=file_path_jion(&rs,b_additional_path);
    file_content_swap(get_file(&mut a_path) ,get_file(&mut b_path ));
}

fn  get_current_dir()->Result<PathBuf,Error>{
          env::current_dir()

}
fn file_path_jion<'a, 'b:'a >( rs:&'b Result< PathBuf,Error>,p:&'a str)->PathBuf{
        match *rs {
                Ok(ref pb)=>{
                        let mut path:PathBuf =PathBuf::new();
                        path.push(pb);
                        path.push(p);
                        return path;
                },
               // Err(ref e)=>{
                Err(_)=>{
                        panic!("error  happen !!!!");
                },
        }

}
fn get_file<'a>(p:&'a mut PathBuf ) -> &'a  Path{
    p.as_path()
}

//  读取两个文件  a,b   ==> a,b 文件内容互换   2017-09-20  工作内容
fn  file_content_swap(a_path :& Path,b_path :& Path )-> bool {

        let mut a_file=OpenOptions::new().read(true).write(true).append(true).open(a_path).unwrap();

        let mut b_file=OpenOptions::new().read(true).write(true).append(true).open(b_path).unwrap();

        let mut a_content=String::new();

        let mut b_content=String::new();
        let a_r_file=a_file.read_to_string(&mut a_content);
        let b_r_file=b_file.read_to_string(&mut b_content);

        if  w_r_file_state(a_r_file,b_r_file){

            print!(" sucess !!!  ");
            let  a= a_content.as_str();
            let  b= b_content.as_str();

            if a.is_empty() && b.is_empty() {
                let a:Split<&str>=a.split("\n");
                let b:Split<&str>=b.split("\n");

                ////--------------------------这里有逻辑性错误------------------------------------------
//                if a >= b {
//
//                    println!("a>=b");
//                }else{
//                    println!("a<b");
//                }

                return true;
            }else {

                return false;
            }




        }

    false


}

fn  w_r_file_state(a_r_file:Result<usize,Error>,b_r_file:Result<usize,Error>)-> bool{
    let   a_file_state;
    let   b_file_state;
    match a_r_file {
        Ok(_)=>{
            a_file_state=true;
        },
        Err(_)=>{
            a_file_state= false;
        },
    }
    match b_r_file {
        Ok(_)=>{
            b_file_state=  true ;
        },
        Err(_)=>{
            b_file_state= false;
        },
    }
    let last_rs= if  a_file_state && b_file_state {
        true
    }else {

        false
    };
    last_rs
}
//
//fn  content_swap_all(a_content:& mut str,b_content:& mut str,a_file:& mut File,b_file:& mut File)->bool{//a_content:&'a mut str,b_content:&'a mut str //,a_file:&'b mut File,b_file:&'b mut File
//
//  //  let mut  a =a_content.split("\n");
//
//  //  let mut  b =b_content.split("\n");
//
//  //  println!("===={},-----{}---{}----{}",a,b,a_file,b_file);
//
////    for m in arrys {
////        if !m.is_empty() {
////            println!("----{}", m);
////        }
////
////    }
//
//
//   true
//}


// 打开文件 --> 设置 读写  以及 追加模式    -->  写入数据  -->一次性读出全部文件--->逐行形式显示
// 遗留问题::  如何逐行读取文件内容
fn _foo()  {
        // 如何获取当前路径
        let path = env::current_dir();
        match path {
                Ok(pb)=>{
                       // println!("The current directory is {}", pb.display());

                        let mut path=PathBuf::new();
                        path.push(pb);
                        path.push("src/a.txt");
                        let path=path.as_path();

                        //let path=  Path::new("/home/zby/rustworkspace/rust_test/src/a.txt");
                        //设置文件的读写模式   也可以设置文件权限
                        let mut file=OpenOptions::new().read(true).write(true).append(true).open(path).unwrap();  //.create(true)
                        //向打开的文件中  写入数据
                        file.write_all(  b"65464614132323123\n").ok();
                        //  file.flush();


                        /// 读取文件中的数据


                        //let mut  line=String::new();  //string 能使用这个或者
                        //let mut line="".to_string(); //string  也可以使用这种方式  性能比较差
                        let mut line =String::from(""); //string  也可以使用这种方式
                        //io::stdin().read_line(&mut line); //从控制台读取一行数据
                        file.seek(SeekFrom::Start(0u64)).ok();
                        let rs=file.read_to_string( &mut line);//一次读出全部的内容
                        match rs {
                                Ok(_)=>{
                                        let  arrys= line.split("\n");
                                        for m in arrys {
                                                if !m.is_empty() {
                                                        println!("----{}", m);
                                                }

                                        }
                                },
                                Err(e)=>{

                                        println!("error ::  {}  !!!",e);

                                },
                        }
                    //如何按行读取!!!!!!!!!!!!!!!!!!!!!
                    {

                        use std::io::BufWriter;

                        use std::io::BufReader;

                        let file = File::open("./foo.txt").unwrap();

                        let mut fin = BufReader::new(file);



                        for line in fin.lines() {

                            println!("-------------------------{}", line.unwrap());

                        }


                    }
                },
                Err(e)=>{
                        println!("The current directory is {:?}", e);
                },
        }
}