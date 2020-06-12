pub mod graph {

    pub mod graph_items {

        pub mod edge {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub origin: String,
                pub destination: String,
                pub attrs: std::collections::HashMap<String, String>,
            }
            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        origin: a.to_string(),
                        destination: b.to_string(),
                        attrs: std::collections::HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for at in attrs.iter() {
                        self.attrs.insert(at.0.to_string(), at.1.to_string());
                    }
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(node: &str) -> Self {
                    Node {
                        name: node.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for at in attrs.iter() {
                        self.attrs.insert(at.0.to_string(), at.1.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    match self.attrs.get(attr) {
                        Some(x) => Some(x),
                        _ => None,
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: std::collections::HashMap<String, String>,
    }

    impl Default for Graph {
        fn default() -> Self {
            Graph::new()
        }
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::<graph_items::node::Node>::new(),
                edges: Vec::new(),
                attrs: std::collections::HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for at in attrs {
                self.attrs.insert(at.0.to_string(), at.1.to_string());
            }
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|x| x.name == name)
        }
    }
}
