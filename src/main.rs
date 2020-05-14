use std::process::Command;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let output = Command::new("xdotool")
        .arg("getactivewindow")
        .arg("getwindowgeometry")
        .arg("--shell")
        .output()
        .expect("xdotool failed to get window geometry for active window");

    let foo: HashMap<&str, &str> = String::from_utf8(output.stdout)
        .expect("unable to get output from command for window geometry")
        .lines()
        .map(|line| {
            let pair = line.splitn(2, "=").collect::<Vec<&str>>();
            (pair[0], pair[1])
        })
        .collect();

    println!("{}", foo.len())
    /*
    for (k, v) in foo {
        println!("{} {}", k, v);
    }
     */

       // .for_each(|line| { println!("{}", line)})
       //.map(|line| line.splitn(2, "="))

       /*
        .lines()
        .map(|x| x.splitn(2, "=").collect()::vec![&str])
        .collect();
        */
}
