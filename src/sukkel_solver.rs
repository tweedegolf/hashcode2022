use super::*;

pub fn solve(mut problem: Problem) -> Solution {
    let mut current_time = 0;
    let mut solution = vec![];

    problem.projects.sort_by(|a, b| {
        if a.deadline == b.deadline {
            b.score.cmp(&a.score)
        } else {
            a.deadline.cmp(&b.deadline)
        }
    });

    for project in problem.projects {

        // skip if we can't make the deadline
        if (current_time + project.duration) as i32 - project.deadline as i32 > project.score as i32 {
            continue;
        }

        // find matching personel for this project
        let mut personel = vec![];
        let mut mentoring = vec![false; problem.skills.len()];
        for (skill, level) in project.roles.iter() {
            let mut found = None;

            for c in problem.contributors.iter() {
                if mentoring[*skill] && c.skills[*skill] == *level - 1 && personel.iter().all(|selected| *selected != c.orig_index) {
                    found = Some(c.orig_index);
                    mentoring[*skill] = false;
                    break;
                }
            }

            for c in problem.contributors.iter() {
                if c.skills[*skill] >= *level && personel.iter().all(|selected| *selected != c.orig_index) {
                    found = Some(c.orig_index);

                    for (mentor_skill, mentor_level) in c.skills.iter().enumerate() {
                        if mentor_skill != *skill {
                            for (s, l) in project.roles.iter() {
                                if *mentor_level >= *l {
                                    mentoring[*s] = true;
                                }
                            }
                        }
                    }

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
            for (skill_idx, person) in personel.iter().enumerate() {
                let (skill, learner_level) = project.roles[skill_idx];
                if problem.contributors[*person].skills[skill] == learner_level {
                    problem.contributors[*person].skills[skill] += 1;
                }
            }
            solution.push((project.orig_index, personel));
            current_time += project.duration;
        }
    }
    solution
}
