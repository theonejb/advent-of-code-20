#[cfg(test)]
mod bags_graph_tests {
    use crate::bags_graph::Graph;

    #[test]
    fn test_number_of_routes_to() {
        let mut graph = Graph::new();
        let lines = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        for line in lines.iter() {
            graph.add_rule_from_description(*line);
        }

        assert_eq!(4, graph.number_of_routes_to("shiny gold"));
    }

    #[test]
    fn test_number_of_bags_needed() {
        let mut graph = Graph::new();
        let lines = [
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];
        for line in lines.iter() {
            graph.add_rule_from_description(*line);
        }

        assert_eq!(126, graph.length_of_rabbit_hole("shiny gold"));
    }
}