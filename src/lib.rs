extern crate toml;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::env;
#[macro_use]
extern crate lazy_static;

lazy_static!{
    pub static ref IC: toml::Value= new(find_config_file());
}

fn find_config_file()->PathBuf{
    if let Some(cfp)=env_finder("IRON_CONFIG_FILE"){
        let p = PathBuf::from(cfp.clone());
        if p.is_file() {
            return p;
        }else{
            panic!("Can not find Iron.toml by env:\nIRON_CONFIG_FILE={}",cfp);
        }
    }else{
        let p=PathBuf::from("./Iron.toml");
        if p.is_file() {
            return p;
        }else{
            let  p=PathBuf::from("./site/Iron.toml");
            if p.is_file() {
                return p;
            }else{
                panic!("Can not find any Iron.toml.Please set it in env or put it in current working dir.");
            }
        }
    }
}


fn env_finder(name:&str)->Option<String>{
    if let Some(v) =  env::var_os(name) {
        return Some(env_recur(v.to_str().unwrap()));
    }else{
        return None;
    }
}

//expand ${HOME}/dir1/${DIR2}/dir3
fn env_recur(path:&str)-> String{
    //TODO:How to limit recursion times
    //cur_recur +=1;
    //if cur_recur > MAX_RECUR{return;}
    let mut s=path.splitn(2,"${");
    let head=s.next();
    let mut re = head.unwrap().to_owned();
    let tail=s.next();
    match tail {
        Some(t) => {
            let mut s2=t.splitn(2,"}");
            let name=s2.next().unwrap();
            let tail2=s2.next();
            match tail2{
                Some(t2) => {
                    if let Some(val) = env::var_os(name){
                        re.push_str(&env_recur(val.to_str().unwrap()));
                    }
                    if t2 != ""{
                        re.push_str(&env_recur(t2));
                }
                },
                None => {}
            }
        },
        None => {} 
    }
    return re;
}


fn new(conf_file_path : PathBuf)-> toml::Value{
    let mut con_file = OpenOptions::new()
        .read(true)
        .open(conf_file_path)
        .expect("Can not open file Iron.toml");

    let mut  content=String::new();
    let _ = con_file.read_to_string(&mut content);
    content.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        env::set_var("A","dir1/${B}/dir3");
        env::set_var("B","dir4/${C}/${C}");
        env::set_var("C","dir5");
        assert_eq!(env_finder("A"),Some("dir1/dir4/dir5/dir5/dir3".to_owned()));
    }
}
