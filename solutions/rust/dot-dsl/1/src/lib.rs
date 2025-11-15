pub mod graph {
    pub use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            pub use crate::graph::Attrs;

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
            }

            impl Attrs for Edge {
                fn attrs(&self) -> &HashMap<String, String> {
                    &self.attrs
                }

                fn attrs_mut(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            use crate::graph::Attrs;

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
            }

            impl Attrs for Node {
                fn attrs(&self) -> &HashMap<String, String> {
                    &self.attrs
                }

                fn attrs_mut(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
                }
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
            nodes.iter().for_each(|node| self.nodes.push(node.clone()));
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            edges.iter().for_each(|edge| self.edges.push(edge.clone()));
            self
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|&node| node.name == name).cloned()
        }
    }

    impl Attrs for Graph {
        fn attrs(&self) -> &HashMap<String, String> {
            &self.attrs
        }

        fn attrs_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.attrs
        }
    }

    pub trait Attrs {
        fn attrs(&self) -> &HashMap<String, String>;

        fn attrs_mut(&mut self) -> &mut HashMap<String, String>;

        fn attr<'a>(&'a self, name: &str) -> Option<&'a str> {
            self.attrs().get(name).map(|v| v.as_str())
        }

        fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self
        where
            Self: Sized,
        {
            let attrs_map = self.attrs_mut();
            for attr in attrs.iter() {
                attrs_map.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }
    }
}
