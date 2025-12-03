fn main() {

    let v = vec![101,255,331,200];
    //v owns the object currently

    let v2 = v;
    //two pointers to the same content is not allowed in rust

    println!("{:?}",v);
    //This is incorrect as two variables cannot point to the same heap
}
