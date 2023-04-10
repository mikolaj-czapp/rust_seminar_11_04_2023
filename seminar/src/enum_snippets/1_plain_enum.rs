fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn sample(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => {println!("I'm a V4!")}
            IpAddrKind::V6 => {println!("I'm a V6!")}
        }
    }

    sample(four);
    sample(six)
}
