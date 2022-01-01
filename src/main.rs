mod work;

fn main() {
    if let Option::Some(url) = std::env::args().last() {
        if let Result::Err(e) = work::do_work(&url) {
            println!("{}", e);
        }
    }
}
