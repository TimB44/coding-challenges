use std::iter::Flatten;

const OPS: [fn(Rational, Rational) -> Rational; 4] =
    [Rational::add, Rational::sub, Rational::mul, Rational::div];

pub struct Solution;
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        check_eq_24(cards.into())
    }
}

fn check_eq_24(cur: MSet) -> bool {
    println!("cur = {:?}", cur);
    if cur.size() == 1 {
        return cur.into_iter().next().unwrap().eq_24();
    }

    cur.into_iter()
        .flat_map(|cur_n| {
            let left = cur.remove(cur_n);
            left.into_iter().flat_map(move |n| {
                let new_cur = left.remove(n);

                OPS.into_iter()
                    .map(move |op| check_eq_24(new_cur.add(op(cur_n, n))))
            })
        })
        .any(|x| x)
}

#[derive(Clone, Copy, Debug)]
struct MSet {
    members: [Option<Rational>; 4],
}

impl From<Vec<i32>> for MSet {
    fn from(value: Vec<i32>) -> Self {
        let arr: [_; 4] = value.try_into().unwrap();
        Self {
            members: arr.map(|x| Some(Rational::new(x))),
        }
    }
}
impl IntoIterator for MSet {
    type Item = Rational;
    type IntoIter = Flatten<std::array::IntoIter<Option<Rational>, 4>>;
    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter().flatten()
    }
}

impl MSet {
    fn add(self, i: Rational) -> Self {
        let mut new_self = self;
        let open = new_self.members.iter().position(Option::is_none).unwrap();
        new_self.members[open] = Some(i);
        new_self
    }
    fn remove(self, i: Rational) -> Self {
        let mut new_self = self;
        let i = new_self.members.iter().position(|x| *x == Some(i)).unwrap();
        new_self.members[i] = None;
        new_self
    }
    fn size(&self) -> usize {
        self.members.into_iter().filter(Option::is_some).count()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rational {
    num: i32,
    denom: i32,
}

impl Rational {
    fn new(num: i32) -> Self {
        Self { num, denom: 1 }
    }
}

impl Rational {
    fn add(self, other: Self) -> Self {
        Self {
            num: self.num * other.denom + other.num * self.denom,
            denom: self.denom * other.denom,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            num: self.num * other.denom - other.num * self.denom,
            denom: self.denom * other.denom,
        }
    }

    fn mul(self, other: Self) -> Self {
        Self {
            num: self.num * other.num,
            denom: self.denom * other.denom,
        }
    }
    fn div(self, other: Self) -> Self {
        self.mul(Self {
            num: other.denom,
            denom: other.num,
        })
    }

    fn eq_24(self) -> bool {
        self.denom != 0 && self.num % self.denom == 0 && self.num / self.denom == 24
    }
}
