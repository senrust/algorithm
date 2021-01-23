#[derive(Debug)]
struct Proc{
    name: String,
    proctime: i32,
}


fn main() {
    let info: Vec<i32> = input::read_numline();
    let mut procs: Vec<Proc> = Vec::new();
    let size = info[0];
    let time = info[1];
    for _ in 0..size {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().unwrap();
        let mut iter = line.split_whitespace();
        let proc_name = match iter.next() {
            Some(name) => name,
            _ => "none",
        };
        let proc_time = match iter.next() {
            Some(time) => time.parse::<i32>().ok().unwrap(),
            _ => 0,
        };
        procs.push(Proc{name: proc_name.to_string(), proctime: proc_time});
    }

    let mut total_time = 0;
    while procs.len() > 0 {
        let mut proc = procs.remove(0);
        if proc.proctime <= time {
            total_time += proc.proctime;
            println!("{} finished at {}", proc.name, total_time);
        } else {
            total_time += time;
            proc.proctime -= time;
            procs.push(proc);
        }
    }
}

