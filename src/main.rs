mod work;

fn main() {
    if let Result::Err(e) = work::do_work() {
        println!("{}", e);
    }
}
