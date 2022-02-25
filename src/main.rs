use glob::glob;
use std::path::PathBuf;

mod read_problem;
mod scorer;
mod sukkel_solver;
mod solver;
mod par_solver;
mod write_solution;

#[derive(Debug, Clone, Default)]
pub struct Problem {
    name: String,
    skills: Vec<String>,
    contributors: Vec<Contributor>,
    projects: Vec<Project>,
}

#[derive(Debug, Clone, Default)]
pub struct Contributor {
    name: String,
    orig_index: usize,
    skills: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct Project {
    name: String,
    orig_index: usize,
    duration: usize,
    score: usize,
    deadline: usize,
    roles: Vec<(usize, usize)>,
}

pub type Solution = Vec<(usize, Vec<usize>)>;

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "data/*.in.txt")]
    files: String,
}

fn main() {
    use clap::Parser;
    let args = Args::parse();
    let paths: Vec<PathBuf> = glob(&args.files)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .collect();

    let mut total_score = 0;

    for path in paths.into_iter() {
        println!("Reading file {:?}", path);
        let contents = std::fs::read_to_string(&path).unwrap();
        let problem = read_problem::read_problem(contents, &path);
        let solution = par_solver::par_solve(problem.clone());
        // let solution = sukkel_solver::solve(problem.clone());
        let score = scorer::validated_score(&problem, &solution).unwrap();
        total_score += score;
        println!("Problem '{}': {}", problem.name, score);
        write_solution::write_solution(&problem, &solution, &path, score);
    }

    println!("Total score: {}", total_score);
}
