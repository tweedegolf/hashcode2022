use super::*;

pub fn score(problem: &Problem, solution: &Solution) -> usize {
    let mut time_available: Vec<usize> = Vec::new();
    let mut score = 0;

    time_available.resize(problem.contributors.len(), 0);

    for choice in solution {
        let mut time_start = 0;
        for i in &choice.1 {
            if time_available[*i] > time_start {
                time_start = time_available[*i];
            }
        }

        let time_end = time_start + problem.projects[choice.0].duration;
        if time_end <= problem.projects[choice.0].deadline {
            score += problem.projects[choice.0].score;
        } else if time_end <= problem.projects[choice.0].deadline + problem.projects[choice.0].score {
            score += problem.projects[choice.0].score + problem.projects[choice.0].deadline - time_end;
        }

        for i in &choice.1 {
            time_available[*i] = time_end;
        }
    }

    score
}

pub fn validated_score(problem: &Problem, solution: &Solution) -> Option<usize> {
    let mut problem = problem.to_owned();

    let mut time_available: Vec<usize> = Vec::new();
    let mut score = 0;

    let mut project_used: Vec<bool> = Vec::new();
    project_used.resize(problem.projects.len(), false);

    time_available.resize(problem.contributors.len(), 0);

    for choice in solution {
        if project_used[choice.0] {
            println!("Warning: Project {} used double", choice.0);
            return None
        }
        project_used[choice.0] = true;

        if choice.1.len() != problem.projects[choice.0].roles.len() {
            println!("Warning: Incorrect number of people on project {}", choice.0);
            return None
        }

        let mut time_start = 0;
        for (idx, i) in choice.1.iter().enumerate() {
            if time_available[*i] > time_start {
                time_start = time_available[*i];
            }

            if problem.contributors[*i].skills[problem.projects[choice.0].roles[idx].0] == problem.projects[choice.0].roles[idx].1-1 {
                let mut ok = false;
                for j in &choice.1 {
                    if problem.contributors[*j].skills[problem.projects[choice.0].roles[idx].0] >= problem.projects[choice.0].roles[idx].1 {
                        ok = true;
                    }
                }
                if !ok {
                    println!("Warning: Contributor {} is missing mentoring for role {} in project {}", *i, idx, choice.0);
                    return None;
                }
            } else if problem.contributors[*i].skills[problem.projects[choice.0].roles[idx].0] < problem.projects[choice.0].roles[idx].1 {
                println!("Warning: Contributor {} is not skilled enough for role {} in project {}", *i, idx, choice.0);
                return None
            }
        }

        let time_end = time_start + problem.projects[choice.0].duration;
        if time_end <= problem.projects[choice.0].deadline {
            score += problem.projects[choice.0].score;
        } else if time_end <= problem.projects[choice.0].deadline + problem.projects[choice.0].score {
            score += problem.projects[choice.0].score + problem.projects[choice.0].deadline - time_end;
        }

        for (idx,i) in choice.1.iter().enumerate() {
            time_available[*i] = time_end;
            if problem.contributors[*i].skills[problem.projects[choice.0].roles[idx].0] <= problem.projects[choice.0].roles[idx].1 {
                problem.contributors[*i].skills[problem.projects[choice.0].roles[idx].0] += 1
            }
        }
    }

    Some(score)
}
