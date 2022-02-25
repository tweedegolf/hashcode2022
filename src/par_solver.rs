use std::cmp::Ordering;

use super::*;

pub fn par_solve(mut problem: Problem) -> Solution {
    let mut solution = vec![];

    problem.projects.sort_by(|a, b| {
        if a.deadline == b.deadline {
            b.score.cmp(&a.score)
        } else {
            a.deadline.cmp(&b.deadline)
        }
    });

    let mut cont_order = vec![];
    cont_order.reserve(problem.contributors.len());
    for i in 0..problem.contributors.len() {
        cont_order.push(i);
    }

    let mut time_available = vec![];
    time_available.resize(problem.contributors.len(), 0);

    let mut person_suggested = vec![];
    person_suggested.resize(problem.contributors.len(), false);
    for project in problem.projects.iter() {
        // find matching personel for this project
        let mut personel: Vec<usize> = vec![];
        personel.resize(project.roles.len(), problem.contributors.len());
        let mut personel_selected = 0;

        let mut skill_order = vec![];
        for i in 0..project.roles.len() {
            skill_order.push(i)
        }
        skill_order.sort_by(|a,b| {project.roles[*b].1.cmp(&project.roles[*a].1)});
        for i in skill_order.iter() {
            let skill = project.roles[*i].0;
            let mut level = project.roles[*i].1;
            for p in personel.iter() {
                if *p == problem.contributors.len() {
                    continue;
                }
                if problem.contributors[*p].skills[skill] >= level {
                    level -= 1;
                    break;
                }
            }

            cont_order.sort_by(|a, b| -> Ordering {
                let a_ok = problem.contributors[*a].skills[skill] >= level;
                let b_ok = problem.contributors[*b].skills[skill] >= level;
                if a_ok && !b_ok {
                    return Ordering::Less;
                }
                if b_ok && !a_ok {
                    return Ordering::Greater;
                }
                let cmp_available = time_available[*a].cmp(&time_available[*b]);
                if cmp_available.is_eq() {
                    let a_skills = problem.contributors[*a].skills.iter().fold(0, |c, f| f + c);
                    let b_skills = problem.contributors[*a].skills.iter().fold(0, |c, f| f + c);
                    a_skills.cmp(&b_skills)
                } else {
                    cmp_available
                }
            });

            let mut found = None;
            for c in cont_order.iter() {
                if problem.contributors[*c].skills[skill] < level {
                    break;
                }
                if !person_suggested[*c] {
                    found = Some(*c);
                    break;
                }
            }
            if let Some(orig_index) = found {
                personel_selected+=1;
                person_suggested[orig_index] = true;
                personel[*i] = orig_index;
            } else {
                break;
            }
        }

        for p in personel.iter() {
            if *p == problem.contributors.len() {
                continue;
            }
            person_suggested[*p] = false;
        }

        // personel is available, add to planning if useful
        if personel_selected == project.roles.len() {
            let mut time_start = 0;
            for c in personel.iter() {
                if time_available[*c] > time_start {
                    time_start = time_available[*c];
                }
            }

            let time_end = project.duration + time_start;
            if time_end >= project.deadline + project.score {
                //ignore
                continue;
            }

            for (idx, c) in personel.iter().enumerate() {
                time_available[*c] = time_end;
                if problem.contributors[*c].skills[project.roles[idx].0] <= project.roles[idx].1 {
                    problem.contributors[*c].skills[project.roles[idx].0] += 1;
                }
            }
            solution.push((project.orig_index, personel));
        }
    }
    solution
}
