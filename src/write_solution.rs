use glob::glob;
use std::{io::Write, path::PathBuf};
use owo_colors::OwoColorize;

use crate::{Problem, Solution};

pub fn is_improvement(path: &PathBuf, score: usize) -> bool {
    let target_stem = path
        .file_stem()
        .expect("Could not get stem of path")
        .to_str()
        .expect("Could not convert stem, to string");
    let search = format!("out/{}-*-solution.txt", target_stem);

    for entry in glob(&search).expect("Failed to read glob pattern") {
        match entry {
            Ok(found_path) => {
                let stem = found_path
                    .file_stem()
                    .expect("Could not get stem of path")
                    .to_str()
                    .expect("Could not convert stem, to string");

                let parts = stem.split('-').collect::<Vec<&str>>();
                let part = parts[1];

                if let Ok(found_score) = part.parse::<usize>() {
                    if score <= found_score {
                        println!(
                            "Did not found improvement for {:?}: {} <= {}",
                            path.red(),
                            score,
                            found_score.red()
                        );
                        return false;
                    }
                }
            }
            _ => {}
        }
    }

    true
}

pub fn write_solution(problem: &Problem, solution: &Solution, path: &PathBuf, score: usize) {
    if !is_improvement(path, score) {
        return;
    }

    println!("Found improvement for {:?}: {}", path.green(), score.green());

    let postfix = format!("-{:07}-solution.txt", score);
    let path = problem
        .name
        .clone()
        .replace("data", "out")
        .replace(".txt", &postfix);

    let mut out_file = std::fs::File::create(path).unwrap();

    write!(out_file, "{}\n", solution.len()).unwrap();

    for (project_index, contributors) in solution.iter() {
        write!(out_file, "{}\n", problem.projects[*project_index].name).unwrap();

        let names: Vec<&str> = contributors.iter().map(|contributor_index| {
            problem.contributors[*contributor_index].name.as_ref()
        }).collect();

        write!(out_file, "{}\n", names.join(" ")).unwrap();
    }
}

#[test]
fn test_write_solution() {
    use crate::{Contributor, Project, scorer};

    let path = PathBuf::from("data/a_an_example.in.txt");
    let problem = Problem {
     name: "data/a_an_example.in.txt".into(),
     skills: vec!["C++".into(), "HTML".into(), "CSS".into(), "Python".into()],
     contributors: vec![
         Contributor {
             name: "Anna".into(),
             orig_index: 0,
             skills: vec![2, 0, 0, 0],
         },
         Contributor {
            name: "Bob".into(),
            orig_index: 1,
            skills: vec![0, 5, 5, 0],
        },
        Contributor {
            name: "Maria".into(),
            orig_index: 2,
            skills: vec![0, 0, 0, 3],
        }
     ],
     projects: vec![
         Project {
             name: "Logging".into(),
             orig_index: 0,
             duration: 5,
             score: 10,
             deadline: 5,
             roles: vec![(0, 3)],
         },
         Project {
            name: "WebServer".into(),
            orig_index: 1,
            duration: 7,
            score: 10,
            deadline: 7,
            roles: vec![(1, 3), (0, 2)],
        },
        Project {
            name: "WebChat".into(),
            orig_index: 2,
            duration: 10,
            score: 20,
            deadline: 20,
            roles: vec![(3, 3), (1, 3)],
        }
     ]
    };

    let solution: Solution = vec![
        (1, vec![1, 0]),
        (0, vec![0]),
        (2, vec![2, 1]),
    ];

    let score = scorer::validated_score(&problem, &solution).unwrap();

    write_solution(&problem, &solution, &path, score);
}
