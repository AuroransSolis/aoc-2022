#[derive(Clone, Copy, Debug)]
struct Directory<'a> {
    name: &'a str,
    parentid: Option<usize>,
    size: usize,
    startid: usize,
    endid: usize,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parentid: Option<usize>) -> Self {
        Directory {
            name,
            parentid,
            size: 0,
            startid: 0,
            endid: 0,
        }
    }
}

// ASSUMPTIONS:
//    - `ls` is only done on each dir once
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut cursor = 12;
    let mut tree = Vec::new();
    tree.push(Directory::new("/", None));
    let mut count = 0;
    read_dirs_p1(input, &mut cursor, &mut tree, 0, &mut count);
    count
}

fn read_dirs_p1<'a>(
    input: &'a [u8],
    cursor: &mut usize,
    tree: &mut Vec<Directory<'a>>,
    pwd: usize,
    count: &mut usize,
) -> usize {
    let mut size = 0;
    tree[pwd].startid = tree.len();
    while *cursor < input.len() {
        if input[*cursor] == b'd' {
            *cursor += 4;
            let name = read_name::<b'\n'>(input, cursor);
            *cursor += 1;
            tree.push(Directory::new(name, Some(pwd)));
        } else if input[*cursor] == b'$' {
            break;
        } else {
            let filesize = util::readnum::<usize, b' '>(input, cursor);
            while input[*cursor] != b'\n' {
                *cursor += 1;
            }
            *cursor += 1;
            size += filesize;
        }
    }
    *cursor += 5;
    tree[pwd].endid = tree.len();
    while *cursor < input.len() {
        if input[*cursor] == b'.' {
            if let Some(pid) = tree[pwd].parentid {
                tree[pid].size += size;
            }
            tree[pwd].size = size;
            *cursor += 8;
            break;
        }
        let find_name = read_name::<b'\n'>(input, cursor);
        *cursor += 6;
        let Directory {
            startid: start,
            endid: end,
            ..
        } = tree[pwd];
        let new_pwd = (start..end)
            .find(|&ind| tree[ind].name == find_name)
            .unwrap();
        let child_size = read_dirs_p1(input, cursor, tree, new_pwd, count);
        size += child_size;
    }
    if size < 100_000 {
        *count += size;
    }
    size
}

const MAXSIZE: usize = 70_000_000;
const REQFREE: usize = 30_000_000;

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut cursor = 12;
    let mut tree = Vec::new();
    tree.push(Directory::new("/", None));
    let total_size = read_dirs_p2(input, &mut cursor, &mut tree, 0);
    tree[0].size = total_size;
    let total_free = MAXSIZE - total_size;
    let min_free = REQFREE - total_free;
    let mut min_size = usize::MAX;
    for dir in &tree[1..] {
        if dir.size > min_free && dir.size < min_size {
            min_size = dir.size;
        }
    }
    min_size
}

fn read_dirs_p2<'a>(
    input: &'a [u8],
    cursor: &mut usize,
    tree: &mut Vec<Directory<'a>>,
    pwd: usize,
) -> usize {
    let mut size = 0;
    tree[pwd].startid = tree.len();
    while *cursor < input.len() {
        if input[*cursor] == b'd' {
            *cursor += 4;
            let name = read_name::<b'\n'>(input, cursor);
            *cursor += 1;
            tree.push(Directory::new(name, Some(pwd)));
        } else if input[*cursor] == b'$' {
            break;
        } else {
            let filesize = util::readnum::<usize, b' '>(input, cursor);
            while input[*cursor] != b'\n' {
                *cursor += 1;
            }
            *cursor += 1;
            size += filesize;
        }
    }
    *cursor += 5;
    tree[pwd].size = size;
    tree[pwd].endid = tree.len();
    while *cursor < input.len() {
        if input[*cursor] == b'.' {
            if let Some(pid) = tree[pwd].parentid {
                tree[pid].size += size;
            }
            *cursor += 8;
            break;
        }
        let find_name = read_name::<b'\n'>(input, cursor);
        *cursor += 6;
        let Directory {
            startid: start,
            endid: end,
            ..
        } = tree[pwd];
        let new_pwd = (start..end)
            .find(|&ind| tree[ind].name == find_name)
            .unwrap();
        let child_size = read_dirs_p2(input, cursor, tree, new_pwd);
        size += child_size;
    }
    size
}

#[allow(dead_code)]
fn print_dirs(tree: &[Directory], pwd: usize, depth: usize) {
    let parent = tree[pwd];
    for _ in 0..depth {
        print!("  ");
    }
    println!("- {} (size = {})", parent.name, parent.size);
    for childid in parent.startid..parent.endid {
        print_dirs(tree, childid, depth + 1);
    }
}

fn read_name<'a, const END: u8>(input: &'a [u8], cursor: &mut usize) -> &'a str {
    let start = *cursor;
    while input[*cursor] != END {
        *cursor += 1;
    }
    unsafe { std::str::from_utf8_unchecked(&input[start..*cursor]) }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fs::read_to_string;

    #[test]
    fn p1test() {
        let input = read_to_string("../inputs/d7test.txt").unwrap();
        assert_eq!(super::part1(&input), 95437);
    }

    #[test]
    fn p2test() {
        let input = read_to_string("../inputs/d7test.txt").unwrap();
        assert_eq!(super::part2(&input), 24933642);
    }
}
