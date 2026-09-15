use simple::get;

fn main() -> Result<(), std::io::Error> {
    let before_hotreload = get();
    assert_eq!(before_hotreload, 1);

    print!("\n");
    let _sync = std::io::stdin().read_line(&mut String::new())?;

    let after_hotreload = get();
    assert_eq!(after_hotreload, 2);

    dbg!(before_hotreload, after_hotreload);
    Ok(())
}
