fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    //map迭代器适配器
    //复用Iterator trait提供的迭代功能，还能通过闭包自定义部分具体行为
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    for v in v1_iter {
        println!("Got: {}", v);
    }
}
