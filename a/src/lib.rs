#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub extern fn heythere() {
    println!("hey there!");
}

#[no_mangle]
pub extern fn primes() -> Box<Iterator<Item=usize>> {
    Box::new((2..).scan(Vec::<usize>::new(), |mut prevs, n| {
        let is_prime = prevs.iter().cloned().take_while(|x| {
                *x <= ((n as f64).sqrt() as usize)
            }).all(|x| {
                n % x != 0
            });
        if is_prime {prevs.push(n);}
        Some((n, is_prime))
    }).filter_map(|tuple| {
        let (n, is_prime) = tuple;
        if is_prime {Some(n)} else {None}
    }))
}
