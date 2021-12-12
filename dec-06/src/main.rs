fn main() {
    println!("Advent of Code: Day 6");
}

#[derive(Clone)]
struct Population {
    population: Vec<u8>,
}

impl Population {
    fn new(initial: Vec<u8>) -> Self {
        Self {
            population: initial,
        }
    }

    fn step(&mut self) {
        let mut new_fish: Vec<u8> = Vec::new();
        self.population.iter_mut().for_each(|f| {
            if *f == 0 {
                *f = 6;
                new_fish.push(8)
            } else {
                *f -= 1;
            }
        });
        self.population.append(&mut new_fish);
    }

    fn len(&self) -> usize {
        self.population.len()
    }
}

fn solve1(pop: &mut Population, steps: usize) -> usize {
    for _ in 1..=steps {
        pop.step();
    }
    pop.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    // PART 1
    #[test]
    fn test_step() {
        let mut initial = Population::new(vec![3, 4, 3, 1, 2]);
        initial.step();
        assert_eq!(initial.population, [2, 3, 2, 0, 1]);
        initial.step();
        assert_eq!(initial.population, [1, 2, 1, 6, 0, 8]);
        initial.step();
        assert_eq!(initial.population, [0, 1, 0, 5, 6, 7, 8]);
        initial.step();
        assert_eq!(initial.population, [6, 0, 6, 4, 5, 6, 7, 8, 8]);
        initial.step();
        assert_eq!(initial.population, [5, 6, 5, 3, 4, 5, 6, 7, 7, 8]);
    }

    #[test]
    fn test_solve1() {
        let pop = Population::new(vec![3, 4, 3, 1, 2]);
        assert_eq!(solve1(&mut pop.clone(), 18), 26);
        assert_eq!(solve1(&mut pop.clone(), 80), 5934);
    }
}
