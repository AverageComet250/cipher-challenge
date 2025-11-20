use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use itertools::Itertools;
use rayon::prelude::*;

use crate::{dictionary, tools};

static MAX_GENERATIONS: u32 = 2_u32.pow(12);
static CROSSOVER_GENERATIONS: u32 = 2_u32.pow(4);
static POP_MULT: u32 = 2_u32.pow(6);
static MUTATION_RATE: f64 = 0.05;

pub fn decipher(ciphertext: &str, aligned: bool) -> Option<String> {
    let ciphertext: Arc<str> = Arc::from(ciphertext);
    let mut islands = vec![HybridHillClimb::new(ciphertext.clone(), aligned); 16];
    let finished = Arc::new(AtomicBool::new(false));

    for _i in 0..(MAX_GENERATIONS / CROSSOVER_GENERATIONS) {
        islands.par_iter_mut().for_each(|island| {
            let finished = Arc::clone(&finished);

            for _ in 0..CROSSOVER_GENERATIONS {
                if finished.load(Ordering::Relaxed) {
                    break;
                }

                if island.hybrid_breed() {
                    finished.store(true, Ordering::Relaxed);
                    break;
                }
            }
        });

        fastrand::shuffle(&mut islands);

        for pair in islands.chunks_exact_mut(2) {
            let (p1, p2) = pair.split_at_mut(1);
            p1[0].population.sort_unstable_by_key(|p| p.fitness);
            p2[0].population.sort_unstable_by_key(|p| p.fitness);
            let buf = p1[0].get_migrants();
            p1[0].set_migrants(p2[0].get_migrants());
            p1[0].set_migrants(buf);
        }

        let elite = islands
            .iter()
            .map(|island| island.get_fittest())
            .max_by_key(|phenotype| phenotype.fitness)
            .unwrap()
            .clone();

        dbg!(elite.fitness as f64 / FITNESS_MULT);
        dbg!(&elite.genes);

        let middle = islands.len() / 4 * 3;
        for island in &mut islands[..middle] {
            island.population.push(elite.clone());
        }

        dbg!("Island migration complete");
    }

    None
}

static FITNESS_MULT: f64 = u32::MAX as f64;

fn look_for_cribs(ciphertext: &str, aligned: bool) -> Vec<HashMap<char, char>> {
    let ciphertext: String = ciphertext
        .chars()
        .filter(|c| c.is_alphabetic() || c == &' ')
        .collect();

    // ciphered -> plain
    let mut partial_keys = vec![];

    if aligned {
        let possible_cribs = [
            ["a", "i", "", "", ""],
            ["to", "of", "is", "my", "it"],
            ["the", "and", "our", "for", "you"],
            ["that", "have", "your", "will", "this"],
            ["while", "dodge", "fully", "begun", "taken"],
            ["friend", "cannot", "letter", "energy", ""],
            ["charles", "support", "british", "babbage", "dickens"],
            ["american", "consider", "entirely", "", ""],
        ];

        let words = ciphertext.split(" ").collect_vec();
        for word in words {
            let len = word.chars().count();
            if len == 0 || len > 8 {
                continue;
            }
            for crib in possible_cribs[len - 1] {
                if let Some(map) = calculate_partial_map(word, crib) {
                    if !partial_keys.contains(&map) {
                        partial_keys.push(map);
                    }
                }
            }
        }
    } else {
        let possible_cribs = [
            "the", "and", "you", "that", "your", "friend", "cannot", "letter", "charles",
            "british", "babbage", "dickens", "american",
        ];

        let text_chars = ciphertext
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect_vec();
        for crib in possible_cribs {
            let size = crib.chars().count();

            for window in text_chars.as_slice().windows(size) {
                let crib_chars = crib.chars().collect_vec();
                let map: HashMap<_, _> = window
                    .iter()
                    .zip(crib_chars)
                    .map(|(k, v)| (*k, v))
                    .collect();

                if map.len() != size || !partial_keys.contains(&map) {
                    partial_keys.push(map)
                }
            }
        }
    }

    partial_keys
}

fn calculate_partial_map(word: &str, crib: &str) -> Option<HashMap<char, char>> {
    if crib.is_empty() {
        return None;
    }

    let mut current_map = HashMap::new();

    for (i, cipher_char) in word.chars().enumerate() {
        let plain_char = crib.chars().nth(i).expect("Error while calculating cribs");
        match current_map.get(&cipher_char) {
            Some(plaintext_in_map) => {
                if plain_char != *plaintext_in_map {
                    return None;
                }
            }
            None => {
                if current_map.values().contains(&plain_char) {
                    return None;
                }
                current_map.insert(
                    cipher_char,
                    crib.chars().nth(i).expect("Error while calculating cribs"),
                );
            }
        }
    }

    Some(current_map)
}

#[derive(Clone)]
struct HybridHillClimb {
    population: Vec<Phenotype>,
    pop_size: u32,
    ciphertext: Arc<str>,
    last_best: u32,
    repeats: u32,
    cribs: Vec<HashMap<char, char>>,
}

impl HybridHillClimb {
    fn new(ciphertext: Arc<str>, aligned: bool) -> HybridHillClimb {
        let cribs = look_for_cribs(&ciphertext, aligned);
        let pop_size = num_cpus::get() as u32 * POP_MULT;
        let pop: Vec<Phenotype> = (0..pop_size)
            .map(|_| {
                let mut key = dictionary::ALPHABET_ARRAY.clone().to_vec();
                fastrand::shuffle(&mut key);
                Phenotype::new_from_key(key, ciphertext.clone(), aligned)
            })
            .collect();
        HybridHillClimb {
            population: pop,
            pop_size,
            ciphertext,
            last_best: 0u32,
            repeats: 0,
            cribs,
        }
    }

    fn set_migrants(&mut self, pop_section: Vec<Phenotype>) {
        let pop_len = self.population.len();
        self.population
            .splice(pop_len - pop_section.len().., pop_section);
    }

    fn get_migrants(&mut self) -> Vec<Phenotype> {
        let fraction = 1.0 / 8.0;

        let end = (self.population.len() as f64 * (1.0 - fraction)) as usize;
        self.population[..end].to_vec()
    }

    fn hybrid_breed(&mut self) -> bool {
        let fittest_phenotype = self.get_fittest();
        let elite = fittest_phenotype.clone();
        if fittest_phenotype.fitness > (0.9 * FITNESS_MULT) as u32 {
            return true;
        }

        if self.repeats > 400 {
            return true;
        }

        if fittest_phenotype.fitness == self.last_best {
            self.repeats += 1
        } else {
            self.repeats = 0
        }

        let mutation_rate = if self.repeats > 15 {
            (MUTATION_RATE * 2.0f64.powf(self.repeats as f64 / 15.0)).clamp(0.0, 0.3333)
        } else {
            MUTATION_RATE
        };

        self.last_best = elite.fitness;

        let mut new_population: HashSet<Phenotype> = (1..self.pop_size / 4 * 3)
            .map(|_| {
                let parent1 = self.select_parent();
                let parent2 = self.select_parent();

                Phenotype::new_from_breeding(parent1, parent2, mutation_rate, &self.cribs)
            })
            .unique()
            .collect();

        while new_population.len() < self.pop_size as usize {
            let mut genes = dictionary::ALPHABET_ARRAY.to_vec();
            fastrand::shuffle(&mut genes);
            new_population.insert(Phenotype::new_from_key(
                genes,
                self.ciphertext.clone(),
                elite.aligned,
            ));
        }

        new_population.insert(elite);

        self.population = new_population.into_iter().collect_vec();

        false
    }

    fn get_fittest(&self) -> &Phenotype {
        self.population
            .iter()
            .sorted_by_key(|phenotype| phenotype.fitness)
            .next_back()
            .unwrap()
    }

    fn select_parent(&self) -> &Phenotype {
        let epsilon = 1e-6;
        let total_fitness: f64 = self
            .population
            .iter()
            .map(|phenotype| phenotype.fitness as f64 / FITNESS_MULT + epsilon)
            .sum();
        let pick = fastrand::f64() * total_fitness;
        let mut cumulative = 0.0;
        for phenotype in &self.population {
            cumulative += phenotype.fitness as f64 / FITNESS_MULT + epsilon;
            if cumulative >= pick {
                return phenotype;
            }
        }
        panic!("Something went wrong when selecting parent");
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Phenotype {
    genes: Vec<char>,
    fitness: u32,
    aligned: bool,
    ciphertext: Arc<str>,
}

impl Phenotype {
    fn new_from_key(cipher_key: Vec<char>, ciphertext: Arc<str>, aligned: bool) -> Phenotype {
        let mut phenotype = Phenotype {
            genes: cipher_key,
            fitness: 0,
            aligned,
            ciphertext,
        };
        phenotype.compute_score();
        phenotype
    }

    fn compute_score(&mut self) {
        if self.aligned {
            self.fitness = (tools::uncipher_vec_and_score(&self.genes, &self.ciphertext)
                * FITNESS_MULT) as u32;
        } else {
            self.fitness =
                (tools::uncipher_vec_and_score_non_aligned(&self.genes, &self.ciphertext)
                    * FITNESS_MULT) as u32;
        }
    }

    fn new_from_breeding(
        parent1: &Phenotype,
        parent2: &Phenotype,
        mutation_rate: f64,
        cribs: &Vec<HashMap<char, char>>,
    ) -> Phenotype {
        let cut1 = fastrand::usize(0..24);
        let cut2 = fastrand::usize(cut1..26);

        let mut child_genes = [None; 26];
        for (i, child_gene) in child_genes.iter_mut().enumerate().take(cut2 + 1).skip(cut1) {
            *child_gene = Some(parent1.genes[i]);
        }

        let parent2_genes: Vec<&char> = parent2
            .genes
            .iter()
            .filter(|gene| !child_genes.contains(&Some(**gene)))
            .collect();

        let mut j = 0;
        for child_gene in child_genes.iter_mut() {
            if child_gene.is_none() {
                *child_gene = Some(*parent2_genes[j]);
                j += 1;
            }
        }

        let mut child = Phenotype {
            genes: child_genes.iter().map(|gene| gene.unwrap()).collect(),
            fitness: 0,
            aligned: parent1.aligned,
            ciphertext: parent1.ciphertext.clone(),
        };

        while fastrand::f64() < mutation_rate {
            child.mutate(cribs);
        }
        child.compute_score();
        child
    }

    pub fn swap_mutation(&mut self) {
        for _ in 0..fastrand::u8(1..=16) {
            let i1 = fastrand::usize(0..24);
            let i2 = fastrand::usize(i1..26);
            self.genes.swap(i1, i2);
        }
    }

    fn inversion_mutation(&mut self) {
        let i1 = fastrand::usize(0..24);
        let i2 = fastrand::usize(i1..26);

        self.genes = self.genes[..i1]
            .iter()
            .chain(self.genes[i1..i2].iter().rev())
            .chain(self.genes[i2..].iter())
            .copied()
            .collect();
    }

    fn scramble_mutation(&mut self) {
        let seg_len = fastrand::usize(3..7);
        let random_i = fastrand::usize(0..=26 - seg_len);
        fastrand::shuffle(&mut self.genes[random_i..random_i + seg_len]);
    }

    fn crib_mutation(&mut self, cribs: &Vec<HashMap<char, char>>) {
        let crib = fastrand::choice(cribs);
        for (cipher_char, plain_char) in crib.unwrap() {
            let cipher_i = self.genes.iter().position(|c| c == cipher_char).unwrap();
            self.genes
                .swap((*plain_char as u8 - b'a') as usize, cipher_i);
        }
    }

    fn _letter_mutation_linear(&mut self) {
        let selected = fastrand::f64();
        let mut total = 0.0;
        let mut letter = 27;
        let freqs = dictionary::LETTER_FREQ;
        for (i, letter_freq) in freqs.iter().enumerate() {
            total += letter_freq;
            if total > selected {
                letter = i;
                break;
            }
        }

        let mut best = (self.fitness, self.genes.clone());

        for i in 0..26 {
            self.genes.clone().swap(i, letter);
            self.compute_score();
            if self.fitness > best.0 {
                best = (self.fitness, self.genes.clone());
            }
            self.genes.clone().swap(i, letter);
        }
    }

    fn letter_mutation(&mut self) {
        let selected = fastrand::f64();
        let mut total = 0.0;
        let mut letter = 27;
        let freqs = dictionary::LETTER_FREQ.iter().map(|f| 1.0 / f);
        let sum = freqs.clone().sum::<f64>();
        let freqs = freqs.map(|f| f / sum);
        for (i, letter_freq) in freqs.enumerate() {
            total += letter_freq;
            if total > selected {
                letter = i;
                break;
            }
        }

        let mut best = (self.fitness, self.genes.clone());

        for i in 0..26 {
            self.genes.clone().swap(i, letter);
            self.compute_score();
            if self.fitness > best.0 {
                best = (self.fitness, self.genes.clone());
            }
            self.genes.clone().swap(i, letter);
        }
    }

    fn mutate(&mut self, cribs: &Vec<HashMap<char, char>>) {
        match fastrand::f64() {
            r if r < 0.50 => self.swap_mutation(),
            r if r < 0.66 => self.letter_mutation(),
            r if r < 0.77 => self.inversion_mutation(),
            r if r < 0.90 => self.crib_mutation(cribs),
            _ => self.scramble_mutation(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swap_mutation() {
        fastrand::seed(1);
        let mut p = Phenotype {
            genes: dictionary::ALPHABET_ARRAY.to_vec(),
            fitness: 0,
            aligned: false,
            ciphertext: Arc::new("Hello, world"),
        };
        p.swap_mutation();
        assert_eq!(
            p.genes,
            vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'u',
                'q', 'r', 's', 't', 'p', 'v', 'w', 'x', 'y', 'z'
            ]
        );
    }
}
