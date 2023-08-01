use std::collections::HashSet;

use crate::sync::{BlockIdFor, Justification, PeerId};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Importance {
    Auxiliary,
    Required,
    ExplicitlyRequired,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum HeaderImportance {
    Imported,
    Unimported(Importance),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InnerVertex<J: Justification> {
    /// Empty Vertex.
    Empty { required: Importance },
    /// Vertex with added Header.
    Header {
        importance: HeaderImportance,
        parent: BlockIdFor<J>,
    },
    /// Vertex with added Header and Justification.
    Justification {
        imported: bool,
        justification: J,
        parent: BlockIdFor<J>,
    },
}

/// The complete vertex, including metadata about peers that know most about the data it refers to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vertex<I: PeerId, J: Justification> {
    inner: InnerVertex<J>,
    know_most: HashSet<I>,
}

impl<I: PeerId, J: Justification> Vertex<I, J> {
    /// Create a new empty vertex.
    pub fn new() -> Self {
        Vertex {
            inner: InnerVertex::Empty {
                required: Importance::Auxiliary,
            },
            know_most: HashSet::new(),
        }
    }

    /// Whether the vertex is auxiliary.
    pub fn auxiliary(&self) -> bool {
        use Importance::*;
        use InnerVertex::*;
        matches!(
            self.inner,
            Empty {
                required: Auxiliary,
                ..
            } | Header {
                importance: HeaderImportance::Unimported(Auxiliary),
                ..
            }
        )
    }

    /// Whether we want the referenced block in our database.
    pub fn importable(&self) -> bool {
        use Importance::*;
        use InnerVertex::*;
        matches!(
            self.inner,
            Empty {
                required: Required | ExplicitlyRequired
            } | Header {
                importance: HeaderImportance::Unimported(Required | ExplicitlyRequired),
                ..
            } | Justification {
                imported: false,
                ..
            }
        )
    }

    /// Whether the referenced block should be requested.
    /// This ignores blocks requested due to justifications, as these will be requested separately.
    pub fn requestable(&self) -> bool {
        use Importance::*;
        use InnerVertex::*;
        matches!(
            self.inner,
            Empty {
                required: ExplicitlyRequired
            } | Header {
                importance: HeaderImportance::Unimported(ExplicitlyRequired),
                ..
            }
        )
    }

    /// Whether the vertex is imported.
    pub fn imported(&self) -> bool {
        use InnerVertex::*;
        matches!(
            self.inner,
            Header {
                importance: HeaderImportance::Imported,
                ..
            } | Justification { imported: true, .. }
        )
    }

    /// Whether the vertex represents imported justified block.
    pub fn justified_block(&self) -> bool {
        matches!(
            self.inner,
            InnerVertex::Justification { imported: true, .. }
        )
    }

    /// Deconstructs the vertex into a justification if it is ready to be imported,
    /// i.e. the related block has already been imported, otherwise returns it.
    pub fn ready(self) -> Result<J, Self> {
        match self.inner {
            InnerVertex::Justification {
                imported: true,
                justification,
                ..
            } => Ok(justification),
            _ => Err(self),
        }
    }

    /// The parent of the vertex, if known.
    pub fn parent(&self) -> Option<&BlockIdFor<J>> {
        match &self.inner {
            InnerVertex::Empty { .. } => None,
            InnerVertex::Header { parent, .. } => Some(parent),
            InnerVertex::Justification { parent, .. } => Some(parent),
        }
    }

    /// The list of peers which know most about the data this vertex refers to.
    pub fn know_most(&self) -> &HashSet<I> {
        &self.know_most
    }

    /// Set the vertex to be explicitly required, returns whether anything changed, i.e. the vertex
    /// was not explicitly required or imported before.
    pub fn set_explicitly_required(&mut self) -> bool {
        use HeaderImportance::*;
        use Importance::*;
        use InnerVertex::*;
        match &self.inner {
            Empty {
                required: Required | Auxiliary,
            } => {
                self.inner = Empty {
                    required: ExplicitlyRequired,
                };
                true
            }
            Header {
                importance: Unimported(Required | Auxiliary),
                parent,
            } => {
                self.inner = Header {
                    importance: Unimported(ExplicitlyRequired),
                    parent: parent.clone(),
                };
                true
            }
            _ => false,
        }
    }

    /// Set the vertex to be required, returns whether anything changed, i.e. the vertex was not
    /// required or imported before.
    pub fn set_required(&mut self) -> bool {
        use HeaderImportance::*;
        use Importance::*;
        use InnerVertex::*;
        match &self.inner {
            Empty {
                required: Auxiliary,
            } => {
                self.inner = Empty { required: Required };
                true
            }
            Header {
                importance: Unimported(Auxiliary),
                parent,
            } => {
                self.inner = Header {
                    importance: Unimported(Required),
                    parent: parent.clone(),
                };
                true
            }
            _ => false,
        }
    }

    /// Adds a peer that knows most about the block this vertex refers to. Does nothing if we
    /// already have a justification.
    pub fn add_block_holder(&mut self, holder: Option<I>) {
        if let Some(holder) = holder {
            if !matches!(self.inner, InnerVertex::Justification { .. }) {
                self.know_most.insert(holder);
            }
        }
    }

    /// Adds the information the header provides to the vertex.
    pub fn insert_header(&mut self, parent: BlockIdFor<J>, holder: Option<I>) {
        self.add_block_holder(holder);
        if let InnerVertex::Empty { required } = self.inner {
            self.inner = InnerVertex::Header {
                importance: HeaderImportance::Unimported(required),
                parent,
            };
        }
    }

    /// Adds the information the header provides to the vertex and marks it as imported. Returns
    /// whether finalization is now possible.
    pub fn insert_body(&mut self, parent: BlockIdFor<J>) -> bool {
        use InnerVertex::*;
        match &self.inner {
            Empty { .. } | Header { .. } => {
                self.inner = Header {
                    parent,
                    importance: HeaderImportance::Imported,
                };
                false
            }
            Justification {
                imported: false,
                parent,
                justification,
            } => {
                self.inner = Justification {
                    imported: true,
                    parent: parent.clone(),
                    justification: justification.clone(),
                };
                true
            }
            _ => false,
        }
    }

    /// Adds a justification to the vertex.
    pub fn insert_justification(
        &mut self,
        parent: BlockIdFor<J>,
        justification: J,
        holder: Option<I>,
    ) {
        use InnerVertex::*;
        match self.inner {
            Empty { .. }
            | Header {
                importance: HeaderImportance::Unimported(_),
                ..
            } => {
                self.inner = Justification {
                    imported: false,
                    parent,
                    justification,
                };
                self.know_most = holder.into_iter().collect();
            }
            Header {
                importance: HeaderImportance::Imported,
                ..
            } => {
                self.inner = Justification {
                    imported: true,
                    parent,
                    justification,
                };
                self.know_most = holder.into_iter().collect();
            }
            Justification { .. } => {
                if let Some(holder) = holder {
                    self.know_most.insert(holder);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vertex;
    use crate::sync::{
        mock::{MockHeader, MockIdentifier, MockJustification, MockPeerId},
        Header,
    };

    type MockVertex = Vertex<MockPeerId, MockJustification>;

    #[test]
    fn initially_empty() {
        let vertex = MockVertex::new();
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
        assert!(!vertex.imported());
        assert!(vertex.parent().is_none());
        assert!(vertex.know_most().is_empty());
        assert_eq!(vertex.clone().ready(), Err(vertex));
    }

    #[test]
    fn empty_remembers_block_holders() {
        let mut vertex = MockVertex::new();
        let peer_id = rand::random();
        vertex.add_block_holder(Some(peer_id));
        assert!(vertex.know_most().contains(&peer_id));
    }

    #[test]
    fn empty_set_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_required());
        assert!(vertex.importable());
        assert!(!vertex.requestable());
        assert!(!vertex.set_required());
        assert!(vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn empty_set_explicitly_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
        assert!(!vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
    }

    #[test]
    fn empty_set_required_then_explicitly_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_required());
        assert!(vertex.importable());
        assert!(!vertex.requestable());
        assert!(vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
    }

    #[test]
    fn empty_set_explicitly_required_then_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
        assert!(!vertex.set_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
    }

    #[test]
    fn empty_to_header() {
        let mut vertex = MockVertex::new();
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent.clone(), Some(peer_id));
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
        assert!(!vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert!(vertex.know_most().contains(&peer_id));
        assert_eq!(vertex.clone().ready(), Err(vertex));
    }

    #[test]
    fn header_remembers_block_holders() {
        let mut vertex = MockVertex::new();
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent, Some(peer_id));
        let other_peer_id = rand::random();
        vertex.add_block_holder(Some(other_peer_id));
        assert!(vertex.know_most().contains(&peer_id));
        assert!(vertex.know_most().contains(&other_peer_id));
    }

    #[test]
    fn header_set_required() {
        let mut vertex = MockVertex::new();
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent, Some(peer_id));
        assert!(vertex.set_required());
        assert!(vertex.importable());
        assert!(!vertex.set_required());
        assert!(vertex.importable());
    }

    #[test]
    fn header_set_explicitly_required() {
        let mut vertex = MockVertex::new();
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent, Some(peer_id));
        assert!(vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
        assert!(!vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
    }

    #[test]
    fn header_still_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_required());
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent, Some(peer_id));
        assert!(vertex.importable());
        assert!(!vertex.set_required());
        assert!(vertex.importable());
    }

    #[test]
    fn header_still_explicitly_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_explicitly_required());
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent, Some(peer_id));
        assert!(vertex.importable());
        assert!(vertex.requestable());
        assert!(!vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(vertex.requestable());
    }

    #[test]
    fn empty_to_body() {
        let mut vertex = MockVertex::new();
        let parent = MockIdentifier::new_random(43);
        assert!(!vertex.insert_body(parent.clone()));
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
        assert!(vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert_eq!(vertex.clone().ready(), Err(vertex));
    }

    #[test]
    fn header_to_body() {
        let mut vertex = MockVertex::new();
        let peer_id = rand::random();
        let parent = MockIdentifier::new_random(43);
        vertex.insert_header(parent.clone(), Some(peer_id));
        assert!(!vertex.insert_body(parent.clone()));
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
        assert!(vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert_eq!(vertex.clone().ready(), Err(vertex));
    }

    #[test]
    fn body_set_required() {
        let mut vertex = MockVertex::new();
        let parent = MockIdentifier::new_random(43);
        assert!(!vertex.insert_body(parent));
        assert!(!vertex.set_required());
        assert!(!vertex.importable());
        assert!(!vertex.set_explicitly_required());
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn body_no_longer_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_required());
        let parent = MockIdentifier::new_random(43);
        assert!(!vertex.insert_body(parent));
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn body_no_longer_explicitly_required() {
        let mut vertex = MockVertex::new();
        assert!(vertex.set_explicitly_required());
        let parent = MockIdentifier::new_random(43);
        assert!(!vertex.insert_body(parent));
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn empty_to_justification() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        let peer_id = rand::random();
        vertex.insert_justification(parent.clone(), justification, Some(peer_id));
        assert!(vertex.importable());
        assert!(!vertex.requestable());
        assert!(!vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert!(vertex.know_most().contains(&peer_id));
        assert_eq!(vertex.clone().ready(), Err(vertex));
    }

    #[test]
    fn header_to_justification() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        let peer_id = rand::random();
        vertex.insert_header(parent.clone(), Some(peer_id));
        vertex.insert_justification(parent.clone(), justification, None);
        assert!(vertex.importable());
        assert!(!vertex.requestable());
        assert!(!vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert!(vertex.know_most().is_empty());
        assert_eq!(vertex.clone().ready(), Err(vertex));
    }

    #[test]
    fn body_to_justification() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        assert!(!vertex.insert_body(parent.clone()));
        vertex.insert_justification(parent.clone(), justification.clone(), None);
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
        assert!(vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert_eq!(vertex.ready(), Ok(justification));
    }

    #[test]
    fn justification_set_required() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        let peer_id = rand::random();
        vertex.insert_justification(parent, justification, Some(peer_id));
        assert!(!vertex.set_required());
        assert!(vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn justification_set_explicitly_required() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        let peer_id = rand::random();
        vertex.insert_justification(parent, justification, Some(peer_id));
        assert!(!vertex.set_explicitly_required());
        assert!(vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn justification_still_required() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        let peer_id = rand::random();
        assert!(vertex.set_required());
        vertex.insert_justification(parent, justification, Some(peer_id));
        assert!(vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn justification_no_longer_explicitly_required() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        let peer_id = rand::random();
        assert!(vertex.set_explicitly_required());
        vertex.insert_justification(parent, justification, Some(peer_id));
        assert!(vertex.importable());
        assert!(!vertex.requestable());
    }

    #[test]
    fn my_body_is_ready() {
        let mut vertex = MockVertex::new();
        let parent_header = MockHeader::random_parentless(0);
        let header = parent_header.random_child();
        let parent = header.parent_id().expect("born of a parent");
        let justification = MockJustification::for_header(header);
        vertex.insert_justification(parent.clone(), justification.clone(), None);
        assert!(vertex.insert_body(parent.clone()));
        assert!(!vertex.importable());
        assert!(!vertex.requestable());
        assert!(vertex.imported());
        assert_eq!(vertex.parent(), Some(&parent));
        assert_eq!(vertex.ready(), Ok(justification));
    }
}
