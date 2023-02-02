use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Debug)]
pub struct Universe {
    pub bound: (u32, u32),
    pub gen: u32,
    pub alive: Vec<Point>,
    pub died: Vec<Point>,
    pub born: Vec<Point>,
}

impl Universe {
    pub fn new(bound: (u32, u32), genesis: Vec<Point>) -> Universe {
        Universe {
            bound,
            gen: 0,
            alive: genesis,
            died: Vec::new(),
            born: Vec::new(),
        }
    }

    fn neighbors(&self, p: &Point) -> HashSet<Point> {
        let Point(x, y) = *p;
        let ns = [
            Point(x + 1, y),
            Point(x - 1, y),
            Point(x, y + 1),
            Point(x, y - 1),
            Point(x + 1, y + 1),
            Point(x - 1, y + 1),
            Point(x + 1, y - 1),
            Point(x - 1, y - 1),
        ];
        let (xsize, ysize) = self.bound;
        HashSet::from(ns)
            .into_iter()
            .filter(|p| {
                let Point(p_x, p_y) = *p;
                p_x >= 0 && p_x < (xsize as i32) && p_y >= 0 && p_y < (ysize as i32)
            })
            .collect()
    }

    fn live_neighbors(&self, alive: &HashSet<Point>, p: &Point) -> HashSet<Point> {
        let ns = self.neighbors(p);
        ns.into_iter().filter(|n| alive.contains(n)).collect()
    }

    fn dead_neighbors(&self, alive: &HashSet<Point>, p: &Point) -> HashSet<Point> {
        let ns = self.neighbors(p);
        ns.into_iter().filter(|n| !alive.contains(n)).collect()
    }

    pub fn tick(self) -> Universe {
        let mut alive: HashSet<Point> = self.alive.iter().map(|p| *p).collect();

        for p in &self.died {
            alive.remove(p);
        }
        for p in &self.born {
            alive.insert(*p);
        }

        let died: HashSet<Point> = alive
            .iter()
            .filter(|p| {
                let len = self.live_neighbors(&alive, p).len();
                len < 2 || len > 3
            })
            .map(|p| *p)
            .collect();

        let born: HashSet<Point> = alive
            .iter()
            .flat_map(|p| self.dead_neighbors(&alive, p))
            .filter(|p| self.live_neighbors(&alive, p).len() == 3)
            .collect();

        Universe {
            bound: self.bound,
            gen: self.gen + 1,
            alive: alive.into_iter().collect(),
            died: died.into_iter().collect(),
            born: born.into_iter().collect(),
        }
    }
}
