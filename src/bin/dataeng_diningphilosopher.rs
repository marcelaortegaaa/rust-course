/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/

// Why do they eat with two forks? What?

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(name: &str, fork_a: Arc<Fork>, fork_b: Arc<Fork>) -> Philosopher {
        if fork_a.id <= fork_b.id {
            Philosopher {
                name: name.to_string(),
                left_fork: fork_a,
                right_fork: fork_b,
            }
        } else {
            Philosopher {
                name: name.to_string(),
                left_fork: fork_b,
                right_fork: fork_a,
            }
        }

        // No ID needed if you make them take the lower fork first, no deadlock
    }

    fn eat(&self, stats: &TableStats) {
        let wait_start = Instant::now();

        // Try to acquire forks
        let _first_guard = self.left_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, self.left_fork.id);

        let _second_guard = self.right_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, self.right_fork.id);

        // Record how long we waited for forks
        let wait_duration = wait_start.elapsed();
        stats.record_wait(&self.name, wait_duration);

        // Now eat
        println!("{} is eating.", self.name);
        let eat_start = Instant::now();
        thread::sleep(Duration::from_millis(500));
        let eat_duration = eat_start.elapsed();

        stats.record_eating(&self.name, eat_duration);

        println!("{} finished eating.", self.name);
        println!("{} put down fork {}.", self.name, self.left_fork.id);
        println!("{} put down fork {}.", self.name, self.right_fork.id);
    }

    fn think(&self) {
        println!("{} is thinking.", self.name);
        thread::sleep(Duration::from_millis(100)); // Thinking is faster than eating
    }
}

struct PhilosopherStats {
    times_eaten: usize,
    total_wait_time: Duration,
    total_eating_time: Duration,
    longest_wait: Duration,
}

impl PhilosopherStats {
    fn new() -> Self {
        PhilosopherStats {
            times_eaten: 0,
            total_wait_time: Duration::from_secs(0),
            total_eating_time: Duration::from_secs(0),
            longest_wait: Duration::from_secs(0),
        }
    }
}

struct TableStats {
    stats: Mutex<HashMap<String, PhilosopherStats>>,
    simulation_start: Instant,
}

impl TableStats {
    fn new() -> Self {
        TableStats {
            stats: Mutex::new(HashMap::new()),
            simulation_start: Instant::now(),
        }
    }

    fn record_wait(&self, name: &str, wait_duration: Duration) {
        let mut stats = self.stats.lock().unwrap();
        let philosopher_stats = stats
            .entry(name.to_string())
            .or_insert(PhilosopherStats::new());

        philosopher_stats.total_wait_time += wait_duration;
        if wait_duration > philosopher_stats.longest_wait {
            philosopher_stats.longest_wait = wait_duration;
        }
    }

    fn record_eating(&self, name: &str, eating_duration: Duration) {
        let mut stats = self.stats.lock().unwrap();
        let philosopher_stats = stats
            .entry(name.to_string())
            .or_insert(PhilosopherStats::new());

        philosopher_stats.times_eaten += 1;
        philosopher_stats.total_eating_time += eating_duration;
    }

    fn print_report(&self) {
        let stats = self.stats.lock().unwrap();
        let total_time = self.simulation_start.elapsed();

        println!("\n{:=<70}", "");
        println!("SIMULATION STATISTICS");
        println!("{:=<70}", "");
        println!("Total simulation time: {:.2?}", total_time);
        println!();

        // Collect stats for analysis
        let mut eat_counts: Vec<usize> = Vec::new();
        let mut wait_times: Vec<Duration> = Vec::new();

        println!(
            "{:<20} {:>8} {:>12} {:>12} {:>12}",
            "Philosopher", "Ate", "Total Wait", "Avg Wait", "Longest Wait"
        );
        println!("{:-<70}", "");

        for (name, philosopher_stats) in stats.iter() {
            let avg_wait = if philosopher_stats.times_eaten > 0 {
                philosopher_stats.total_wait_time / philosopher_stats.times_eaten as u32
            } else {
                Duration::from_secs(0)
            };

            println!(
                "{:<20} {:>8} {:>12.2?} {:>12.2?} {:>12.2?}",
                name,
                philosopher_stats.times_eaten,
                philosopher_stats.total_wait_time,
                avg_wait,
                philosopher_stats.longest_wait
            );

            eat_counts.push(philosopher_stats.times_eaten);
            wait_times.push(philosopher_stats.total_wait_time);
        }

        println!("\n{:=<70}", "");
        println!("FAIRNESS ANALYSIS");
        println!("{:=<70}", "");

        if !eat_counts.is_empty() {
            let total_meals: usize = eat_counts.iter().sum();
            let max_meals = *eat_counts.iter().max().unwrap();
            let min_meals = *eat_counts.iter().min().unwrap();
            let avg_meals = total_meals as f64 / eat_counts.len() as f64;

            println!("Total meals served: {}", total_meals);
            println!("Average meals per philosopher: {:.2}", avg_meals);
            println!("Most meals by one philosopher: {}", max_meals);
            println!("Least meals by one philosopher: {}", min_meals);
            println!(
                "Meal distribution ratio (max/min): {:.2}",
                max_meals as f64 / min_meals.max(1) as f64
            );
        }

        println!("{:=<70}\n", "");
    }
}

fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    //we only have 4 forks at the table
    let forks = (0..4)
        .map(|id| {
            Arc::new(Fork {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect::<Vec<_>>();

    let philosophers = vec![
        ("Jürgen Habermas", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Karl Marx", 2, 3),
        ("Thomas Piketty", 3, 0),
        ("Michel Foucault", 0, 1),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Epicurus", 0, 1),
        ("Zeno of Citium", 1, 2),
        ("Thales of Miletus", 2, 3),
    ]
    .into_iter()
    .map(|(name, left, right)| {
        Philosopher::new(name, Arc::clone(&forks[left]), Arc::clone(&forks[right]))
    })
    .collect::<Vec<_>>();

    let stats = Arc::new(TableStats::new());

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            let stats = Arc::clone(&stats);
            thread::spawn(move || {
                for round in 1..=3 {
                    println!("{} starting round {}", philosopher.name, round);
                    philosopher.eat(&stats);
                    philosopher.think();
                }
                println!("{} is done with all meals!", philosopher.name);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    stats.print_report();
}
