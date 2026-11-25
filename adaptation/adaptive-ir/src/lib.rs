//! The Adaptive IR (AIR) data model.
//!
//! AIR is the representation in which software composition and change are
//! expressed, verified and executed (`docs/05-adaptive-ir.md`). Stage 0 defines
//! only the constructs the specification names. The grammar is settled by an RFC
//! before a parser is written, so this crate holds the AST types and no parsing.

/// A declared Capability binding inside a workspace or plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapabilityDecl {
    /// The Capability identifier, for example `csv.reader`.
    pub capability: String,
    /// The alias used to refer to it in connections, for example `reader`.
    pub alias: String,
    /// The declared source, if any, for example an input file.
    pub source: Option<String>,
}

impl CapabilityDecl {
    /// Creates a declaration with no source.
    #[must_use]
    pub fn new(capability: impl Into<String>, alias: impl Into<String>) -> Self {
        Self {
            capability: capability.into(),
            alias: alias.into(),
            source: None,
        }
    }
}

/// A connection from one Capability's output to another's input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    /// The producing reference, for example `reader.rows`.
    pub from: String,
    /// The consuming reference, for example `viewer.input`.
    pub to: String,
}

impl Connection {
    /// Creates a connection between two references.
    #[must_use]
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

/// A declared workspace: a task environment with a goal.
///
/// This is the `workspace` construct of `docs/05-adaptive-ir.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceDecl {
    /// The workspace name.
    pub name: String,
    /// The goal of the task.
    pub goal: String,
    /// The Capabilities the workspace binds.
    pub capabilities: Vec<CapabilityDecl>,
    /// The connections between Capabilities.
    pub connections: Vec<Connection>,
}

impl WorkspaceDecl {
    /// Creates an empty workspace declaration for `name` and `goal`.
    #[must_use]
    pub fn new(name: impl Into<String>, goal: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            goal: goal.into(),
            capabilities: Vec::new(),
            connections: Vec::new(),
        }
    }
}

/// A change to an existing workspace, applied as a transaction.
///
/// This is the `plan extend` construct of `docs/05-adaptive-ir.md`, applied per
/// `docs/10-adaptation-transactions.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    /// The name of the workspace the plan extends.
    pub base: String,
    /// The Capabilities the plan adds.
    pub add: Vec<CapabilityDecl>,
    /// The connections the plan adds.
    pub connect: Vec<Connection>,
}

impl Plan {
    /// Creates an empty plan that extends `base`.
    #[must_use]
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            add: Vec::new(),
            connect: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CapabilityDecl, Connection, Plan, WorkspaceDecl};

    #[test]
    fn a_workspace_declaration_builds_up() {
        let mut decl = WorkspaceDecl::new("sales-analysis", "analyse sales.csv");
        decl.capabilities
            .push(CapabilityDecl::new("csv.reader", "reader"));
        decl.connections
            .push(Connection::new("reader.rows", "viewer.input"));

        assert_eq!(decl.name, "sales-analysis");
        assert_eq!(decl.capabilities.len(), 1);
        assert_eq!(decl.connections.len(), 1);
        assert_eq!(decl.capabilities[0].source, None);
    }

    #[test]
    fn a_plan_extends_a_base() {
        let mut plan = Plan::new("sales-analysis");
        plan.add
            .push(CapabilityDecl::new("stats.correlation", "corr"));

        assert_eq!(plan.base, "sales-analysis");
        assert_eq!(plan.add.len(), 1);
        assert!(plan.connect.is_empty());
    }
}
