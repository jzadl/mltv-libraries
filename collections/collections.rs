use std::collections::HashMap;

pub fn chunk(list: Vec<i64>, size: i64) -> Vec<Vec<i64>> {
    let s = size.max(1) as usize;
    list.chunks(s).map(|c| c.to_vec()).collect()
}

pub fn unique(list: Vec<i64>) -> Vec<i64> {
    let mut seen = Vec::new();
    let mut out = Vec::new();
    for item in list {
        if !seen.contains(&item) {
            seen.push(item);
            out.push(item);
        }
    }
    out
}

pub fn flatten(list: Vec<Vec<i64>>) -> Vec<i64> {
    let mut out = Vec::new();
    for sub in list {
        out.extend(sub);
    }
    out
}

pub fn group_by(list: Vec<String>, key_fn: String) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for item in list {
        let k = item.clone();
        map.entry(k).or_default().push(item);
    }
    map
}

pub fn count_by(list: Vec<String>) -> HashMap<String, i64> {
    let mut map = HashMap::new();
    for item in list {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

pub fn zip(keys: Vec<String>, vals: Vec<i64>) -> HashMap<String, i64> {
    let mut map = HashMap::new();
    let len = keys.len().min(vals.len());
    for i in 0..len {
        map.insert(keys[i].clone(), vals[i]);
    }
    map
}

pub fn merge_maps(a: HashMap<String, i64>, b: HashMap<String, i64>) -> HashMap<String, i64> {
    let mut m = a;
    m.extend(b);
    m
}

pub fn map_keys(map: HashMap<String, i64>) -> Vec<String> {
    map.into_keys().collect()
}

pub fn map_vals(map: HashMap<String, i64>) -> Vec<i64> {
    map.into_values().collect()
}

pub fn min(list: Vec<i64>) -> i64 {
    list.into_iter().min().unwrap_or(0)
}

pub fn max(list: Vec<i64>) -> i64 {
    list.into_iter().max().unwrap_or(0)
}

pub fn sum(list: Vec<i64>) -> i64 {
    list.into_iter().sum()
}

pub fn product(list: Vec<i64>) -> i64 {
    list.into_iter().product()
}

pub fn mean(list: Vec<f64>) -> f64 {
    let len = list.len();
    if len == 0 { return 0.0; }
    list.into_iter().sum::<f64>() / len as f64
}
