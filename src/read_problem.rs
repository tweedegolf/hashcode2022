use std::collections::HashMap;

use super::*;

pub fn read_problem(content: String, path: &PathBuf) -> Problem {
    let mut lines = content.lines();
    let name = path
        .to_str()
        .expect("Could not convert path to string")
        .to_owned();

    let mut problem = Problem::default();

    problem.name = name;

    let mut header = lines.next().unwrap().split(' ');
    let c: usize = header.next().unwrap().parse().unwrap();
    let p: usize = header.next().unwrap().parse().unwrap();

    let mut skill_index = HashMap::new();

    for _ in 0..c {
        let mut cur_contributor = Contributor::default();
        let mut cheader = lines.next().unwrap().split(' ');
        cur_contributor.name = cheader.next().unwrap().to_owned();
        cur_contributor.skills.resize(problem.skills.len(), 0);
        let n: usize = cheader.next().unwrap().parse().unwrap();

        for _ in 0..n {
            let mut skill = lines.next().unwrap().split(' ');
            let skill_name = skill.next().unwrap();
            let level: usize = skill.next().unwrap().parse().unwrap();
            if let Some(idx) = skill_index.get(skill_name) {
                cur_contributor.skills[*idx] = level;
            } else {
                let idx = problem.skills.len();
                skill_index.insert(skill_name.to_owned(), idx);
                problem.skills.push(skill_name.to_owned());
                cur_contributor.skills.push(level);
            }
        }

        cur_contributor.orig_index = problem.contributors.len();

        problem.contributors.push(cur_contributor);
    }

    for _ in 0..p {
        let mut cur_project = Project::default();
        let mut pheader = lines.next().unwrap().split(' ');
        cur_project.name = pheader.next().unwrap().to_owned();
        cur_project.duration = pheader.next().unwrap().parse().unwrap();
        cur_project.score = pheader.next().unwrap().parse().unwrap();
        cur_project.deadline = pheader.next().unwrap().parse().unwrap();
        let r: usize = pheader.next().unwrap().parse().unwrap();

        for _ in 0..r {
            let mut role = lines.next().unwrap().split(' ');
            let role_skill = role.next().unwrap();
            let role_level: usize = role.next().unwrap().parse().unwrap();
            if let Some(idx) = skill_index.get(role_skill) {
                cur_project.roles.push((*idx, role_level));
            } else {
                let idx = problem.skills.len();
                problem.skills.push(role_skill.to_owned());
                skill_index.insert(role_skill.to_owned(), idx);
                cur_project.roles.push((idx, role_level));
            }
        }

        cur_project.orig_index = problem.projects.len();

        problem.projects.push(cur_project);
    }

    for contributor in &mut problem.contributors {
        contributor.skills.resize(problem.skills.len(), 0);
    }

    problem
}
