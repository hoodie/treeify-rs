extern crate treeline;
extern crate yaml_rust;

use yaml_rust::yaml;
use yaml_rust::yaml::Yaml;
use treeline::Tree;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};


#[derive(Debug)]
struct YamlTree(yaml::Yaml);

struct KVPair(Yaml, Yaml);

impl fmt::Display for YamlTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self.0 {
            Yaml::Real(ref s)    => s.to_owned(),
            Yaml::Integer(ref i) => format!("{}", i),
            Yaml::String(ref s)  => s.to_owned(),
            Yaml::Boolean(ref b) => format!("{}", b),
            Yaml::Array(ref v)   => format!("{:?}", v),
            Yaml::Hash(ref h)   => format!("{:?}", h),
            Yaml::Alias(ref u)   => format!("{}: Alias", u),
            Yaml::Null       => String::from("Null"),
            Yaml::BadValue   => String::from("BadValue"),
        };

        write!(f, "{}", string)
    }
}


impl Into<Tree<String>> for KVPair {
    fn into(self) -> Tree<String> {
        if let KVPair(Yaml::String(key), Yaml::Array(values)) = self {
            Tree::new(key, values.into_iter().map(|yval| YamlTree(yval).into()).collect())
        } else if let KVPair(Yaml::String(key), Yaml::Hash(hash)) = self {
            Tree::new(key, hash.into_iter().map(|(k,v)| KVPair(k,v).into()).collect())
        } else{
            Tree::new(YamlTree(self.0).to_string(), vec![ YamlTree(self.1).into() ])
        }
    }
}


impl Into<Tree<String>> for YamlTree {
    fn into(self) -> Tree<String> {
        match self.0 {
            Yaml::Real(s)    => Tree::root(s),
            Yaml::Integer(i) => Tree::root(format!("{}", i)),
            Yaml::String(s)  => Tree::root(s),
            Yaml::Boolean(b) => Tree::root(format!("{}", b)),
            Yaml::Array(v)   => {
                let leaves = v.into_iter().map(|v| YamlTree(v).into()).collect();
                Tree::new(String::from("Array"), leaves)
            }
            Yaml::Hash(h)    => {
                let leaves = h.into_iter()
                              .map(|(k, v)| KVPair(k,v).into())
                              .collect();
                Tree::new(String::from("Hash"), leaves)
            }
            Yaml::Alias(u)   => Tree::root(format!("{}: Alias", u)),
            Yaml::Null       => Tree::root(String::from("Null")),
            Yaml::BadValue   => Tree::root(String::from("BadValue")),
        }
    }
}


fn std_in() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    if let Some(arg1) = env::args().nth(1){
        let mut file = File::open(&arg1).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let docs = yaml::YamlLoader::load_from_str(&content).unwrap();
        let doc = docs.get(0).unwrap().to_owned();

        let tree: Tree<String> = KVPair(
            Yaml::String(String::from(arg1)),
            doc).into();
        println!("{}", tree);

    } else if let Ok(content) = std_in() {
        let docs = yaml::YamlLoader::load_from_str(&content).unwrap();
        let doc = docs.get(0).unwrap().to_owned();

        let tree: Tree<String> = KVPair(
            Yaml::String(String::from("-")),
            doc).into();
        println!("{}", tree);
    } else {
        println!("no arg provided!");
    }
}
