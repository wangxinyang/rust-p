use getaver;

fn main() {
    let mut aver_obj = getaver::AverCollect::new();
    aver_obj.add(1);
    println!("1 average = {}", aver_obj.average());

    aver_obj.add(10);
    println!("2 average = {}", aver_obj.average());

    aver_obj.add(5);
    println!("3 average = {}", aver_obj.average());

    aver_obj.add(3);
    println!("4 average = {}", aver_obj.average());

    aver_obj.add(2);
    println!("5 average = {}", aver_obj.average());

    println!("{:?}", aver_obj);
    println!("hello world");
}
