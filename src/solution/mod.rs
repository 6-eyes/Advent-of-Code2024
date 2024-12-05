use super::{Solution, Error};
use std::collections::HashMap;

pub struct Day1;

impl Solution for Day1 {
    fn part_1(&self, input: String) -> Result<usize, Error> {
        let mut re: (Vec<usize>, Vec<usize>) = input.lines().map(|l| {
            let mut s = l.split_whitespace();
            let parse = |s: Option<&str>| s.expect("invalid input").parse::<usize>().expect("unable to parse input");
            (parse(s.next()), parse(s.next()))
        }).unzip();

        re.0.sort();
        re.1.sort();

        let ans = re.0.into_iter().zip(re.1).map(|(v1, v2)| v1.abs_diff(v2)).sum::<usize>();

        Ok(ans)
    }

    fn part_2(&self, input: String) -> Result<usize, Error> {
        let re: (Vec<usize>, Vec<usize>) = input.lines().map(|l| {
            let mut s = l.split_whitespace();
            let parse = |s: Option<&str>| s.expect("invalid input").parse::<usize>().expect("unable to parse input");
            (parse(s.next()), parse(s.next()))
        }).unzip();

        let data = re.1.into_iter().fold(HashMap::<usize, (usize, usize)>::new(), |mut acc, val| {
            acc.entry(val).and_modify(|v| v.1 += 1).or_insert((0, 1));
            acc
        });

        Ok(re.0.into_iter().fold(data, |mut acc, val| {
            acc.entry(val).and_modify(|v| v.0 += 1);
            acc
        }).into_iter().map(|(k, (v1, v2))| k * v1 * v2).sum::<usize>())
    }
}

pub struct Day2;

impl Solution for Day2 {
    fn part_1(&self, input: String) -> Result<usize, Error> {
        Ok(input.lines().fold(0, |acc, l| {
            let nums = l.split_whitespace().map(|v| v.parse::<usize>().expect("unable to parse input. Input not a number!")).collect::<Vec<usize>>();
            let mut increasing = None;
            for win in nums.windows(2) {
                let (left, right) = (win[0], win[1]);
                let diff = right.abs_diff(left);
                if !(1..=3).contains(&diff) { return acc; }
                match increasing {
                    None => increasing = Some(right > left),
                    Some(true) => if right < left { return acc; },
                    Some(false) => if left < right { return acc; },
                };
            }
            acc + 1
        }))
    }

    fn part_2(&self, input: String) -> Result<usize, Error> {
        Ok(input.lines().fold(0, |acc, l| {
            let nums = l.split_whitespace().map(|v| v.parse::<usize>().expect("unable to parse input. Input not a number!")).collect::<Vec<usize>>();
            for i in 0..nums.len() {
                let nums = nums[0..i].iter().chain(&nums[i + 1..]).map(usize::to_owned).collect::<Vec<usize>>();
                if Self::valid_levels(nums) { return acc + 1 }
            }
            acc
        }))
    }
}

impl Day2 {
    fn valid_levels(levels: Vec<usize>) -> bool {
        let mut increasing = None;
        for win in levels.windows(2) {
            let (left, right) = (win[0], win[1]);
            let diff = right.abs_diff(left);
            if !(1..=3).contains(&diff) { return false; }
            match increasing {
                None => increasing = Some(right > left),
                Some(true) => if right < left { return false; },
                Some(false) => if left < right { return false; },
            };
        }

        true
    }
}

pub struct Day3;

impl Solution for Day3 {
    fn part_1(&self, input: String) -> Result<usize, Error> {
        let initial = "mul(";
        let comma = ",";
        let end_parenthesis = ")";
        Ok(input.lines().fold(0, |mut acc, l| {
            let get_num = |start| {
                let num = &l[start..].chars().take_while(char::is_ascii_digit).collect::<String>();
                num.parse::<usize>().ok().map(|v| (v, num.len()))
            };

            let mut idx = 0;
            // min length is mul(0,0), i.e. 4 more than initials. Plus 1 for getting index.
            while idx <= l.len() - initial.len() - 4 {
                if l[idx..].starts_with(initial) {
                    idx += initial.len();

                    if let Some((first, len)) = get_num(idx) {
                        idx += len;

                        if &l[idx..idx + 1] == comma {
                            idx += comma.len();
                            if let Some((second, len)) = get_num(idx) {
                                idx += len;

                                if &l[idx..idx + 1] == end_parenthesis {
                                    idx += end_parenthesis.len();
                                    acc += first * second;
                                }
                            }
                        }
                    }
                }
                else {
                    idx += 1;
                }
            }
            acc
        }))
    }

    fn part_2(&self, input: String) -> Result<usize, Error> {
        let initial = "mul(";
        let do_instruction = "do()";
        let dont_instruction = "don't()";
        let comma = ",";
        let end_parenthesis = ")";

        let mut enabled = true;
        Ok(input.lines().fold(0, |mut acc, l| {
            let get_num = |start| {
                let num = &l[start..].chars().take_while(char::is_ascii_digit).collect::<String>();
                num.parse::<usize>().ok().map(|v| (v, num.len()))
            };

            let mut idx = 0;
            // min length is mul(0,0), i.e. 4 more than initials. Plus 1 for getting index.
            while idx <= l.len() - initial.len() - 4 {
                if l[idx..].starts_with(do_instruction) {
                    enabled = true;
                    idx += do_instruction.len();
                }
                else if l[idx..].starts_with(dont_instruction) {
                    enabled = false;
                    idx += dont_instruction.len();
                }

                if enabled && l[idx..].starts_with(initial) {
                    idx += initial.len();

                    if let Some((first, len)) = get_num(idx) {
                        idx += len;

                        if &l[idx..idx + 1] == comma {
                            idx += comma.len();
                            if let Some((second, len)) = get_num(idx) {
                                idx += len;

                                if &l[idx..idx + 1] == end_parenthesis {
                                    idx += end_parenthesis.len();
                                    acc += first * second;
                                }
                            }
                        }
                    }
                }
                else {
                    idx += 1;
                }
            }
            acc
        }))
    }
}

pub struct Day4;

impl Solution for Day4 {
    fn part_1(&self, input: String) -> Result<usize, Error> {
        let word = "XMAS";
        let grid = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let w = grid.first().expect("invalid input").len() as isize;
        let h = grid.len() as isize;
        let char_at = |x: isize, y: isize| grid[y as usize][x as usize];

        let directions = [-1, 0, 1].into_iter().flat_map(|y| [-1, 0, 1].into_iter().map(move |x| (x, y))).filter(|&(x, y)| x != 0 || y != 0).collect::<Vec<(isize, isize)>>();
        let word_vec = word.chars().collect::<Vec<char>>();

        Ok((0..h).fold(0, |h_acc, y| {
            (0..w).fold(0, |mut w_acc, x| {
                w_acc += directions.iter().fold(0, |acc, (del_x, del_y)| {

                    let mut match_count = 0;
                    let (mut i, mut j) = (x, y);
                    while i > -1 && i < w && j > -1 && j < h && match_count < word_vec.len() && char_at(i, j) == word_vec[match_count] {
                        match_count += 1;
                        i += del_x;
                        j += del_y;
                    }

                    match match_count == word.len() {
                        true => acc + 1,
                        false => acc,
                    }
                });
                w_acc
            }) + h_acc
        }))
    }

    fn part_2(&self, input: String) -> Result<usize, Error> {
        let grid = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let w = grid.first().expect("invalid input").len() as isize;
        let h = grid.len() as isize;

        if h < 3 || w < 3 {
            return Ok(0);
        }

        let char_at = |x: isize, y: isize| grid[y as usize][x as usize];

        Ok((1..h - 1).fold(0, |h_acc, y| {
            (1..w - 1).fold(0, |w_acc, x| {
                match char_at(x, y) == 'A' && (char_at(x - 1, y - 1) == 'M' && char_at(x + 1, y + 1) == 'S' || char_at(x - 1, y - 1) == 'S' && char_at(x + 1, y + 1) == 'M') && (char_at(x - 1, y + 1) == 'M' && char_at(x + 1, y - 1) == 'S' || char_at(x - 1, y + 1) == 'S' && char_at(x + 1, y - 1) == 'M') {
                    true => w_acc + 1,
                    false => w_acc,
                }
            }) + h_acc
        }))

    }
}
