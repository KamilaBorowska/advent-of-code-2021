use crate::Solution;
use std::iter;

struct Tracking<I> {
    iter: I,
    count: u64,
}

impl<I> Iterator for Tracking<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        self.count += 1;
        self.iter.next()
    }
}

fn bits(input: &str) -> Tracking<impl Iterator<Item = bool> + '_> {
    Tracking {
        iter: input.chars().flat_map(|c| {
            let d = c.to_digit(16).expect("a digit");
            (0..4).rev().map(move |i| d >> i & 1 != 0)
        }),
        count: 0,
    }
}

fn get_int(bits: &mut Tracking<impl Iterator<Item = bool>>, count: u8) -> u64 {
    let mut value = 0;
    for _ in 0..count {
        value <<= 1;
        value |= u64::from(bits.next().expect("a bit"));
    }
    value
}

fn get_version_sum(bits: &mut Tracking<impl Iterator<Item = bool>>) -> u64 {
    let mut version = get_int(bits, 3);
    if get_int(bits, 3) == 4 {
        loop {
            let has_continuation = bits.next().expect("a bit");
            get_int(bits, 4);
            if !has_continuation {
                break;
            }
        }
    } else if bits.next().expect("a bit") {
        for _ in 0..get_int(bits, 11) {
            version += get_version_sum(bits);
        }
    } else {
        let mut destination = get_int(bits, 15);
        destination += bits.count;
        while bits.count != destination {
            version += get_version_sum(bits);
        }
    }
    version
}

fn binary_op(mut iter: impl Iterator<Item = u64>, op: impl Fn(u64, u64) -> bool) -> u64 {
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    u64::from(op(a, b))
}

fn calculate(bits: &mut Tracking<impl Iterator<Item = bool>>) -> u64 {
    get_int(bits, 3);
    let type_id = get_int(bits, 3);
    if type_id == 4 {
        let mut value = 0;
        loop {
            let has_continuation = bits.next().expect("a bit");
            value <<= 4;
            value |= get_int(bits, 4);
            if !has_continuation {
                return value;
            }
        }
    }
    let is_number_of_subpackets = bits.next().expect("a bit");
    let mut iter1;
    let mut iter2;
    let iter: &mut dyn Iterator<Item = u64> = if is_number_of_subpackets {
        iter1 = (0..get_int(bits, 11)).map(|_| calculate(bits));
        &mut iter1
    } else {
        let mut destination = get_int(bits, 15);
        destination += bits.count;
        iter2 = iter::from_fn(move || (bits.count != destination).then(|| calculate(bits)));
        &mut iter2
    };
    match type_id {
        0 => iter.sum(),
        1 => iter.product(),
        2 => iter.min().unwrap(),
        3 => iter.max().unwrap(),
        5 => binary_op(iter, |a, b| a > b),
        6 => binary_op(iter, |a, b| a < b),
        7 => binary_op(iter, |a, b| a == b),
        _ => unreachable!(),
    }
}

pub(super) const DAY16: Solution = Solution {
    part1: |input| Ok(get_version_sum(&mut bits(input)).to_string()),
    part2: |input| Ok(calculate(&mut bits(input)).to_string()),
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY16.part1,
        example1: "8A004A801A8002F478" => 16,
        example2: "620080001611562C8802118E34" => 12,
        example3: "C0015000016115A2E0802F182340" => 23,
        example4: "A0016C880162017C3686B18A3D4780" => 31,
        input: 925,
    );
    test!(
        DAY16.part2,
        example1: "C200B40A82" => 3,
        example2: "04005AC33890" => 54,
        example3: "880086C3E88112" => 7,
        example4: "CE00C43D881120" => 9,
        example5: "D8005AC2A8F0" => 1,
        example6: "F600BC2D8F" => 0,
        example7: "9C005AC2F8F0" => 0,
        example8: "9C0141080250320F1802104A08" => 1,
        input: 342997120375,
    );
}
