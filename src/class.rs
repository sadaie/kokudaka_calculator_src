use lazy_static::lazy_static;

lazy_static! {
    static ref HANS: Vec<Han> = {
        let mut hans = include_str!("../hans.txt").lines().map(|l| {
            let row = l.split(',').collect::<Vec<&str>>();
            Han {
                han_name: row[0].to_owned(),
                family_name: row[1].to_owned(),
                kokudaka: row[2].parse().expect("must be parsed"),
            }
        }).collect::<Vec<Han>>();
        hans.reverse();

        hans
    };
}

trait GetValue<T> {
    fn get(self) -> T;
}

impl<T> GetValue<T> for Result<T, T>
where
    T: Copy,
{
    fn get(self) -> T {
        if let Some(o) = self.as_ref().ok() {
            *o
        } else {
            self.err().unwrap()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Han {
    pub han_name: String,
    pub family_name: String,
    pub kokudaka: u64,
}

pub enum Class {
    Over,
    CloseTo(Han),
    Less,
}

pub fn find_closest_han(target: u64) -> Class {
    if target < HANS.first().expect("must exist").kokudaka {
        Class::Less
    } else if target > HANS.last().expect("must exist").kokudaka {
        Class::Over
    } else {
        let estimated_index = HANS.binary_search_by(|c| c.kokudaka.cmp(&target)).get();
        println!("target: {}, estimated idx: {}", target, estimated_index);
        //println!("target: {}, han: {}", target, han.kokudaka);
        Class::CloseTo(HANS[estimated_index].clone())
    }
}

#[test]
fn test_find_closest_han() {
    (0..2000000).for_each(|i| {
        match (i, find_closest_han(i)) {
            (0..=186000, Class::Less) => assert!(true),
            (1200000..=2000000, Class::Over) => assert!(true),
            (_, Class::CloseTo(c)) => {
                assert!(true);
            },
            _ => assert!(false),
        }
    });
}
