mod data;

fn main() {
    println!("Advent of Code: Day 6");

    let mut pop = Population::new(data::DATA.to_vec());
    let result1 = solve(&mut pop.clone(), 80);
    println!("population after 80 days: {}", result1);

    let result2 = solve(&mut pop, 256);
    println!("population after 256 days: {}", result2);
}

const STAGE: usize = 9;

#[derive(Clone)]
struct Population {
    population: [u64; STAGE],
}

impl Population {
    fn new(initial: Vec<u8>) -> Self {
        let mut pop: [u64; STAGE] = [0; STAGE];
        initial.iter().for_each(|f| pop[*f as usize] += 1);
        Self { population: pop }
    }

    fn step(&mut self) {
        let spawning = self.population[0];

        // fishes move to the next stage
        for i in 1..STAGE {
            self.population[i - 1] = self.population[i];
        }

        self.population[6] += spawning;
        self.population[8] = spawning;
    }

    fn size(&self) -> u64 {
        self.population.iter().sum()
    }
}

fn solve(pop: &mut Population, steps: usize) -> u64 {
    for _ in 1..=steps {
        pop.step();
    }
    pop.size()
}

#[cfg(test)]
mod tests {
    use super::*;

    // PART 1
    #[test]
    fn test_step() {
        let mut initial = Population::new(vec![3, 4, 3, 1, 2]);
        initial.step();
        assert_eq!(
            initial.population,
            Population::new(vec![2, 3, 2, 0, 1]).population
        );
        initial.step();
        assert_eq!(
            initial.population,
            Population::new(vec![1, 2, 1, 6, 0, 8]).population
        );
        initial.step();
        assert_eq!(
            initial.population,
            Population::new(vec![0, 1, 0, 5, 6, 7, 8]).population
        );
        initial.step();
        assert_eq!(
            initial.population,
            Population::new(vec![6, 0, 6, 4, 5, 6, 7, 8, 8]).population
        );
        initial.step();
        assert_eq!(
            initial.population,
            Population::new(vec![5, 6, 5, 3, 4, 5, 6, 7, 7, 8]).population
        );
    }

    #[test]
    fn test_solve1() {
        let mut pop = Population::new(vec![3, 4, 3, 1, 2]);
        assert_eq!(solve(&mut pop.clone(), 18), 26);
        assert_eq!(solve(&mut pop, 80), 5934);
    }

    // PART 2
    #[test]
    fn test_solve2() {
        let mut pop = Population::new(vec![3, 4, 3, 1, 2]);
        assert_eq!(solve(&mut pop, 256), 26984457539);
    }
}
