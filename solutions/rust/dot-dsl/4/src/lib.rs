pub mod graph {
    pub use std::collections::HashMap;

    macro_rules! impl_attributes {
        () => {
            pub fn attr<'a>(&'a self, name: &str) -> Option<&'a str> {
                self.attrs.get(name).map(|v| v.as_str())
            }

            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self
            where
                Self: Sized,
            {
                self.attrs
                    .extend(attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())));
                self
            }
        };
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Default, Clone, PartialEq, Debug)]
            pub struct Edge {
                node1: String,
                node2: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: Default::default(),
                    }
                }

                impl_attributes!();
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Default, Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: Default::default(),
                    }
                }

                impl_attributes!();
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    #[derive(Default, Clone, PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|&node| node.name == name).cloned()
        }

        impl_attributes!();
    }
}
