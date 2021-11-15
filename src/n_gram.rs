pub fn n_gram(candidate_fragments: &[String], reference_fragments: &[String], n: usize) -> Vec<usize> {
    n_gram_base(
        candidate_fragments,
        reference_fragments,
        n,
        &|candidate, reference_fragments, _| check_fragment(candidate, reference_fragments, None)
    )
}

pub fn modified_n_gram(candidate_fragments: &[String], reference_fragments: &[String], n: usize) -> Vec<usize> {
    n_gram_base(
        candidate_fragments,
        reference_fragments,
        n,
        &|candidate, reference_fragments, matches| check_fragment(candidate, reference_fragments, Some(&matches))
    )
}

pub fn n_gram_base(candidate_fragments: &[String], reference_fragments: &[String], n: usize, matcher: &dyn Fn(&[String], &[String], &[usize]) -> Vec<usize>) -> Vec<usize> {
    let mut matches = vec![];
    for from_idx in 0..candidate_fragments.len()-n+1 {
        let candidate = &candidate_fragments[from_idx..from_idx+n];
        let mut matched = matcher(candidate, reference_fragments, &matches);
        // println!("{:?}_h{:?}", candidate, matched);
        matches.append(&mut matched);
    }
    matches
}

fn check_fragment(candidate: &[String], reference_fragments: &[String], skip: Option<&[usize]>) -> Vec<usize> {
    let mut matches = vec![];
    for from_idx in 0..reference_fragments.len()-candidate.len()+1 {
        if let Some(skipped_idx) = skip {
            if skipped_idx.contains(&from_idx) {
                continue;
            }
        }
        // println!("f{}_t{}", from_idx, from_idx+candidate.len());
        // println!("c{:?}_r{:?}", candidate, &reference_fragments[from_idx..from_idx+candidate.len()]);
        if candidate == &reference_fragments[from_idx..from_idx+candidate.len()] {
            matches.push(from_idx);
        }
    }
    matches
}

fn fragments(input: &str) -> Vec<String>{
    input.split(' ').into_iter().map(|f| f.to_string().to_lowercase()).collect()
}

fn percision(candidate_fragments: &[String], reference_fragments: &[String], n: usize) -> f32 {
    modified_n_gram(candidate_fragments, reference_fragments, n).len() as f32 / candidate_fragments.len() as f32
}

fn brevity_penalty(candidate_fragments: &[String], reference_fragments: &[String]) -> f32{
    if candidate_fragments.len() > reference_fragments.len() {
        1.
    } else {
        let c_len = candidate_fragments.len() as f32;
        let r_len = reference_fragments.len() as f32;
        f32::EPSILON.powf(1. - (r_len/c_len))
    }
}

fn bleu_metric(candidate_fragments: &[String], reference_fragments: &[String], n: usize) -> f32 {
    let sum = (1..n+1).into_iter().map(|v| {
        let w_n = 1. / n as f32;
        let p_n = percision(candidate_fragments, reference_fragments, v);
        w_n * p_n.log10()
    }).sum::<f32>();
    let bp = brevity_penalty(candidate_fragments, reference_fragments);
    bp * sum.exp()
}

#[test]
fn test_percision(){
    let cand = fragments("the the the the the the the");
    let reference = fragments("The cat is on the mat");
    let p = percision(&cand, &reference, 1);
    println!("2/7 ~ {} = {}", 2./7., p);
}

#[test]
fn test_n_gram(){
    let cand = fragments("Mary had a little lamb of floof.");
    let reference = fragments("Mary had a little long lamb full of floof.");
    n_gram(&cand, &reference, 2);
}

#[test]
fn test_n_gram_vs_modified(){
    let frags = n_gram(
        &fragments("had had had had"),
        &fragments("Mary had a little lamb"),
        1
    );
    assert_eq!(frags, vec![1, 1, 1, 1]);

    let frags = modified_n_gram(
        &fragments("had had had had"),
        &fragments("Mary had a little lamb"),
        1
    );
    assert_eq!(frags, vec![1]);

    let frags = modified_n_gram(
        &fragments("had had had had"),
        &fragments("Mary had had a little lamb"),
        1
    );
    assert_eq!(frags, vec![1, 2]);
}

#[test]
fn test_fragments(){
    let frags = check_fragment(
        &fragments("Mary had"),
        &fragments("Mary had a little lamb"),
        None
    );
    assert_eq!(frags, vec![0]);

    let frags = check_fragment(
        &fragments("a little"),
        &fragments("Mary had a little lamb"),
        None
    );
    assert_eq!(frags, vec![2]);
    
    let frags = check_fragment(
        &fragments("lamb"),
        &fragments("Mary had a little lamb"),
        None
    );
    assert_eq!(frags, vec![4]);

    let frags = check_fragment(
        &fragments("chicken"),
        &fragments("Mary had a little lamb"),
        None
    );
    let expected: Vec<usize> = vec![];
    assert_eq!(frags, expected);
}