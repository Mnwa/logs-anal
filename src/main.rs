use clap::{value_t, value_t_or_exit, App, Arg};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::FromIterator;
use std::ops::Add;
use std::str::FromStr;

fn main() {
    let matches = App::new("Logs Analytics Tool")
        .version("0.1.0")
        .author("Mnwa")
        .about("Analyze logs with comfort")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .index(1),
        )
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .value_name("Enum")
                .help("Runs that command ")
                .possible_values(&["sort", "head", "skip", "tail", "uniq", "wc"])
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("Number")
                .help("Lines for skip or take rows")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("column")
                .short("k")
                .long("column")
                .value_name("Number")
                .help("Column for sorting")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("String")
                .help("Delimiter for sorting (default is whitespace)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("order")
                .short("o")
                .long("order")
                .value_name("ORDER_TYPE")
                .help("Sorting order")
                .possible_values(&["asc", "desc"])
                .takes_value(true),
        )
        .get_matches();

    let stdout = std::io::stdout();
    let stdout_locked = BufWriter::new(stdout.lock());

    let action = value_t_or_exit!(matches.value_of("command"), String);
    let path_wrapped = value_t!(matches.value_of("INPUT"), String);

    let stdin: Box<dyn BufRead> = match path_wrapped {
        Ok(path) => Box::new(BufReader::new(File::open(path).expect("File not exists"))),
        Err(_) => Box::new(BufReader::new(std::io::stdin())),
    };

    match action.as_str() {
        "sort" => sort(
            stdin,
            stdout_locked,
            matches
                .value_of("order")
                .map_or(Order::Asc, |order| Order::from_str(order).unwrap()),
            value_t!(matches.value_of("column"), usize).map_or(None, |v| Some(v)),
            value_t!(matches.value_of("delimiter"), String).map_or(None, |v| Some(v)),
        ),
        "head" => head(
            stdin,
            stdout_locked,
            matches.value_of("lines").map_or(0, |count| {
                usize::from_str(count).expect("lines must be a number")
            }),
        ),
        "skip" => skip(
            stdin,
            stdout_locked,
            matches.value_of("lines").map_or(0, |count| {
                usize::from_str(count).expect("lines must be a number")
            }),
        ),
        "tail" => tail(
            stdin,
            stdout_locked,
            matches.value_of("lines").map_or(0, |count| {
                usize::from_str(count).expect("lines must be a number")
            }),
        ),
        "uniq" => uniq(stdin, stdout_locked),
        "wc" => wc(stdin, stdout_locked),
        _ => panic!("Unknown action"),
    }
}

fn sort<Writable: Write, Readable: BufRead>(
    stdin: Readable,
    stdout: Writable,
    order: Order,
    column: Option<usize>,
    delimiter: Option<String>,
) {
    match column {
        Some(c) => sort_btree(stdin, stdout, order, c, delimiter),
        None => sort_heap(stdin, stdout, order),
    };
}

fn sort_btree<Writable: Write, Readable: BufRead>(
    stdin: Readable,
    stdout: Writable,
    order: Order,
    column: usize,
    delimiter: Option<String>,
) {
    let btree = stdin
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let sorted_key = match &delimiter {
                None => line
                    .split_whitespace()
                    .map(|a| a.to_string())
                    .take(column)
                    .last(),
                Some(d) => line
                    .split(d.as_str())
                    .map(|a| a.to_string())
                    .take(column)
                    .last(),
            };

            match sorted_key {
                Some(key) => (key.to_string(), line),
                None => (line.clone(), line),
            }
        })
        .collect::<BTreeMap<String, String>>();

    match order {
        Order::Asc => pass_to_stdout(stdout, btree.values().into_iter()),
        Order::Desc => pass_to_stdout(stdout, btree.values().into_iter().rev()),
    }
}

fn sort_heap<Writable: Write, Readable: BufRead>(stdin: Readable, stdout: Writable, order: Order) {
    let heap = stdin
        .lines()
        .map(|line| line.unwrap())
        .collect::<BinaryHeap<String>>()
        .into_sorted_vec()
        .into_iter();

    match order {
        Order::Asc => pass_to_stdout(stdout, heap),
        Order::Desc => pass_to_stdout(stdout, heap.rev()),
    }
}

fn uniq<Writable: Write, Readable: BufRead>(stdin: Readable, stdout: Writable) {
    let btree = stdin
        .lines()
        .map(|line| line.unwrap())
        .collect::<BTreeSet<String>>()
        .into_iter();
    pass_to_stdout(stdout, btree);
}

fn wc<Writable: Write, Readable: BufRead>(stdin: Readable, mut stdout: Writable) {
    let data = stdin.lines().map(|line| line.unwrap()).collect::<WcData>();

    write!(stdout, "{} {} {}", data.lines, data.words, data.bytes).unwrap();
}

fn head<Writable: Write, Readable: BufRead>(stdin: Readable, stdout: Writable, count: usize) {
    pass_to_stdout(stdout, stdin.lines().take(count).map(|d| d.unwrap()));
}

fn skip<Writable: Write, Readable: BufRead>(stdin: Readable, stdout: Writable, count: usize) {
    pass_to_stdout(stdout, stdin.lines().skip(count).map(|d| d.unwrap()));
}

fn tail<Writable: Write, Readable: BufRead>(stdin: Readable, stdout: Writable, count: usize) {
    let mut s_data: VecDeque<String> = VecDeque::with_capacity(5);
    stdin.lines().for_each(|line| {
        if s_data.len() == count {
            s_data.pop_front();
        }
        s_data.push_back(line.unwrap());
    });
    pass_to_stdout(stdout, s_data.iter());
}

fn pass_to_stdout<Writable: Write, Item: ToString, Iter: Iterator<Item = Item>>(
    mut stdout: Writable,
    data: Iter,
) {
    data.for_each(|line| {
        stdout
            .write(line.to_string().add("\n").as_bytes())
            .expect("write line error");
    });
}

struct WcData {
    lines: usize,
    words: usize,
    bytes: usize,
}

enum Order {
    Asc,
    Desc,
}

impl FromStr for Order {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        }
    }
}

impl<A: ToString> FromIterator<A> for WcData {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut result = WcData {
            lines: 0,
            words: 0,
            bytes: 0,
        };
        for line in iter {
            let s_line = line.to_string();
            result.bytes += s_line.as_bytes().len();
            result.words += s_line.split_whitespace().count();
            result.lines += 1;
        }
        result
    }
}
