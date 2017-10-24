extern  crate   redis;

//use redis::Commands;
use redis::Client;
//use redis::IntoConnectionInfo;
use redis::ConnectionInfo;
use redis::ConnectionAddr;
//use redis::Connection;
use redis::ConnectionLike;
fn main() {
    let c= get_redis_clientt();
    get_redis_connect(&c);
    get_redis_connect2(&c);
}

fn  get_redis_connect2(c:&Client){
    let conn=c.get_connection();
    match conn {
        Ok(t)=>{
            let cmd=b"set name zhangboyu"  ;
            let rs=t.req_packed_command(cmd);
            match rs {
                Ok(r)=>{
                    println!("============rrrrrrrrrrrr={:?}",r);
                },
                Err(e)=>{
                    println!("=============eeeeeeeeeeeeeeee={:?}",e);
                },
            }
        },
        Err(e)=>{

            println!("rrrrrrrrrrrr=error :{:?}",e);
        },

    }
}


fn  get_redis_connect(c:&Client){
    let conn=c.get_connection();
    match conn {
        Ok(t)=>{
            let cmd=b"set name zhangboyu"  ;
            let rs=t.req_packed_command(cmd);
            match rs {
                Ok(r)=>{
                    println!("============rrrrrrrrrrrr={:?}",r);
                },
                Err(e)=>{
                    println!("=============eeeeeeeeeeeeeeee={:?}",e);
                },
            }
        },
        Err(e)=>{

            println!("rrrrrrrrrrrr=error :{:?}",e);
        },

    }
}

fn  get_redis_clientt()->Client{

    let  client=ConnectionInfo{
        addr:Box::new(ConnectionAddr::Tcp(String::from("10.0.2.222"),6379)),
        db:0,
        passwd:Some(String::from("")),
    };
    let conn=Client::open(client);

    return  conn.unwrap();
}
