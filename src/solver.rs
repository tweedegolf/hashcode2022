use super::*;

pub fn solve(mut problem: Problem) -> Solution {
    let mut current_time = 0;
    let mut solution = vec![];
    problem.projects.sort_by(|a, b| a.deadline.cmp(&b.deadline));
    for project in problem.projects {

        // skip if we can't make the deadline
        if current_time + project.duration > project.deadline {
            continue;
        }

        // find matching personel for this project
        let mut personel = vec![];
        for (skill, level) in project.roles.iter() {
            let mut found = None;
            for c in problem.contributors.iter() {
                if c.skills[*skill] >= *level && personel.iter().all(|selected| *selected != c.orig_index) {
                    found = Some(c.orig_index);
                    break;
                }
            }
            if let Some(orig_index) = found {
                personel.push(orig_index);
            } else {
                break;
            }
        }

        // personel is available, add to planning
        if personel.len() == project.roles.len() {
            solution.push((project.orig_index, personel));
            current_time += project.duration;
        }
    }
    solution
}
