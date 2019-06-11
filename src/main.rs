fn main() {
    closure_vec();
}


fn closure_vec(){
    let haystack = vec![1, 2, 3];
    //func_vec(haystack);
    let contains =  |needle| haystack.contains(needle);
    //func_vec(haystack);

    println!("{:?}", contains(&1));
    //println!("{}", contains(&4));
    //println!("There're {} elements in vec", haystack.len());
}

fn func_vec(v: Vec<i32>) -> bool{
    v.contains(&1)
}