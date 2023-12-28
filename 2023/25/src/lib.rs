use graphrs::{algorithms::community::louvain, Edge, Graph, GraphSpecs};

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut graph = Graph::<&str, &str>::new(GraphSpecs::undirected_create_missing());

    let edges = input
        .trim()
        .lines()
        .map(|line| {
            let (source, suffix) = line.split_once(':').ok_or(format!("bad line: {}", line))?;

            let edges = suffix[1..]
                .split_whitespace()
                .map(|target| Edge::<&str, &str>::new(source, target))
                .collect::<Vec<_>>();

            Ok::<Vec<_>, String>(edges)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    graph.add_edges(edges).map_err(|e| e.to_string())?;

    let best = louvain::louvain_partitions(&graph, false, Some(0f64), Some(4f64), None)
        .map_err(|e| e.to_string())?;

    Ok(best
        .first()
        .ok_or("bad louvain".to_owned())?
        .iter()
        .map(|partitions| partitions.len())
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(54), get_part_one(INPUT));
    }
}
