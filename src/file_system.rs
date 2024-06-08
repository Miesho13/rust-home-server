// this file contain implementation for  
// servers file system logic 
//
// @author: marcin.ryzewskii@gmail.com
// @date: 2024-05-15


// strcutre what has 

// -- DIR --
// DIR parent_dir
// vec<DIR> vectero of path in out graf 
// vec<FILE> vector of file 
// path current path in string 

pub struct file_structure { root: Dir}

struct File {
    size: u32,
    contetn: String,
}

struct Dir {
    parent_dir: Dir,
    dirs: Vec<Dir>,
    files: Vec<File>,
    path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::Dir;

    #[test]
    fn foo() {

    }

} 
